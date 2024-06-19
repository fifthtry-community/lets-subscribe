#[ft_sdk::form]
fn subscribe(
    ft_sdk::Query(name): ft_sdk::Query<"name", Option<String>>,
    ft_sdk::Query(phone): ft_sdk::Query<"phone", Option<String>>,
    ft_sdk::Query(email): ft_sdk::Query<"email", Option<String>>,
    ft_sdk::Query(source): ft_sdk::Query<"source", Option<String>>,
    ft_sdk::Query(topic): ft_sdk::Query<"topic", Option<String>>,
    ft_sdk::Query(next): ft_sdk::Query<"next", Option<String>>,
    // on_confirm is the route to redirect to after the user confirms their subscription
    ft_sdk::Query(on_confirm): ft_sdk::Query<"on_confirm", Option<String>>,
    sid: ft_sdk::Cookie<{ ft_sdk::auth::SESSION_KEY }>,
    host: ft_sdk::Host,
    mountpoint: ft_sdk::Mountpoint,
    mut conn: ft_sdk::Connection,
) -> ft_sdk::form::Result {
    use diesel::prelude::*;

    let (subscriber, id, is_authenticated) = validate(&mut conn, name, phone, email, sid)?;
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

    let (has_subscribed, mut data) =
        add_subscription_info(data, topic.clone(), source, &subscriber);

    let name = subscriber.name.unwrap_or_else(|| "".to_string());
    if has_subscribed && !is_authenticated {
        let key = ft_sdk::Rng::generate_key(64);
        data = add_confirmation_key_in_user(data, &key);

        let on_confirm = on_confirm.clone().unwrap_or_else(|| "/".to_string());
        let conf_link = confirmation_link(&key, &subscriber.email, &on_confirm, &host, &mountpoint);
        send_double_opt_in_email(&mut conn, (&name, &subscriber.email), &conf_link, topic)?;
    } else {
        data = subscription::mark_user_verified(data);
        subscription::send_welcome_email(&mut conn, (&name, &subscriber.email))?;
    }

    let data = serde_json::to_string(&data)?;

    diesel::update(
        ft_sdk::auth::fastn_user::table.filter(ft_sdk::auth::fastn_user::id.eq(user_id.0)),
    )
    .set((
        ft_sdk::auth::fastn_user::data.eq(&data),
        ft_sdk::auth::fastn_user::updated_at.eq(ft_sdk::env::now()),
    ))
    .execute(&mut conn)?;

    let next = if is_authenticated { on_confirm } else { next };

    ft_sdk::form::redirect(next.unwrap_or_else(|| "/thank-you/".to_string()))
}

struct Subscriber {
    name: Option<String>,
    email: String,
    phone: Option<String>,
}

/// construct [Subscriber] from request data and session
/// if email is None, try to get it from logged in user
///
/// return (Subscriber, UserId, is_authenticated)
fn validate(
    conn: &mut ft_sdk::Connection,
    name: Option<String>,
    phone: Option<String>,
    email: Option<String>,
    sid: ft_sdk::Cookie<{ ft_sdk::auth::SESSION_KEY }>,
) -> Result<(Subscriber, Option<ft_sdk::auth::UserId>, bool), ft_sdk::Error> {
    match email {
        Some(email) => {
            if !validator::ValidateEmail::validate_email(&email) {
                return Err(ft_sdk::single_error("email", "Invalid email.").into());
            }

            Ok((Subscriber { name, email, phone }, None, false))
        }
        None => match ft_sdk::auth::ud(sid, conn)? {
            Some(ud) => Ok((
                Subscriber {
                    name: Some(ud.name),
                    email: ud.email,
                    phone,
                },
                Some(ft_sdk::auth::UserId(ud.id)),
                true,
            )),
            None => {
                return Err(ft_sdk::single_error("email", "Email is required.").into());
            }
        },
    }
}

fn add_confirmation_key_in_user(mut user_data: serde_json::Value, key: &str) -> serde_json::Value {
    match user_data
        .as_object_mut()
        .expect("data is always a json object")
        .get_mut("subscription")
    {
        Some(sub) => {
            sub.as_object_mut()
                .expect("subscription is always a json object")
                .insert(
                    "confirmation_key".to_string(),
                    serde_json::Value::String(key.to_string()),
                );
        }
        None => {
            user_data
                .as_object_mut()
                .expect("data is always a json object")
                .insert(
                    "subscription".to_string(),
                    serde_json::json!({
                        "confirmation_key": key,
                    }),
                );
        }
    }

    user_data
}

