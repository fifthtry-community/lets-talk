// // #[ft_sdk::form]

// // fn schedule_meeting(
// //     title: ft_sdk::Required<"title">,
// //     attendies: ft_sdk::RequiredUsers,
// //     user: crate::auth::RequiredUser,
// //     host: ft_sdk::Host,
// //     config: crate::Config,
// // ) -> ft_sdk::form::Result {
// //     if !user.is_special(&config) {
// //         return Err(title
// //             .error("You are not authorized to create a meeting")
// //             .into());
// //     }

// //     let meeting = crate::dyte::create_meeting(&title)?;
// //     let name = if user.name.is_empty() {
// //         None
// //     } else {
// //         Some(user.name.as_str())
// //     }

// //     let preset = config.preset_host;

// //     let username = crate::dyte::Username::new(user.username, &host);
// //     let participant = crate::dyte::add_participant(&meeting.data.id, %preset, name, username);

// //     let session_cookie = crate::create_session_cookie(
// //         &participant.data.token,
// //         &meeting.data.id,
// //         &host,
// //         config.secure_sessions
// //     )
// // }

// #[ft_sdk::form]
// /// Create a meeting and add the user to it as a participant
// fn schedule_meeting(
//     title: ft_sdk::Required<"title">,
//     date: ft_sdk::Required<"date">,
//     // attendies: ft_sdk::Required<"attendies">,
//     user: crate::auth::RequiredUser,
//     host: ft_sdk::Host,
//     config: crate::Config,
//     app_url: ft_sdk::AppUrl,
//     scheme: crate::HTTPSScheme,
// ) -> ft_sdk::form::Result {
//     println!("Scheduling meeting rust file ran!");
//     // if !user.is_special(&config) {
//     //     return Err(title
//     //         .error("You are not authorized to create a meeting")
//     //         .into());
//     // }

//     // let meeting = crate::dyte::create_meeting(&title)?;
//     // let name = if user.name.is_empty() {
//     //     None
//     // } else {
//     //     Some(user.name.as_str())
//     // };

//     // let date_parts: Vec<&str> = date.split('|').collect();
//     // let start_date = date_parts[0];
//     // let end_date = date_parts[1];



//     // let preset = config.preset_host;

//     // ft_sdk::println!("Using preset: {preset} to schedule a meeting");
//     // ft_sdk::println!("Scheduled meeting for {date}");
//     // // ft_sdk::println!("Attendies for scheduled meeting for {attendies}"); 
//     // let username = crate::dyte::Username::new(user.username, &host);
//     // let participant = crate::dyte::add_participant(&meeting.data.id, &preset, name, username)?;

//     // let session_cookie = crate::create_session_cookie(
//     //     &participant.data.token,
//     //     &meeting.data.id,
//     //     &host,
//     //     config.secure_sessions,
//     // )?;

//     // // lets-talk.fifthtry.site/meeting.ftd
//     // let app_url = crate::temp_fix_app_url(app_url);
//     // let meeting_page_url = app_url.join(&scheme, &host, "meeting")?;

//     // Ok(
//     //     ft_sdk::form::redirect(format!("{meeting_page_url}{}", meeting.data.id))?
//     //         .with_cookie(session_cookie),
//     // )
// }

// // fn create_ics_file()


#[ft_sdk::form]
/// Create a meeting and add the user to it as a participant
fn schedule_meeting(
    title: ft_sdk::Required<"title">,
    start_date: ft_sdk::Required<"start_datetime">,
    end_date: ft_sdk::Required<"end_datetime">,
    attendees: ft_sdk::Required<"attendees">,
    user: crate::auth::RequiredUser,
    host: ft_sdk::Host,
    config: crate::Config,
    app_url: ft_sdk::AppUrl,
    scheme: crate::HTTPSScheme,
) -> ft_sdk::form::Result {
    ft_sdk::println!("Start Date:");
    ft_sdk::println!("{}", start_date);    
    
    ft_sdk::println!("End Date:");
    ft_sdk::println!("{}", end_date);    
        
    if !user.is_special(&config) {
        return Err(title
            .error("You are not authorized to create a meeting")
            .into());
    }
    // ft_sdk::println!("{}", start_date);
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
        &host,
        config.secure_sessions,
    )?;

    // lets-talk.fifthtry.site/meeting.ftd
    let app_url = crate::temp_fix_app_url(app_url);
    let meeting_page_url = app_url.join(&scheme, &host, "meeting")?;
    Ok(
        ft_sdk::form::redirect(format!("{meeting_page_url}{}", meeting.data.id))?
            .with_cookie(session_cookie),
    )
}
