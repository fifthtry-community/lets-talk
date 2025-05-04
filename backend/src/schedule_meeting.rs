use crate::uuid::gen_uuid_with_xorshift;
use std::env;
#[ft_sdk::form]

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
    // Uncommeneted for debugging reasons
    // if !user.is_special(&config) {
    //     return Err(title
    //         .error("You are not authorized to create a meeting")
    //         .into());
    // }

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
    ft_sdk::println!("Meeting.data.id: {:?}", &meeting.data.id);
    let session_cookie = crate::create_session_cookie(
        &participant.data.token,
        &meeting.data.id,
        &host,
        config.secure_sessions,
    )?;

    // lets-talk.fifthtry.site/meeting.ftd
    let app_url = crate::temp_fix_app_url(app_url);
    let meeting_page_url = app_url.join(&scheme, &host, "meeting")?;
    let start_date_debug = "20250625T100000Z";
    let end_date_debug = "20250625T100000Z";
    let organizer_email = "something@gmail.com";
    let uuid = gen_uuid_with_xorshift(0.1);
     
    let output = return_ics_file(
        &title, 
        &meeting_page_url, 
        "ayush@deolasee.org, somethingelse@gmail.com, nothing@nothing", 
        uuid,
        start_date_debug.to_string(), 
        end_date_debug.to_string(), 
        organizer_email.to_string() 
    );
    
    
    Ok(
        ft_sdk::form::redirect("")?
        // Removed for debugging reasons 
        // ft_sdk::form::redirect(format!("{meeting_page_url}{}", meeting.data.id))?
        //     .with_cookie(session_cookie),
    )
}

// meeting_name &str, start_date: &str, end_date: &str
fn return_ics_file(meeting_title: &str, meeting_url: &str, attendees: &str, uid: String, start_datetime: String, end_datetime: String, organizer_email: String) -> std::io::Result<String> {
    let mut ics_string = String::new(); 
    let mut attendees_list = Vec::<String>::new();
    
    for attendee in attendees.split(',') {
        let attendee = attendee.trim();
        if !attendee.is_empty() {
            if !attendee.contains('@') {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Email address: {}", attendee),
                ));
            } else {
                attendees_list.push(attendee.to_string());
            }
        }
    }

    if attendees_list.is_empty() && !attendees.trim().is_empty() {
        return Err(std::io::Error::new(
           std::io::ErrorKind::InvalidInput,
           "No valid attendee email addresses provided.",
       ));
   }

    ics_string.push_str(format!(
        "BEGIN:VCALENDAR\nVERSION:2.0\nPRODID:fifthtry/lets-talk\nCALSCALE:GREGORIAN\nMETHOD:REQUEST\nBEGIN:VEVENT\nUID:{uid}\nDISTAMP:{start_datetime}\nSUMMARY:{meeting_title}\nLOCATION:{meeting_url}\nDTSTART:{start_datetime}\nDTEND:{end_datetime}\nORGANIZER:mailto:{organizer_email}\n"
    ).as_str());

    for attendee in attendees_list {
        ics_string.push_str(format!("ATTENDEE;ROLE=REQ-PARTICIPANT;RSVP=TRUE;PARTSTAT=NEEDS-ACTION:mailto:{attendee}\n").as_str());    
    }

    ics_string.push_str(
        format!(
        "SEQUENCE:0\nTRANSP:OPAQUE\nEND:VEVENT\nEND:VCALENDAR"
    ).as_str());

    Ok(ics_string)
}


fn send_calender_invite(ics_content: String, organizer_email: &str, attendee_email: &str, meeting_title: &str) -> std::io::Result<()> {
    let email = Message::builder()
        .from(organzier_email.parse()?)
        .to(attendee_email.parse()?)
        .subject(meeting_title)
        .multipart(
            MultiPart::mixed()
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_PLAIN)
                        .body(String::from("Please find attached the calendar invitation for our upcoming meeting."))
                )
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::parse("text/calendar; method=REQUEST").unwrap())
                        .header(header::ContentDisposition::attachment("invite.ics"))
                        .body(ics_content)
                ),
        )?;

        // TODO: Add instructions in README file to export STMP_USERNAME and STMP_PASSWORD as env variables
        let smtp_username = env::var("SMTP_USERNAME")
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::NotFound, format!("SMTP_USERNAME environment variable not found: {}", e)))?;
        let smtp_password = env::var("SMTP_PASSWORD")
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::NotFound, format!("SMTP_PASSWORD environment variable not found: {}", e)))?;


        let mailer = SmtpTransport::relay("smtp.gmail.com")? 
        .credentials(Credentials::new(
            smtp_username.to_string(),
            smtp_password.to_string(),
        ))
        .build();

        Send the email
        mailer.send(&email)?;

    Ok(())
}