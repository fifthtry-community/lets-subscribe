#[ft_sdk::processor]
pub fn confirm_subscription(
    mut conn: ft_sdk::Connection,
    ft_sdk::Query(code): ft_sdk::Query<"code">,
    ft_sdk::Query(email): ft_sdk::Query<"email">,
    ft_sdk::Query(next): ft_sdk::Query<"next", Option<String>>,
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

    let data = {
        let mut data = data;

        data = mark_user_verified(data);

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

    let next = next.unwrap_or_else(|| "/".to_string());
    ft_sdk::processor::temporary_redirect(next)
}

pub fn mark_user_verified(mut user_data: serde_json::Value) -> serde_json::Value {
    match user_data
        .as_object_mut()
        .expect("data is always a json object")
        .get_mut("subscription")
    {
        Some(sub) => {
            sub.as_object_mut()
                .expect("subscription is always a json object")
                .insert("confirmed".to_string(), serde_json::Value::Bool(true));

            sub.as_object_mut()
                .expect("subscription is always a json object")
                .remove("confirmation_key");
        }
        None => unreachable!("if we found this user from the ?code then the object must exist"),
    }

    user_data
}
