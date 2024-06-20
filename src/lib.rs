extern crate self as subscription;

mod confirm_email_templ;
mod confirm_subscription;
mod is_subscribed;
mod subscribe;
mod set_tracker;
mod unsubscribe;
mod welcome_email_templ; // set-tracker

pub(crate) use confirm_subscription::mark_user_verified;
pub(crate) use confirm_subscription::send_welcome_email;
pub(crate) use subscribe::email_from_address_from_env;

pub const EMAIL_PROVIDER_ID: &str = "email";
pub const SUBSCRIPTION_PROVIDER_ID: &str = "subscription";
pub const DEFAULT_REDIRECT_ROUTE: &str = "/";

pub fn tracker_cookie(tid: &str, host: ft_sdk::Host) -> Result<http::HeaderValue, ft_sdk::Error> {
    let cookie = cookie::Cookie::build((ft_sdk::tracker::TRACKER_KEY, tid))
        .domain(host.without_port())
        .path("/")
        .max_age(cookie::time::Duration::seconds(34560000))
        .same_site(cookie::SameSite::Strict)
        .build();

    Ok(http::HeaderValue::from_str(cookie.to_string().as_str())?)
}
