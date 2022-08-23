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

pub fn get_element_json_value<Msg, AppEffect>(dom_id: &DomId) -> Effect<Msg, AppEffect> {
    Effect::Dom(Dom::GetElementValue {
        element_id: dom_id.clone(),
        parse_as_json: true,
    })
}

pub fn get_element_string_value<Msg, AppEffect>(dom_id: &DomId) -> Effect<Msg, AppEffect> {
    Effect::Dom(Dom::GetElementValue {
        element_id: dom_id.clone(),
        parse_as_json: false,
    })
}

pub fn window_size<Msg, AppEffect>() -> Effect<Msg, AppEffect> {
    Effect::Dom(Dom::GetWindowSize)
}
