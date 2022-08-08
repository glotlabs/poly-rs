use crate::browser::Effect;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "config")]
#[serde(rename_all = "camelCase")]
pub enum Navigation {
    PushUrl(String),
    ReplaceUrl(String),
}

pub fn push_url<Msg>(url: &str) -> Effect<Msg> {
    Effect::Navigation(Navigation::PushUrl(url.to_string()))
}

pub fn replace_url<Msg>(url: &str) -> Effect<Msg> {
    Effect::Navigation(Navigation::ReplaceUrl(url.to_string()))
}
