#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum QueueStrategy {
    Fifo,
    DropOlder,
}