fn confirmation_link(
    key: &str,
    email: &str,
    next: &str,
    ft_sdk::Host(host): &ft_sdk::Host,
    ft_sdk::Mountpoint(mountpoint): &ft_sdk::Mountpoint,
) -> String {
    format!(
        "https://{host}{mountpoint}{confirm_sub_route}?code={key}&email={email}&next={next}",
        confirm_sub_route = "/confirm-subscription/",
        mountpoint = mountpoint.trim_end_matches('/'),
    )
}

fn send_double_opt_in_email(
    conn: &mut ft_sdk::Connection,
    to: (&str, &str),
    conf_link: &str,
    topic: Option<String>,
) -> Result<(), ft_sdk::Error> {
    let (from_name, from_email) = email_from_address_from_env();

    let name_or_email = if to.0.is_empty() { to.1 } else { to.0 };

    let to_topic = topic
        .map(|topic| format!("to the {}", topic))
        .unwrap_or_default();

    let body_html = subscription::confirm_email_templ::HTML_BODY
        .replace("{name}", name_or_email)
        .replace("{confirmation_link}", conf_link)
        .replace("{topic}", &to_topic);

    let body_txt = subscription::confirm_email_templ::TEXT_BODY
        .replace("{name}", name_or_email)
        .replace("{confirmation_link}", conf_link)
        .replace("{topic}", &to_topic);

    Ok(ft_sdk::send_email(
        conn,
        (&from_name, &from_email),
        vec![to],
        // TODO: this should be configurable
        "Confirm your subscription",
        &body_html,
        &body_txt,
        None,
        None,
        None,
        "subscription.confirm_subscription_request",
    )?)
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
                ft_sdk::auth::UserDataError::NoDataFound => create_user(conn, subscriber)?,
                e => return Err(e.into()),
            },
        }
    };

    Ok(user_id)
}

fn create_user(
    conn: &mut ft_sdk::SqliteConnection,
    subscriber: &Subscriber,
) -> Result<ft_sdk::UserId, ft_sdk::Error> {
    use diesel::prelude::*;
    use ft_sdk::auth::fastn_user;

    let data = serde_json::to_string(&serde_json::json!({
        "email": {
            "emails": [subscriber.email],
            "identity": subscriber.email,
        },
        "subscription": {
            "identity": subscriber.email,
            "name": subscriber.name,
        },
    }))?;

    let user_id: i64 = diesel::insert_into(fastn_user::table)
        .values((
            fastn_user::name.eq(&subscriber.name),
            fastn_user::data.eq(&data),
            fastn_user::created_at.eq(ft_sdk::env::now()),
            fastn_user::updated_at.eq(ft_sdk::env::now()),
        ))
        .returning(fastn_user::id)
        .get_result(conn)?;

    Ok(ft_sdk::UserId(user_id))
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
                            "sources".to_string(),
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
            let sub = sub
                .as_object_mut()
                .expect("subscription is always a json object");

            let old_value = sub.insert("subscribed".to_string(), serde_json::Value::Bool(true));

            if old_value.is_none() {
                subscribed = true;
            }

            if let Some(name) = &subscriber.name {
                sub.insert(
                    "name".to_string(),
                    serde_json::Value::String(name.to_string()),
                );
            }

            if let Some(phone) = &subscriber.phone {
                sub.insert(
                    "phone".to_string(),
                    serde_json::Value::String(phone.to_string()),
                );
            }

            sub.insert(
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
                        "double_optin": false,
                    }),
                );

            subscribed = true;
        }
    }

    (subscribed, data)
}

pub(crate) fn email_from_address_from_env() -> (String, String) {
    let email = ft_sdk::env::var("FASTN_SMTP_SENDER_EMAIL".to_string())
        .unwrap_or_else(|| "support@fifthtry.com".to_string());
    let name = ft_sdk::env::var("FASTN_SMTP_SENDER_NAME".to_string())
        .unwrap_or_else(|| "FifthTry Team".to_string());

    (name, email)
}
