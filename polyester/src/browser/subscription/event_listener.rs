use crate::browser::dom;
use crate::browser::dom_id::DomId;
use crate::browser::keyboard::Key;
use crate::browser::keyboard::KeyCombo;
use crate::browser::selector::Selector;
use crate::browser::Subscription;
use crate::browser::SubscriptionMsg;
use crate::browser::Value;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ListenTarget {
    Window,
    Document,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "config")]
#[serde(rename_all = "camelCase")]
pub enum EventMatcher {
    ExactSelector { selector: Selector },
    ClosestSelector { selector: Selector },
    KeyboardKey { key: Key },
    KeyCombo { combo: KeyCombo },
}

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventListener<Msg, CustomEffect> {
    pub id: String,
    pub listen_target: ListenTarget,
    pub event_type: EventType,
    pub matchers: Vec<EventMatcher>,
    pub msg: SubscriptionMsg<Msg, CustomEffect>,
    pub propagation: EventPropagation,
}

pub fn on_click<Msg, CustomEffect>(id: &DomId, msg: Msg) -> Subscription<Msg, CustomEffect> {
    Subscription::EventListener(EventListener {
        id: id.to_string(),
        listen_target: ListenTarget::Document,
        matchers: vec![EventMatcher::ExactSelector {
            selector: id.selector(),
        }],
        event_type: EventType::Click,
        msg: SubscriptionMsg::pure(msg),
        propagation: EventPropagation {
            stop_propagation: true,
            prevent_default: true,
        },
    })
}

pub fn on_click_closest<Msg, CustomEffect>(
    id: &DomId,
    msg: Msg,
) -> Subscription<Msg, CustomEffect> {
    Subscription::EventListener(EventListener {
        id: id.to_string(),
        listen_target: ListenTarget::Document,
        matchers: vec![EventMatcher::ClosestSelector {
            selector: id.selector(),
        }],
        event_type: EventType::Click,
        msg: SubscriptionMsg::pure(msg),
        propagation: EventPropagation {
            stop_propagation: true,
            prevent_default: true,
        },
    })
}

pub fn on_input<Msg, CustomEffect, ToMsg>(
    id: &DomId,
    to_msg: ToMsg,
) -> Subscription<Msg, CustomEffect>
where
    ToMsg: Fn(String) -> Msg,
{
    Subscription::EventListener(EventListener {
        id: id.to_string(),
        listen_target: ListenTarget::Document,
        matchers: vec![EventMatcher::ExactSelector {
            selector: id.selector(),
        }],
        event_type: EventType::Input,
        msg: SubscriptionMsg::effectful(to_msg, dom::get_element_string_value(id)),
        propagation: EventPropagation {
            stop_propagation: true,
            prevent_default: true,
        },
    })
}

pub fn on_change<Msg, CustomEffect, ToMsg>(
    id: &DomId,
    to_msg: ToMsg,
) -> Subscription<Msg, CustomEffect>
where
    ToMsg: Fn(Value) -> Msg,
{
    Subscription::EventListener(EventListener {
        id: id.to_string(),
        listen_target: ListenTarget::Document,
        matchers: vec![EventMatcher::ExactSelector {
            selector: id.selector(),
        }],
        event_type: EventType::Change,
        msg: SubscriptionMsg::effectful(to_msg, dom::get_element_json_value(id)),
        propagation: EventPropagation {
            stop_propagation: true,
            prevent_default: true,
        },
    })
}

pub fn on_change_string<Msg, CustomEffect, ToMsg>(
    id: &DomId,
    to_msg: ToMsg,
) -> Subscription<Msg, CustomEffect>
where
    ToMsg: Fn(String) -> Msg,
{
    Subscription::EventListener(EventListener {
        id: id.to_string(),
        listen_target: ListenTarget::Document,
        matchers: vec![EventMatcher::ExactSelector {
            selector: id.selector(),
        }],
        event_type: EventType::Change,
        msg: SubscriptionMsg::effectful(to_msg, dom::get_element_string_value(id)),
        propagation: EventPropagation {
            stop_propagation: true,
            prevent_default: true,
        },
    })
}

pub fn on_keyup_element<Msg, CustomEffect, ToMsg>(
    id: &DomId,
    to_msg: ToMsg,
) -> Subscription<Msg, CustomEffect>
where
    ToMsg: Fn(String) -> Msg,
{
    Subscription::EventListener(EventListener {
        id: id.to_string(),
        listen_target: ListenTarget::Document,
        event_type: EventType::Keyup,
        matchers: vec![EventMatcher::ExactSelector {
            selector: id.selector(),
        }],
        msg: SubscriptionMsg::effectful(to_msg, dom::get_element_string_value(id)),
        propagation: EventPropagation {
            stop_propagation: true,
            prevent_default: true,
        },
    })
}

pub fn on_keyup_document<Msg, CustomEffect>(key: Key, msg: Msg) -> Subscription<Msg, CustomEffect> {
    Subscription::EventListener(EventListener {
        id: format!("keyboard-key-{}", key),
        listen_target: ListenTarget::Document,
        event_type: EventType::Keyup,
        matchers: vec![EventMatcher::KeyboardKey { key: key }],
        msg: SubscriptionMsg::pure(msg),
        propagation: EventPropagation {
            stop_propagation: false,
            prevent_default: false,
        },
    })
}

pub fn on_window_resize<Msg, CustomEffect, ToMsg>(to_msg: ToMsg) -> Subscription<Msg, CustomEffect>
where
    ToMsg: Fn(Value) -> Msg,
{
    Subscription::EventListener(EventListener {
        id: "window-resize".to_string(),
        listen_target: ListenTarget::Window,
        event_type: EventType::Resize,
        matchers: vec![],
        msg: SubscriptionMsg::effectful(to_msg, dom::window_size()),
        propagation: EventPropagation {
            stop_propagation: false,
            prevent_default: false,
        },
    })
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventPropagation {
    pub stop_propagation: bool,
    pub prevent_default: bool,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EventType {
    Click,
    Input,
    Change,
    Keyup,
    Resize,
}
