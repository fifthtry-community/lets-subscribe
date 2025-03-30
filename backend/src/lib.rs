#![allow(clippy::derive_partial_eq_without_eq, clippy::get_first)]
#![deny(unused_crate_dependencies)]
#![warn(clippy::used_underscore_binding)]
#![forbid(unsafe_code)]

extern crate self as subscription;

mod confirm_email_templ;
mod confirm_subscription;
mod is_subscribed;
mod subscribe;
mod set_tracker;
mod unsubscribe;
mod welcome_email_templ;

pub(crate) use confirm_subscription::mark_subscription_verified;
pub(crate) use confirm_subscription::send_welcome_email;
pub(crate) use subscribe::email_from_address_from_env;

pub const EMAIL_PROVIDER_ID: &str = "email";
pub const SUBSCRIPTION_PROVIDER_ID: &str = "subscription";
pub const DEFAULT_REDIRECT_ROUTE: &str = "/";
// TODO: make this configurable as well. We need DKIM support among other things before we can do
// this
pub const EMAIL_SENDER: &str = "support@fifthtry.com";

pub fn tracker_cookie(tid: &str, host: ft_sdk::Host) -> Result<http::HeaderValue, ft_sdk::Error> {
    let cookie = cookie::Cookie::build((ft_sdk::session::TRACKER_KEY, tid))
        .domain(host.without_port())
        .path("/")
        .max_age(cookie::time::Duration::seconds(34560000))
        .same_site(cookie::SameSite::Strict)
        .build();

    Ok(http::HeaderValue::from_str(cookie.to_string().as_str())?)
}

pub fn session_cookie(sid: &str, host: ft_sdk::Host) -> Result<http::HeaderValue, ft_sdk::Error> {
    // DO NOT CHANGE THINGS HERE, consult logout code in fastn.
    let cookie = cookie::Cookie::build((ft_sdk::auth::SESSION_KEY, sid))
        .domain(host.without_port())
        .path("/")
        .max_age(cookie::time::Duration::seconds(34560000))
        .same_site(cookie::SameSite::Strict)
        .build();

    Ok(http::HeaderValue::from_str(cookie.to_string().as_str())?)
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    email_sender_name: String,
    email_reply_to: String,
}

impl Config {
    pub fn from_email(&self) -> ft_sdk::EmailAddress {
        ft_sdk::EmailAddress {
            name: Some(self.email_sender_name.clone()),
            email: EMAIL_SENDER.to_string(),
        }
    }

    pub fn reply_to(&self) -> ft_sdk::EmailAddress {
        ft_sdk::EmailAddress {
            name: Some(self.email_sender_name.clone()),
            email: self.email_reply_to.clone(),
        }
    }
}
