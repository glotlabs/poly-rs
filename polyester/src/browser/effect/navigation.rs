use crate::browser::Effect;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "config")]
#[serde(rename_all = "camelCase")]
pub enum Navigation {
    PushUrl(String),
    ReplaceUrl(String),
    SetLocation(String),
}

pub fn push_url<Msg, AppEffect>(url: &str) -> Effect<Msg, AppEffect> {
    Effect::Navigation(Navigation::PushUrl(url.to_string()))
}

pub fn replace_url<Msg, AppEffect>(url: &str) -> Effect<Msg, AppEffect> {
    Effect::Navigation(Navigation::ReplaceUrl(url.to_string()))
}

pub fn set_location<Msg, AppEffect>(url: &str) -> Effect<Msg, AppEffect> {
    Effect::Navigation(Navigation::SetLocation(url.to_string()))
}
