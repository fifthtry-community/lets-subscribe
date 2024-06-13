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
    tid: ft_sdk::Cookie<"fastn-tid">,
    mut conn: ft_sdk::Connection,
) -> ft_sdk::data::Result {
    let data = match user_data_from_sid_or_tid(sid, tid, &mut conn) {
        Ok(d) => d,
        Err(_) => return ft_sdk::data::json(serde_json::json!({ "subscribed": false })),
    };

    let subscribed = check_if_subscribed(topic, data);

    ft_sdk::data::json(serde_json::json!({ "subscribed": subscribed }))
}

fn check_if_subscribed(topic: Option<String>, user_data: serde_json::Value) -> bool {
    let subscribed = if let Some(topic) = topic {
        user_data
            .as_object()
            .and_then(|v| v.get("subscription"))
            .and_then(|v| v.as_object())
            .and_then(|v| v.get("topics"))
            .and_then(|v| v.as_array())
            .map(|topics| topics.iter().any(|t| t.as_str() == Some(&topic)))
            .unwrap_or_default()
    } else {
        user_data
            .as_object()
            .and_then(|v| v.get("subscription"))
            .and_then(|v| v.as_object())
            .and_then(|v| v.get("subscribed"))
            .and_then(|v| v.as_bool())
            .unwrap_or_default()
    };

    subscribed
}

fn user_data_from_sid_or_tid(
    ft_sdk::Cookie(user_id_from_session): ft_sdk::Cookie<{ ft_sdk::auth::SESSION_KEY }>,
    ft_sdk::Cookie(user_id_from_tracker): ft_sdk::Cookie<"fastn-tid">,
    conn: &mut ft_sdk::Connection,
) -> Result<serde_json::Value, ft_sdk::Error> {
    use diesel::prelude::*;

    let query;
    if user_id_from_session.is_some() {
        let user_id = user_id_from_session.unwrap();
        query = diesel::sql_query(
            r#"
            SELECT
                fastn_user.data as data
            FROM fastn_user
            JOIN fastn_session
            WHERE
                fastn_session.id = $1
                AND fastn_user.id = fastn_session.uid
            LIMIT 1;
            "#,
        )
        .bind::<diesel::sql_types::Text, _>(user_id);
    } else if user_id_from_tracker.is_some() {
        let user_id = user_id_from_tracker.unwrap();
        query = diesel::sql_query(
            r#"
            SELECT
                fastn_user.data as data
            FROM fastn_user
            JOIN fastn_tracker
            WHERE
                fastn_tracker.id = $1
                AND fastn_user.id = fastn_tracker.uid
            LIMIT 1;
            "#,
        )
        .bind::<diesel::sql_types::Text, _>(user_id);
    } else {
        return Err(ft_sdk::single_error("user_id", "user_id is required").into());
    }

    #[derive(diesel::QueryableByName)]
    #[diesel(table_name = ft_sdk::auth::fastn_user)]
    struct UserData {
        data: String,
    }

    let d: UserData = query.get_result(conn)?;

    Ok(serde_json::from_str(&d.data)?)
}
