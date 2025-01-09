// https://dev.dyte.io/presets?orgId=f233a0a4-afa0-4afd-9969-971d77e799d8
pub const MEETING_PRESET_HOST: &str = "group_call_host";

#[inline]
pub fn add_participant(
    meeting_id: &str,
    preset_name: &str,
    custom_participant_id: &str,
) -> Result<DyteResponse<DyteAddParticipant>, ft_sdk::Error> {
    call_dyte::<DyteAddParticipant>(
        &format!("/meetings/{}/participants", meeting_id),
        http::Method::POST,
        &serde_json::json!({
            "preset_name": preset_name,
            "custom_participant_id": custom_participant_id,
        }),
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

const DYTE_ORG_ID: &str = env!("DYTE_ORG_ID");
const DYTE_API_KEY: &str = env!("DYTE_API_KEY");

/// Call the Dyte API
/// `path` is prefixed with the base dyte api URL
/// This is generic over the Dyte `data` response
fn call_dyte<D: DyteData>(
    path: &str,
    method: http::Method,
    body: &impl serde::Serialize,
) -> Result<DyteResponse<D>, ft_sdk::Error> {
    use base64::Engine as _;

    let key = base64::engine::general_purpose::STANDARD
        .encode(format!("{}:{}", DYTE_ORG_ID, DYTE_API_KEY));

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

#[derive(serde::Deserialize, Debug)]
pub struct DyteResponse<T> {
    pub success: bool,
    pub data: T,
}

#[derive(serde::Deserialize, Debug)]
/// See [create_meeting] to create instance of this type
/// https://docs.dyte.io/api/#/operations/create_meeting
pub struct DyteCreateMeeting {
    pub id: String,
    pub title: String,
}

#[derive(serde::Deserialize, Debug)]
/// See [add_participant] to create instance of this type
/// https://docs.dyte.io/api/#/operations/add_participant
pub struct DyteAddParticipant {
    pub id: String,
    // The docs say `preset_name` but the server returns `preset_id`
    pub preset_id: String,
    pub token: String,
}

trait DyteData: serde::de::DeserializeOwned {
    fn deserialize(str: impl AsRef<str>) -> Result<DyteResponse<Self>, ft_sdk::Error> {
        serde_json::from_str(str.as_ref())
            .map_err(|e| ft_sdk::anyhow!("Failed to deserialize `data`: {:?}", e))
    }
}

impl DyteData for DyteCreateMeeting {}
impl DyteData for DyteAddParticipant {}
