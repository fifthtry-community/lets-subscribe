extern crate self as subscription;

mod subscribe;
mod unsubscribe;
mod confirm_subscription;
mod confirm_email_templ;
mod welcome_email_templ;
mod is_subscribed;
mod t; // set-tracker


pub(crate) use confirm_subscription::mark_user_verified;
pub(crate) use confirm_subscription::send_welcome_email;
pub(crate) use subscribe::email_from_address_from_env;

pub const EMAIL_PROVIDER_ID: &str = "email";
pub const SUBSCRIPTION_PROVIDER_ID: &str = "subscription";
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
