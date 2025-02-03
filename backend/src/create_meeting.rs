#[ft_sdk::form]
/// Create a meeting and add the user to it as a participant
fn create_meeting(
    title: ft_sdk::Required<"title">,
    ft_sdk::Required(meeting_page_url): ft_sdk::Required<"meeting-page-url">,
    user: crate::auth::RequiredUser,
    scheme: ft_sdk::Scheme,
    host: ft_sdk::Host,
    current_app_url: ft_sdk::AppUrl,
    mut conn: ft_sdk::Connection,
) -> ft_sdk::form::Result {
    let config = crate::config(&scheme, &host, &current_app_url)
        .inspect_err(|e| ft_sdk::println!("[create-meeting] Failed to read config: {e}"))
        .unwrap_or_default();

    if !user.is_special(&mut conn, &config) {
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

    let participant =
        crate::dyte::add_participant(&meeting.data.id, &preset, name, &user.username)?;

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
