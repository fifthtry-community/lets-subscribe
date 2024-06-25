/// ### Set Tracker: `/set-tracker/?t=<encrypted/hashed uid>`
/// create a tracker cookie with the user_id decrypted from `t`
///
// if there's a ?t then set session to tracking user id
// if there's no ?t and the sid.0 is None then create a new session
#[ft_sdk::processor]
fn set_tracker(
    mut conn: ft_sdk::Connection,
    // t is the encrypted user_id (i64)
    ft_sdk::Query(t): ft_sdk::Query<"t", Option<String>>,
    sid: ft_sdk::Cookie<{ ft_sdk::auth::SESSION_KEY }>,
    host: ft_sdk::Host,
) -> ft_sdk::processor::Result {
    // set the tracking cookie
    let session_id = match t {
        Some(t) => Some(create_session_using_t(t, &mut conn)?),
        None => {
            if sid.0.is_none() {
                // create empty session
                Some(ft_sdk::SessionID::create(&mut conn, None, None)?)
            } else {
                // session is already set, do nothing
                None
            }
        }
    };

    let mut resp = ft_sdk::processor::json(serde_json::json!({ "success": true }))?;

    if let Some(session_id) = session_id {
        resp = resp.with_cookie(subscription::session_cookie(&session_id.0, host.clone())?);
    }

    resp = resp.with_cookie(subscription::tracker_cookie(
        &ft_sdk::utils::uuid_v8(),
        host,
    )?);

    Ok(resp)
}

/// create tracker entry using the user id decrypted and parsed from `t`
/// `t` is the encrypted user_id (i64)
fn create_session_using_t(
    t: String,
    conn: &mut ft_sdk::Connection,
) -> Result<ft_sdk::SessionID, ft_sdk::Error> {
    let user_id: ft_sdk::PlainText =
        ft_sdk::EncryptedString::from_already_encrypted_string(t).try_into()?;

    let user_id = user_id.to_string().parse::<i64>()?;
    let session_data = serde_json::json!({ "subscription_uid": user_id });

    ft_sdk::SessionID::create(conn, None, Some(session_data))
}
