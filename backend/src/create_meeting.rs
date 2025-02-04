#[ft_sdk::form]
/// Create a meeting and add the user to it as a participant
fn create_meeting(
    title: ft_sdk::Required<"title">,
    ft_sdk::Required(meeting_page_url): ft_sdk::Required<"meeting-page-url">,
    user: crate::auth::RequiredUser,
    host: ft_sdk::Host,
    config: crate::Config,
) -> ft_sdk::form::Result {
    if !user.is_special(&config) {
        return Err(title
            .error("You are not authorized to create a meeting")
            .into());
    }

    let meeting = crate::dyte::create_meeting(&title)?;

    let name = if user.name.is_empty() {
        None
    } else {
        Some(user.name.as_str())
    };

    let preset = config.preset_host;

    ft_sdk::println!("Using preset: {preset} to create meeting");

    let username = crate::dyte::Username::new(user.username, &host);
    let participant = crate::dyte::add_participant(&meeting.data.id, &preset, name, username)?;

    let session_cookie = crate::create_session_cookie(
        &participant.data.token,
        &meeting.data.id,
        host,
        config.secure_sessions,
    )?;

    Ok(
        ft_sdk::form::redirect(format!("{meeting_page_url}{}", meeting.data.id))?
            .with_cookie(session_cookie),
    )
}
