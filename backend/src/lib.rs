#![deny(unused_crate_dependencies)]
extern crate self as lets_talk;

mod auth;
mod dyte;
mod uuid;

mod create_meeting;
mod token;
mod session;

const TALK_TOKEN_COOKIE: &str = "talk-token";

fn create_session_cookie(
    token: &str,
    meeting_id: &str,
    host: ft_sdk::Host,
) -> Result<http::HeaderValue, ft_sdk::Error> {
    let talk_secure_sessions = ft_sdk::env::var("LETS_TALK_SECURE_SESSIONS".to_string()).is_some();

    let val = format!("{}:{}", meeting_id, token);
    let cookie = cookie::Cookie::build((TALK_TOKEN_COOKIE, val))
        .domain(host.without_port())
        // TODO: make this so that only mountpoint can access the cookie
        .path("/")
        .max_age(cookie::time::Duration::seconds(34560000))
        .same_site(cookie::SameSite::Strict)
        .secure(talk_secure_sessions)
        .build();

    Ok(http::HeaderValue::from_str(cookie.to_string().as_str())?)
}
