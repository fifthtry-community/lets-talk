mod auth;
mod dyte;

#[ft_sdk::data]
fn ping() -> ft_sdk::data::Result {
    ft_sdk::data::json(serde_json::json!({ "ping": "pong" }))
}

#[ft_sdk::form]
/// Create a meeting and add the user to it as a participant
fn create_meeting(
    ft_sdk::Query(title): ft_sdk::Query<"title">,
    user: auth::RequiredUser,
) -> ft_sdk::form::Result {
    ft_sdk::println!("======= in handler ======");
    let meeting = dyte::create_meeting(&title)?;
    ft_sdk::println!("{:?}", meeting);

    let participant =
        dyte::add_participant(&meeting.data.id, dyte::MEETING_PRESET_HOST, &user.username)?;

    ft_sdk::form::redirect(format!(
        "/talk/?mid={}&token={}",
        meeting.data.id, participant.data.token
    ))
}
