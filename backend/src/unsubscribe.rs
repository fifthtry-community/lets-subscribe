#[ft_sdk::form]
fn unsubscribe(
    ft_sdk::Query(email): ft_sdk::Query<"email", Option<String>>,
    ft_sdk::Query(topic): ft_sdk::Query<"topic", Option<String>>,
    ft_sdk::Query(next): ft_sdk::Query<"next", Option<String>>,
    sid: ft_sdk::Cookie<{ ft_sdk::auth::SESSION_KEY }>,
    mut conn: ft_sdk::Connection,
) -> ft_sdk::form::Result {
    use diesel::prelude::*;

    let user_id = get_user(&mut conn, email, sid)?;

    #[derive(diesel::QueryableByName)]
    #[diesel(table_name = ft_sdk::auth::fastn_user)]
    struct UserData {
        data: String,
    }

    let data: serde_json::Value = match diesel::sql_query(
        r#"
        SELECT data from fastn_user where id = $1 LIMIT 1;
        "#,
    )
    .bind::<diesel::sql_types::Integer, _>(user_id.0)
    .get_result::<UserData>(&mut conn)
    {
        Ok(d) => serde_json::from_str(&d.data)?,
        Err(e) => return Err(e.into()),
    };

    let data = remove_subscription_info(data, topic);
    let data = serde_json::to_string(&data)?;

    diesel::update(
        ft_sdk::auth::fastn_user::table.filter(ft_sdk::auth::fastn_user::id.eq(user_id.0)),
    )
    .set((
        ft_sdk::auth::fastn_user::data.eq(data),
        ft_sdk::auth::fastn_user::updated_at.eq(ft_sdk::env::now()),
    ))
    .execute(&mut conn)?;

    ft_sdk::form::redirect(next.unwrap_or_else(|| "/goodbye/".to_string()))
}

/// get authenticated user id if it exists
/// if it does not exist, get user data by email
fn get_user(
    conn: &mut ft_sdk::Connection,
    email: Option<String>,
    sid: ft_sdk::Cookie<{ ft_sdk::auth::SESSION_KEY }>,
) -> Result<ft_sdk::auth::UserId, ft_sdk::Error> {
    match ft_sdk::auth::ud(sid, conn)? {
        Some(ud) => Ok(ft_sdk::auth::UserId(ud.id)),
        None => {
            if email.is_none() {
                return Err(ft_sdk::single_error("email", "email is required").into());
            }

            if !validator::ValidateEmail::validate_email(&email) {
                return Err(ft_sdk::single_error("email", "Invalid email.").into());
            }

            let email = email.unwrap();

            let (user_id, _) =
                ft_sdk::auth::provider::user_data_by_email(conn, crate::EMAIL_PROVIDER_ID, &email)?;

            Ok(user_id)
        }
    }
}

/// mark the subscription as unsubscribed if `topic` is None
/// otherwise, remove the topic from the subscription
fn remove_subscription_info(
    mut data: serde_json::Value,
    topic: Option<String>,
) -> serde_json::Value {
    if let Some(topic) = topic {
        // unsubscribe from this topic if it exists
        match data
            .as_object_mut()
            .expect("data is always a json object")
            .get_mut("subscription")
        {
            Some(sub) => {
                if let Some(topics) = sub.get_mut("topics") {
                    let new_topics: Vec<serde_json::Value> = topics
                        .as_array()
                        .expect("topics is always a json array")
                        .iter()
                        .filter_map(|t| {
                            let tstr = t.as_str().expect("topic is a str");

                            if tstr != topic {
                                Some(t.to_owned())
                            } else {
                                None
                            }
                        })
                        .collect();

                    sub.as_object_mut()
                        .expect("subscription is always a json object")
                        .insert("topics".to_string(), serde_json::Value::Array(new_topics));
                }
            }
            None => {
                data.as_object_mut()
                    .expect("data is always a json object")
                    .insert(
                        "subscription".to_string(),
                        serde_json::json!({
                            "topics": [],
                            "sources": [],
                        }),
                    );
            }
        }
    } else {
        // mark subscribed: false i.e. unsubscribe to all topics
        match data
            .as_object_mut()
            .expect("data is always a json object")
            .get_mut("subscription")
        {
            Some(sub) => {
                sub.as_object_mut()
                    .expect("subscription is always a json object")
                    .insert("subscribed".to_string(), serde_json::Value::Bool(false));
            }
            None => {
                data.as_object_mut()
                    .expect("data is always a json object")
                    .insert(
                        "subscription".to_string(),
                        serde_json::json!({
                            "topics": [],
                            "sources": [],
                            "subscribed": false,
                        }),
                    );
            }
        }
    }

    data
}
