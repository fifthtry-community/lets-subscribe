struct Subscriber {
    name: Option<String>,
    email: String,
    phone: Option<String>,
}

impl Subscriber {
    fn to_provider_data(&self) -> ft_sdk::auth::ProviderData {
        ft_sdk::auth::ProviderData {
            identity: self.email.clone(),
            username: self.name.clone(),
            name: self.name.clone(),
            emails: vec![self.email.clone()],
            verified_emails: vec![],
            profile_picture: None,
            custom: serde_json::json!({}),
        }
    }
}

/// construct [Subscriber] from request data and session
/// authenticated user data takes precedence over request data
fn validate(
    conn: &mut ft_sdk::Connection,
    name: Option<String>,
    phone: Option<String>,
    email: Option<String>,
    sid: ft_sdk::Cookie<{ ft_sdk::auth::SESSION_KEY }>,
) -> Result<(Subscriber, Option<ft_sdk::auth::UserId>), ft_sdk::Error> {
    match ft_sdk::auth::ud(sid, conn)? {
        Some(ud) => Ok((
            Subscriber {
                name: Some(ud.name),
                email: ud.email,
                phone,
            },
            Some(ft_sdk::auth::UserId(ud.id)),
        )),
        None => {
            if email.is_none() {
                return Err(ft_sdk::single_error("email", "Email is required.").into());
            }
            let email = email.unwrap();

            if !validator::ValidateEmail::validate_email(&email) {
                return Err(ft_sdk::single_error("email", "Invalid email.").into());
            }

            Ok((
                Subscriber {
                    name,
                    email,
                    phone,
                },
                None,
            ))
        }
    }
}

#[ft_sdk::form]
fn subscribe(
    ft_sdk::Query(name): ft_sdk::Query<"name", Option<String>>,
    ft_sdk::Query(phone): ft_sdk::Query<"phone", Option<String>>,
    ft_sdk::Query(email): ft_sdk::Query<"email", Option<String>>,
    ft_sdk::Query(source): ft_sdk::Query<"source", Option<String>>,
    ft_sdk::Query(topic): ft_sdk::Query<"topic", Option<String>>,
    ft_sdk::Query(next): ft_sdk::Query<"next", Option<String>>,
    sid: ft_sdk::Cookie<{ ft_sdk::auth::SESSION_KEY }>,
    mut conn: ft_sdk::Connection,
) -> ft_sdk::form::Result {
    use diesel::prelude::*;

    let (subscriber, id) = validate(&mut conn, name, phone, email, sid)?;
    let user_id = get_or_create_user_id(&mut conn, id, &subscriber)?;

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

    let (subscribed, data) = add_subscription_info(data, topic, source, &subscriber);
    let data = serde_json::to_string(&data)?;

    diesel::update(
        ft_sdk::auth::fastn_user::table.filter(ft_sdk::auth::fastn_user::id.eq(user_id.0)),
    )
    .set((
        ft_sdk::auth::fastn_user::data.eq(data),
        ft_sdk::auth::fastn_user::updated_at.eq(ft_sdk::env::now()),
    ))
    .execute(&mut conn)?;

    if subscribed {
        // TODO: send double opt-in email
        // add another boolean to the sub data, `double_optin` and set it to true upon verification
        // (clicking on the link in the email)
    }

    ft_sdk::form::redirect(next.unwrap_or_else(|| "/thank-you/".to_string()))
}

/// return `id` if it is Some
/// otherwise, get user data by email and return its id
/// if user data is not found, create user and return its id
fn get_or_create_user_id(
    conn: &mut ft_sdk::Connection,
    id: Option<ft_sdk::auth::UserId>,
    subscriber: &Subscriber,
) -> Result<ft_sdk::auth::UserId, ft_sdk::Error> {
    let user_id = if let Some(id) = id {
        id
    } else {
        match ft_sdk::auth::provider::user_data_by_email(
            conn,
            crate::EMAIL_PROVIDER_ID,
            &subscriber.email,
        )
        .map(|(id, _)| id)
        {
            Ok(v) => v,
            Err(e) => match e {
                ft_sdk::auth::UserDataError::NoDataFound => ft_sdk::auth::provider::create_user(
                    conn,
                    crate::EMAIL_PROVIDER_ID,
                    subscriber.to_provider_data(),
                )?,
                e => return Err(e.into()),
            },
        }
    };

    Ok(user_id)
}

