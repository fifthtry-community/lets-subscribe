#[ft_sdk::processor]
fn confirm_subscription(
    mut conn: ft_sdk::Connection,
    ft_sdk::Query(code): ft_sdk::Query<"code">,
    ft_sdk::Query(email): ft_sdk::Query<"email">,
    ft_sdk::Query(next): ft_sdk::Query<"next", Option<String>>,
    ft_sdk::Config(config): ft_sdk::Config<crate::Config>,
) -> ft_sdk::processor::Result {
    use diesel::prelude::*;

    if !validator::ValidateEmail::validate_email(&email) {
        return Err(ft_sdk::single_error("email", "Invalid email format.").into());
    }

    #[derive(diesel::QueryableByName)]
    #[diesel(table_name = ft_sdk::auth::fastn_user)]
    struct UserData {
        id: i64,
        data: String,
    }

    let (user_id, data): (i64, serde_json::Value) = match diesel::sql_query(
        r#"
        SELECT id, data FROM fastn_user
        WHERE json_extract(data, '$.subscription.confirmation_key') = $1
        AND EXISTS (
            SELECT 1 FROM json_each(json_extract(data, '$.email.emails'))
            WHERE value = $2
        )
        LIMIT 1;
        "#,
    )
    .bind::<diesel::sql_types::Text, _>(&code)
    .bind::<diesel::sql_types::Text, _>(&email)
    .get_result::<UserData>(&mut conn)
    {
        Ok(d) => (d.id, serde_json::from_str(&d.data)?),
        Err(diesel::result::Error::NotFound) => {
            return Err(ft_sdk::single_error("code", "Invalid code or email").into());
        }
        Err(e) => return Err(e.into()),
    };

    let name = get_name(&data);

    let data = {
        let mut data = data;

        data = mark_subscription_verified(data);

        serde_json::to_string(&data)?
    };

    diesel::update(
        ft_sdk::auth::fastn_user::table.filter(ft_sdk::auth::fastn_user::id.eq(user_id)),
    )
    .set((
        ft_sdk::auth::fastn_user::data.eq(&data),
        ft_sdk::auth::fastn_user::updated_at.eq(ft_sdk::env::now()),
    ))
    .execute(&mut conn)?;

    // TODO: get associated topic from db
    let topic = "newsletter".to_string();
    send_welcome_email(&name, &email, &topic, &config)?;

    let next = next.unwrap_or_else(|| "/".to_string());
    ft_sdk::processor::temporary_redirect(next)
}

fn get_name(user_data: &serde_json::Value) -> String {
    user_data
        .as_object()
        .and_then(|v| v.get("subscription"))
        .and_then(|v| v.as_object())
        .and_then(|v| v.get("name"))
        .and_then(|v| v.as_str())
        .map(|v| v.to_string())
        .unwrap_or_default()
}

pub(crate) fn mark_subscription_verified(mut user_data: serde_json::Value) -> serde_json::Value {
    match user_data
        .as_object_mut()
        .expect("data is always a json object")
        .get_mut("subscription")
    {
        Some(sub) => {
            let sub = sub
                .as_object_mut()
                .expect("subscription is always a json object");

            sub.insert("confirmed".to_string(), serde_json::Value::Bool(true));

            sub.remove("confirmation_key");
        }
        None => unreachable!("if we found this user from the ?code then the object must exist"),
    }

    user_data
}

pub(crate) fn send_welcome_email(
    name: &str,
    email: &str,
    topic: &str,
    config: &crate::Config,
) -> Result<(), ft_sdk::Error> {
    let from = config.from_email();
    let name_or_email = if name.is_empty() { email } else { name };

    if let Err(e) = ft_sdk::email::send(&ft_sdk::Email {
        from,
        to: smallvec::smallvec![(name.to_string(), email.to_string()).into()],
        reply_to: Some(smallvec::smallvec![config.reply_to()]),
        cc: smallvec::smallvec![],
        bcc: smallvec::smallvec![],
        mkind: "subscription-welcome-email".to_string(),
        content: ft_sdk::EmailContent::FromMKind {
            context: Some(
                serde_json::json!({
                    "name": name_or_email,
                    "topic": topic,
                })
                .as_object()
                .unwrap()
                .to_owned(),
            ),
        },
    }) {
        ft_sdk::println!("auth.wasm: failed to queue email: {:?}", e);
        return Err(e.into());
    }

    ft_sdk::println!("Email added to the queue");

    Ok(())
}
