/// Get all sessions of the organization
/// Latest session appears first in the list
#[inline]
pub fn sessions() -> Result<DyteResponse<DyteSessions>, ft_sdk::Error> {
    call_dyte::<DyteSessions>(
        "/sessions?sort_by=createdAt&sort_order=DESC",
        http::Method::GET,
        &None::<serde_json::Value>,
    )
}

/// Get all particpants of a session
#[inline]
pub fn participants(session_id: &str) -> Result<DyteResponse<DyteParticipants>, ft_sdk::Error> {
    let url = format!("/sessions/{session_id}/participants");
    call_dyte::<DyteParticipants>(&url, http::Method::GET, &None::<serde_json::Value>)
}

#[inline]
pub fn add_participant(
    meeting_id: &str,
    preset_name: &str,
    participant_name: Option<&str>,
    username: Username,
) -> Result<DyteResponse<DyteAddParticipant>, ft_sdk::Error> {
    let body = &mut serde_json::json!({
        "preset_name": preset_name,
        "custom_participant_id": username.as_str(),
    });

    // Dyte's api does not like name: null. So we have to remove the key altogether
    if let Some(name) = participant_name {
        body.as_object_mut()
            .unwrap()
            .insert("name".to_string(), serde_json::json!(name));
    }

    call_dyte::<DyteAddParticipant>(
        &format!("/meetings/{}/participants", meeting_id),
        http::Method::POST,
        body,
    )
}

#[inline]
pub fn create_meeting(title: &str) -> Result<DyteResponse<DyteCreateMeeting>, ft_sdk::Error> {
    call_dyte::<DyteCreateMeeting>(
        "/meetings",
        http::Method::POST,
        &serde_json::json!({
            "title": title,
        }),
    )
}

/// Call the Dyte API
/// `path` is prefixed with the base dyte api URL
/// This is generic over the Dyte `data` response
fn call_dyte<D: DyteData>(
    path: &str,
    method: http::Method,
    body: &(impl serde::Serialize + std::fmt::Debug),
) -> Result<DyteResponse<D>, ft_sdk::Error> {
    use base64::Engine as _;

    ft_sdk::println!("Calling with data:");
    ft_sdk::println!("{:?}", body);

    // DYTE_ORG_ID and DYTE_API_KEY are accessible to all the websites hosted on FifthTry
    let dyte_org_id = ft_sdk::env::var("DYTE_ORG_ID".to_string()).ok_or_else(|| {
        ft_sdk::anyhow!("DYTE_ORG_ID is not set. Please set it in the environment variables")
    })?;

    let dyte_api_key = ft_sdk::env::var("DYTE_API_KEY".to_string()).ok_or_else(|| {
        ft_sdk::anyhow!("DYTE_API_KEY is not set. Please set it in the environment variables")
    })?;

    let key = base64::engine::general_purpose::STANDARD
        .encode(format!("{}:{}", dyte_org_id, dyte_api_key));

    let authorization_header = format!("Basic {}", key);

    let url = format!("https://api.dyte.io/v2{path}");
    let body = bytes::Bytes::from(serde_json::to_vec(body).unwrap());

    let client = http::Request::builder();

    let request = client
        .method(method)
        .uri(url)
        .header("Authorization", authorization_header)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .body(body)?;

    let response = ft_sdk::http::send(request).unwrap(); //todo: remove unwrap()
    ft_sdk::println!("=====================================");
    ft_sdk::println!("{:?}", response);
    ft_sdk::println!("=====================================");

    DyteData::deserialize(String::from_utf8_lossy(response.body()))
}

pub struct Username(String);

impl Username {
    pub fn new<S: AsRef<str>>(username: S, host: &ft_sdk::Host) -> Self {
        Self(format!("{}__{}", host.without_port(), username.as_ref()))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct DyteResponse<T> {
    #[expect(unused)]
    pub success: bool,
    pub data: T,
}

#[derive(serde::Deserialize, Debug)]
/// See [create_meeting] to create instance of this type
/// https://docs.dyte.io/api/#/operations/create_meeting
pub struct DyteCreateMeeting {
    pub id: String,
    #[expect(unused)]
    pub title: String,
}

#[derive(serde::Deserialize, Debug)]
/// See [add_participant] to create instance of this type
/// https://docs.dyte.io/api/#/operations/add_participant
pub struct DyteAddParticipant {
    #[expect(unused)]
    pub id: String,
    // The docs say `preset_name` but the server returns `preset_id`
    #[expect(unused)]
    pub preset_id: String,
    pub token: String,
}

/// See [sessions] to create instance of this type
/// https://docs.dyte.io/api#/operations/GetSessions
#[derive(serde::Deserialize, Debug)]
pub struct DyteSessions {
    pub sessions: Vec<DyteSession>,
}

/// See [sessions] to create instance of this type
/// https://docs.dyte.io/api#/operations/GetSessions
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct DyteSession {
    pub id: String,
    #[serde(rename = "associated_id")]
    pub meeting_id: String,
    pub meeting_display_name: String,
}

/// See [participants] to create instance of this type
/// https://docs.dyte.io/api#/operations/GetSessionParticipants
#[derive(serde::Deserialize, Debug)]
pub struct DyteParticipants {
    pub participants: Vec<DyteParticipant>,
}

/// See [participants] to create instance of this type
/// https://docs.dyte.io/api#/operations/GetSessionParticipants
#[derive(serde::Deserialize, Debug)]
pub struct DyteParticipant {
    #[expect(unused)]
    pub id: String,
    pub custom_participant_id: String,
    #[expect(unused)]
    pub display_name: String,
    pub duration: f64,
    pub joined_at: String,
    pub left_at: String,
}

trait DyteData: serde::de::DeserializeOwned {
    fn deserialize(str: impl AsRef<str>) -> Result<DyteResponse<Self>, ft_sdk::Error> {
        serde_json::from_str(str.as_ref())
            .map_err(|e| ft_sdk::anyhow!("Failed to deserialize `data`: {:?}", e))
    }
}

impl DyteData for DyteCreateMeeting {}
impl DyteData for DyteAddParticipant {}
impl DyteData for DyteSessions {}
impl DyteData for DyteParticipants {}
