#![deny(unused_crate_dependencies)]
extern crate self as lets_talk;

mod auth;
mod dyte;
mod uuid;

mod create_meeting;
mod session;
mod token;

const TALK_TOKEN_COOKIE: &str = "talk-token";

fn create_session_cookie(
    token: &str,
    meeting_id: &str,
    host: ft_sdk::Host,
    secure: bool,
) -> Result<http::HeaderValue, ft_sdk::Error> {
    let val = format!("{}:{}", meeting_id, token);
    let cookie = cookie::Cookie::build((TALK_TOKEN_COOKIE, val))
        .domain(host.without_port())
        // TODO: make this so that only mountpoint can access the cookie
        .path("/")
        .max_age(cookie::time::Duration::seconds(34560000))
        .same_site(cookie::SameSite::Strict)
        .secure(secure)
        .http_only(secure)
        .build();

    Ok(http::HeaderValue::from_str(cookie.to_string().as_str())?)
}

pub(crate) fn config(
    scheme: &ft_sdk::Scheme,
    host: &ft_sdk::Host,
    current_app_url: &ft_sdk::AppUrl,
) -> Result<Config, ft_sdk::Error> {
    let url = current_app_url.join(scheme, host, "config")?;
    ft_sdk::println!("Requesting config from url: {url}");

    let req = http::Request::builder()
        .uri(url)
        .body(bytes::Bytes::new())?;

    let res = ft_sdk::http::send(req).unwrap();

    serde_json::from_slice(res.body()).map_err(|e| e.into())
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct Config {
    pub allowed_emails: Vec<String>,
    pub preset_host: String,
    pub preset_participant: String,
    pub require_verification: bool,
    pub secure_sessions: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            preset_host: "group_call_host".to_string(),
            preset_participant: "group_call_participant".to_string(),
            require_verification: false,
            secure_sessions: false,
            allowed_emails: vec![],
        }
    }
}