/// add topic and source to the fastn_user.data's "subscription" provider
/// if "subscription" provider does not exist, create it
/// if "subscription" provider exists, append topic and source to their respective arrays (no
/// duplicates are added)
/// set "subscribed" to true
///
/// return (subscribed, updated_data). The subscribed is a boolean which is true if a new sub was
/// done. false indicates that the user was already subscribed, to this topic or all topics
fn add_subscription_info(
    mut data: serde_json::Value,
    topic: Option<String>,
    source: Option<String>,
    subscriber: &Subscriber,
) -> (bool, serde_json::Value) {
    let mut subscribed = false;

    if let Some(topic) = topic {
        match data
            .as_object_mut()
            .expect("data is always a json object")
            .get_mut("subscription")
        {
            Some(sub) => {
                if let Some(topics) = sub.get_mut("topics") {
                    // add if this topic does not already exist
                    if topics
                        .as_array_mut()
                        .expect("topics is always a json array")
                        .iter()
                        .find(|t| t.as_str().expect("topic is a str") == topic)
                        .is_none()
                    {
                        topics
                            .as_array_mut()
                            .expect("topics is always a json array")
                            .push(serde_json::Value::String(topic));

                        subscribed = true;
                    }
                } else {
                    sub.as_object_mut()
                        .expect("subscription is always a json object")
                        .insert(
                            "topics".to_string(),
                            serde_json::Value::Array(vec![serde_json::Value::String(topic)]),
                        );

                    subscribed = true;
                }
            }
            None => {
                data.as_object_mut()
                    .expect("data is always a json object")
                    .insert(
                        "subscription".to_string(),
                        serde_json::json!({
                            "topics": [topic],
                            "sources": [],
                        }),
                    );

                subscribed = true;
            }
        }
    }

    if let Some(source) = source {
        match data
            .as_object_mut()
            .expect("data is always a json object")
            .get_mut("subscription")
        {
            Some(sub) => {
                if let Some(sources) = sub.get_mut("sources") {
                    if sources
                        .as_array_mut()
                        .expect("topics is always a json array")
                        .iter()
                        .find(|s| s.as_str().expect("topic is a str") == source)
                        .is_none()
                    {
                        sources
                            .as_array_mut()
                            .expect("sources is always a json array")
                            .push(serde_json::Value::String(source));
                    }
                } else {
                    sub.as_object_mut()
                        .expect("subscription is always a json object")
                        .insert(
                            "topics".to_string(),
                            serde_json::Value::Array(vec![serde_json::Value::String(source)]),
                        );
                }
            }
            None => {
                data.as_object_mut()
                    .expect("data is always a json object")
                    .insert(
                        "subscription".to_string(),
                        serde_json::json!({
                            "sources": [source],
                            "topics": [],
                        }),
                    );
            }
        }
    }

    match data
        .as_object_mut()
        .expect("data is always a json object")
        .get_mut("subscription")
    {
        Some(sub) => {
            let old_value = sub
                .as_object_mut()
                .expect("subscription is always a json object")
                .insert("subscribed".to_string(), serde_json::Value::Bool(true));

            if old_value.is_none() {
                subscribed = true;
            }

            if let Some(name) = &subscriber.name {
                sub.as_object_mut()
                    .expect("subscription is always a json object")
                    .insert(
                        "name".to_string(),
                        serde_json::Value::String(name.to_string()),
                    );
            }

            if let Some(phone) = &subscriber.phone {
                sub.as_object_mut()
                    .expect("subscription is always a json object")
                    .insert(
                        "phone".to_string(),
                        serde_json::Value::String(phone.to_string()),
                    );
            }

            sub.as_object_mut()
                .expect("subscription is always a json object")
                .insert(
                    "email".to_string(),
                    serde_json::Value::String(subscriber.email.clone()),
                );
        }
        None => {
            data.as_object_mut()
                .expect("data is always a json object")
                .insert(
                    "subscription".to_string(),
                    serde_json::json!({
                        "subscribed": true,
                        "topics": [],
                        "sources": [],
                        "name": subscriber.name,
                        "phone": subscriber.phone,
                        "email": subscriber.email,
                    }),
                );

            subscribed = true;
        }
    }

    (subscribed, data)
}
