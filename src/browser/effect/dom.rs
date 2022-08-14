use crate::browser::DomId;
use crate::browser::Effect;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "config")]
#[serde(rename_all = "camelCase")]
pub enum Dom {
    #[serde(rename_all = "camelCase")]
    GetElementValue {
        element_id: DomId,
        parse_as_json: bool,
    },
    GetWindowSize,
}

pub fn element_value<Msg, CustomEffect>(dom_id: &DomId) -> Effect<Msg, CustomEffect> {
    Effect::Dom(Dom::GetElementValue {
        element_id: dom_id.clone(),
        parse_as_json: true,
    })
}

pub fn element_value_raw<Msg, CustomEffect>(dom_id: &DomId) -> Effect<Msg, CustomEffect> {
    Effect::Dom(Dom::GetElementValue {
        element_id: dom_id.clone(),
        parse_as_json: false,
    })
}

pub fn window_size<Msg, CustomEffect>() -> Effect<Msg, CustomEffect> {
    Effect::Dom(Dom::GetWindowSize)
}
