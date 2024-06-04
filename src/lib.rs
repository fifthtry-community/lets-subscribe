extern crate self as auth;

mod handlers;

pub const PROVIDER_ID: &str = "subscription";
pub const TRACKER: &str = "fastn_tid";
pub const DEFAULT_REDIRECT_ROUTE: &str = "/";

pub fn tracker_cookie(tid: &str, host: ft_sdk::Host) -> Result<http::HeaderValue, ft_sdk::Error> {
    // DO NOT CHANGE THINGS HERE, consult logout code in fastn.
    let cookie = cookie::Cookie::build((TRACKER, tid))
        .domain(host.without_port())
        .path("/")
        .max_age(cookie::time::Duration::seconds(34560000))
        .same_site(cookie::SameSite::Strict)
        .build();

    Ok(http::HeaderValue::from_str(cookie.to_string().as_str())?)
}
