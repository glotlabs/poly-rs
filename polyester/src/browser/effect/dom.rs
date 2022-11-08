use crate::browser::DomId;
use crate::browser::Effect;
use crate::browser::EventTarget;
use crate::browser::Selector;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "config")]
#[serde(rename_all = "camelCase")]
pub enum Dom {
    #[serde(rename_all = "camelCase")]
    FocusElement {
        element_id: String,
    },
    #[serde(rename_all = "camelCase")]
    SelectInputText {
        element_id: String,
    },
    #[serde(rename_all = "camelCase")]
    GetElementValue {
        element_id: String,
        parse_as_json: bool,
    },
    #[serde(rename_all = "camelCase")]
    GetRadioGroupValue {
        selector: Selector,
        parse_as_json: bool,
    },
    #[serde(rename_all = "camelCase")]
    GetTargetDataValue {
        name: String,
        parse_as_json: bool,
    },
    #[serde(rename_all = "camelCase")]
    GetFiles {
        element_id: String,
    },
    GetWindowSize,
    #[serde(rename_all = "camelCase")]
    DispatchEvent {
        event_target: EventTarget,
        event_type: String,
        bubbles: bool,
        cancelable: bool,
    },
}

pub fn focus_element<Msg, AppEffect, Id>(id: Id) -> Effect<Msg, AppEffect>
where
    Id: DomId,
{
    Effect::Dom(Dom::FocusElement {
        element_id: id.to_string(),
    })
}

pub fn select_input_text<Msg, AppEffect, Id>(id: Id) -> Effect<Msg, AppEffect>
where
    Id: DomId,
{
    Effect::Dom(Dom::SelectInputText {
        element_id: id.to_string(),
    })
}

pub fn get_element_json_value<Msg, AppEffect, Id>(id: Id) -> Effect<Msg, AppEffect>
where
    Id: DomId,
{
    Effect::Dom(Dom::GetElementValue {
        element_id: id.to_string(),
        parse_as_json: true,
    })
}

pub fn get_element_string_value<Msg, AppEffect, Id>(id: Id) -> Effect<Msg, AppEffect>
where
    Id: DomId,
{
    Effect::Dom(Dom::GetElementValue {
        element_id: id.to_string(),
        parse_as_json: false,
    })
}

pub fn get_radio_group_json_value<Msg, AppEffect>(selector: &Selector) -> Effect<Msg, AppEffect> {
    Effect::Dom(Dom::GetRadioGroupValue {
        selector: selector.clone(),
        parse_as_json: true,
    })
}

pub fn get_radio_group_string_value<Msg, AppEffect>(selector: &Selector) -> Effect<Msg, AppEffect> {
    Effect::Dom(Dom::GetRadioGroupValue {
        selector: selector.clone(),
        parse_as_json: false,
    })
}

pub fn get_files<Msg, AppEffect, Id>(id: Id) -> Effect<Msg, AppEffect>
where
    Id: DomId,
{
    Effect::Dom(Dom::GetFiles {
        element_id: id.to_string(),
    })
}

pub fn get_target_data_string_value<Msg, AppEffect>(name: &str) -> Effect<Msg, AppEffect> {
    Effect::Dom(Dom::GetTargetDataValue {
        name: name.to_string(),
        parse_as_json: false,
    })
}

pub fn get_target_data_json_value<Msg, AppEffect>(name: &str) -> Effect<Msg, AppEffect> {
    Effect::Dom(Dom::GetTargetDataValue {
        name: name.to_string(),
        parse_as_json: true,
    })
}

pub fn window_size<Msg, AppEffect>() -> Effect<Msg, AppEffect> {
    Effect::Dom(Dom::GetWindowSize)
}

pub fn dispatch_window_event<Msg, AppEffect>(event_type: &str) -> Effect<Msg, AppEffect> {
    Effect::Dom(Dom::DispatchEvent {
        event_target: EventTarget::Window,
        event_type: event_type.to_string(),
        bubbles: false,
        cancelable: false,
    })
}

pub fn dispatch_document_event<Msg, AppEffect>(event_type: &str) -> Effect<Msg, AppEffect> {
    Effect::Dom(Dom::DispatchEvent {
        event_target: EventTarget::Document,
        event_type: event_type.to_string(),
        bubbles: false,
        cancelable: false,
    })
}

pub fn dispatch_element_event<Msg, AppEffect>(
    id: impl DomId,
    event_type: &str,
) -> Effect<Msg, AppEffect> {
    Effect::Dom(Dom::DispatchEvent {
        event_target: EventTarget::Element {
            element_id: id.to_string(),
        },
        event_type: event_type.to_string(),
        bubbles: false,
        cancelable: false,
    })
}
