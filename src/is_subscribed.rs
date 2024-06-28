/// find if the current user is subscribed to the ?topic
///
/// if topic is None, then check if the user is subscribed to any topic. This is indicated by
/// `subscribed: true` with `topics: []` in the user's data.
///
/// the user is fetched from the session cookie. If the user is not logged in, the tracker cookie
/// is used in its place.
#[ft_sdk::data]
fn is_subscribed(
    ft_sdk::Query(topic): ft_sdk::Query<"topic", Option<String>>,
    sid: ft_sdk::Cookie<{ ft_sdk::auth::SESSION_KEY }>,
    mut conn: ft_sdk::Connection,
) -> ft_sdk::data::Result {
    let data = match user_data_from_sid(sid, &mut conn) {
        Ok(d) => d,
        Err(_) => return ft_sdk::data::json(serde_json::json!({ "subscribed": false })),
    };

    let subscribed = check_if_subscribed(topic, data);

    ft_sdk::data::json(serde_json::json!({ "subscribed": subscribed }))
}

/// user has subscribed and confirmed subscription by clicking on the double opt-in email
fn check_if_subscribed(topic: Option<String>, user_data: serde_json::Value) -> bool {
    let sub = user_data
        .as_object()
        .and_then(|v| v.get("subscription"))
        .and_then(|v| v.as_object());

    let subscribed = if let Some(topic) = topic {
        sub.and_then(|v| v.get("topics"))
            .and_then(|v| v.as_array())
            .map(|topics| topics.iter().any(|t| t.as_str() == Some(&topic)))
            .unwrap_or_default()
    } else {
        sub.and_then(|v| v.get("subscribed"))
            .and_then(|v| v.as_bool())
            .unwrap_or_default()
    };

    let confirmed = sub
        .and_then(|v| v.get("confirmed"))
        .and_then(|v| v.as_bool())
        .unwrap_or_default();

    subscribed && confirmed
}

/// try to get user data from the session's uid or session's data->'subscription_uid'
fn user_data_from_sid(
    ft_sdk::Cookie(sid): ft_sdk::Cookie<{ ft_sdk::auth::SESSION_KEY }>,
    conn: &mut ft_sdk::Connection,
) -> Result<serde_json::Value, ft_sdk::Error> {
    use diesel::prelude::*;

    let query = if sid.is_some() {
        let session_id = sid.unwrap();
        // WARN: try to get user data from the session's uid
        // if that fails then try to get user data from the session's data->'subscription_uid'
        // reason: there's a situation where `uid` and `subscription_uid` can be different. We
        // favor uid in that situation.
        // The situation: a visitor (not logged in) subscribes. We set a `subscription_uid` with
        // this newly created user. Later this same visitor from the same browser can do a
        // login/signup with a different email (different id) which will update the session store's
        // `uid` field to this new id. The `uid` takes precendence in this case
        // TODO: test this query on postgres
        diesel::sql_query(
            r#"
            WITH first_join AS (
                SELECT
                    fastn_user.data as data
                FROM fastn_user
                JOIN fastn_session
                ON fastn_user.id = fastn_session.uid 
                WHERE fastn_session.id = $1
            ),
            second_join AS (
                SELECT
                    fastn_user.data as data
                FROM fastn_user
                JOIN fastn_session
                ON fastn_user.id = json_extract(fastn_session.data, '$.subscription_uid')
                WHERE fastn_session.id = $1
                AND NOT EXISTS (
                    SELECT 1
                    FROM first_join
                )
            )
            SELECT * FROM first_join
            UNION ALL
            SELECT * FROM second_join
            LIMIT 1;
            "#,
        )
        .bind::<diesel::sql_types::Text, _>(session_id)
    } else {
        return Err(ft_sdk::single_error("user_id", "user_id is required").into());
    };

    #[derive(diesel::QueryableByName)]
    #[diesel(table_name = ft_sdk::auth::fastn_user)]
    struct UserData {
        data: String,
    }

    let d: UserData = query.get_result(conn)?;

    Ok(serde_json::from_str(&d.data)?)
}
