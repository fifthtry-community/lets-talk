#![deny(unused_crate_dependencies)]

mod auth;
mod dyte;
mod uuid;

const TALK_TOKEN_COOKIE: &str = "talk-token";

#[ft_sdk::data]
fn ping() -> ft_sdk::data::Result {
    ft_sdk::data::json(serde_json::json!({ "ping": "pong" }))
}

#[ft_sdk::form]
/// Create a meeting and add the user to it as a participant
fn create_meeting(
    title: ft_sdk::Required<"title">,
    user: auth::RequiredUser,
    mut conn: ft_sdk::Connection,
) -> ft_sdk::form::Result {
    let is_staff = auth::is_fifthtry_staff_org_member(&mut conn, user.user_id)?;

    if !is_staff {
        return Err(title
            .error("You are not authorized to create a meeting")
            .into());
    }

    let meeting = dyte::create_meeting(&title)?;

    ft_sdk::form::redirect(format!("/talk/{}", meeting.data.id,))
}

/// Return the token from active session to join the meeting, or,
/// Add the logged in user as participant to the meeting and create a session, or,
/// Use provided name and create guest user if not logged in
#[ft_sdk::processor]
fn session(
    ft_sdk::Cookie(token): ft_sdk::Cookie<{ TALK_TOKEN_COOKIE }>,
    meeting_id: ft_sdk::Query<"meeting-id", String>,
) -> ft_sdk::processor::Result {
    ft_sdk::println!("======= in session handler ======");
    ft_sdk::println!("token: {:?}", token);
    if let Some(token) = token {
        ft_sdk::println!("session cookie set");
        let (mid, token) = token.split_once(':').expect("token format error");

        if mid == meeting_id.0 {
            ft_sdk::println!("session cookie found for current meeting");
            let session = TalkSession {
                token: Some(token.to_string()),
            };
            return ft_sdk::processor::json(session);
        }
    }

    return ft_sdk::processor::temporary_redirect(format!(
        "/talk/api/session/new/?meeting-id={meeting_id}"
    ));
}

#[ft_sdk::data]
fn session_new(
    meeting_id: ft_sdk::Query<"meeting-id", String>,
    user: auth::OptionalUser,
    host: ft_sdk::Host,
) -> ft_sdk::data::Result {
    ft_sdk::println!("======= in session new handler ======");

    let (username, name) = if user.is_logged_in {
        ft_sdk::println!("Found user name through login");
        (user.username, Some(user.name))
    } else {
        let seed: f64 = ft_sdk::env::random();
        let uuid = uuid::gen_uuid_with_xorshift(seed);
        ft_sdk::println!("adding guest with id: {uuid}");
        // The frontend will ask for the name
        (uuid, None)
    };

    let participant = dyte::add_participant(
        &meeting_id,
        dyte::MEETING_PRESET_PARTICIPANT,
        name.as_deref(),
        &username,
    )?;

    ft_sdk::println!("dyte response: {:?}", participant);

    let session_cookie = create_session_cookie(&participant.data.token, &meeting_id, host)?;

    ft_sdk::data::browser_redirect_with_cookie(format!("/talk/{meeting_id}"), session_cookie)
}

/// Get past sessions of the logged in user
/// This call is expensive, should be done in the background
#[ft_sdk::data]
fn past_sessions(user: auth::RequiredUser) -> ft_sdk::data::Result {
    let all_sessions = dyte::sessions()?
        .data
        .sessions
        .into_iter()
        .filter_map(|s| {
            let participants = match dyte::participants(&s.id) {
                Ok(v) => v.data.participants,
                Err(e) => {
                    ft_sdk::println!("Error getting participants: {:?}", e);
                    return None;
                }
            };

            let Some(p) = participants
                .into_iter()
                .find(|p| p.custom_participant_id == user.username)
            else {
                return None;
            };

            let joined_at = parse_timestamp_to_human_time(p.joined_at);
            let left_at = parse_timestamp_to_human_time(p.left_at);

            Some(UserSession {
                id: s.id,
                meeting_title: s.meeting_display_name,
                duration: p.duration,
                joined_at,
                left_at,
            })
        })
        .collect::<Vec<_>>();

    ft_sdk::data::json(all_sessions)
}

#[derive(serde::Serialize)]
struct UserSession {
    id: String,
    meeting_title: String,
    duration: f64,
    joined_at: String,
    left_at: String,
}

#[derive(serde::Serialize)]
struct TalkSession {
    /// <mid>:<token>
    token: Option<String>,
}

fn create_session_cookie(
    token: &str,
    meeting_id: &str,
    host: ft_sdk::Host,
) -> Result<http::HeaderValue, ft_sdk::Error> {
    let talk_secure_sessions = ft_sdk::env::var("TALK_SECURE_SESSIONS".to_string()).is_some();

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

fn parse_timestamp_to_human_time(timestamp: String) -> String {
    let parsed_time: chrono::DateTime<chrono::Utc> = timestamp.parse().unwrap();
    parsed_time.format("%Y-%m-%d at %H:%M hrs").to_string()
}
