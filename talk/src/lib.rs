#[ft_sdk::data]
fn ping() -> ft_sdk::data::Result {
    ft_sdk::data::json(serde_json::json!({ "ping": "pong" }))
}
