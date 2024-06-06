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
                return Err(ft_sdk::single_error("email", "email is required").into());
            }

            Ok((
                Subscriber {
                    name,
                    email: email.unwrap(),
                    phone,
                },
                None,
            ))
        }
    }
}

#[ft_sdk::data]
fn subscribe(
    ft_sdk::Query(name): ft_sdk::Query<"name", Option<String>>,
    ft_sdk::Query(phone): ft_sdk::Query<"phone", Option<String>>,
    ft_sdk::Query(email): ft_sdk::Query<"email", Option<String>>,
    ft_sdk::Query(source): ft_sdk::Query<"source", Option<String>>,
    ft_sdk::Query(topic): ft_sdk::Query<"topic", Option<String>>,
    sid: ft_sdk::Cookie<{ ft_sdk::auth::SESSION_KEY }>,
    mut conn: ft_sdk::Connection,
) -> ft_sdk::data::Result {
    let (subscriber, id) = validate(&mut conn, name, phone, email, sid)?;
    let id = get_or_create_user_id(&mut conn, id, &subscriber)?;

    // ft_sdk::auth::provider::update_user() will overwrite "subscription" data, we want to
    // preserve anything that was imported from other sources so:
    //
    // get `data` of user as string
    // parse it as serde_json::Value::Object
    // add "subscription" key with value:
    // {
    //     subscription: {
    //         subscribed: true,
    //         topics: [] // append here if not exists already
    //         ... any other existing key if any
    //     }
    // }

    todo!()
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
