/// Get past sessions of the logged-in user
/// This call is expensive, should be done in the background
#[ft_sdk::data]
fn past_sessions(user: crate::auth::RequiredUser, host: ft_sdk::Host) -> ft_sdk::data::Result {
    let all_sessions = crate::dyte::sessions()?
        .data
        .sessions
        .into_iter()
        .filter_map(|s| {
            let participants = match crate::dyte::participants(&s.id) {
                Ok(v) => v.data.participants,
                Err(e) => {
                    ft_sdk::println!("Error getting participants: {e:?}");
                    return None;
                }
            };

            let username = crate::dyte::Username::new(&user.username, &host);

            let p = participants
                .into_iter()
                .find(|p| p.custom_participant_id == username.as_str())?;

            Some(UserSession {
                id: s.id,
                meeting_title: s.meeting_display_name,
                duration: p.duration,
                joined_at: p.joined_at,
                left_at: p.left_at,
            })
        })
        .collect::<Vec<_>>();

    ft_sdk::data::json(serde_json::json!({"sessions": all_sessions}))
}

#[derive(serde::Serialize)]
struct UserSession {
    id: String,
    meeting_title: String,
    duration: f64,
    joined_at: String,
    left_at: String,
}
