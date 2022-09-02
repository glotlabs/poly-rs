use crate::browser::dom;
use crate::browser::keyboard::Key;
use crate::browser::keyboard::KeyCombo;
use crate::browser::selector::Selector;
use crate::browser::Subscription;
use crate::browser::SubscriptionMsg;
use crate::browser::ToDomId;
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
pub struct EventListener<Msg, AppEffect> {
    pub id: String,
    pub listen_target: ListenTarget,
    pub event_type: EventType,
    pub matchers: Vec<EventMatcher>,
    pub msg: SubscriptionMsg<Msg, AppEffect>,
    pub propagation: EventPropagation,
}

pub fn on_click<Id, Msg, AppEffect>(id: &Id, msg: Msg) -> Subscription<Msg, AppEffect>
where
    Id: ToDomId,
{
    let dom_id = id.to_dom_id();

    Subscription::EventListener(EventListener {
        id: dom_id.to_string(),
        listen_target: ListenTarget::Document,
        matchers: vec![EventMatcher::ExactSelector {
            selector: dom_id.selector(),
        }],
        event_type: EventType::Click,
        msg: SubscriptionMsg::pure(msg),
        propagation: EventPropagation {
            stop_propagation: true,
            prevent_default: true,
        },
    })
}

pub fn on_click_closest<Id, Msg, AppEffect>(id: &Id, msg: Msg) -> Subscription<Msg, AppEffect>
where
    Id: ToDomId,
{
    let dom_id = id.to_dom_id();

    Subscription::EventListener(EventListener {
        id: dom_id.to_string(),
        listen_target: ListenTarget::Document,
        matchers: vec![EventMatcher::ClosestSelector {
            selector: dom_id.selector(),
        }],
        event_type: EventType::Click,
        msg: SubscriptionMsg::pure(msg),
        propagation: EventPropagation {
            stop_propagation: true,
            prevent_default: true,
        },
    })
}

pub fn on_input<Id, Msg, AppEffect, ToMsg>(id: &Id, to_msg: ToMsg) -> Subscription<Msg, AppEffect>
where
    Id: ToDomId,
    ToMsg: Fn(String) -> Msg,
{
    let dom_id = id.to_dom_id();

    Subscription::EventListener(EventListener {
        id: dom_id.to_string(),
        listen_target: ListenTarget::Document,
        matchers: vec![EventMatcher::ExactSelector {
            selector: dom_id.selector(),
        }],
        event_type: EventType::Input,
        msg: SubscriptionMsg::effectful(to_msg, dom::get_element_string_value(&dom_id)),
        propagation: EventPropagation {
            stop_propagation: true,
            prevent_default: true,
        },
    })
}

pub fn on_change<Id, Msg, AppEffect, ToMsg>(id: &Id, to_msg: ToMsg) -> Subscription<Msg, AppEffect>
where
    Id: ToDomId,
    ToMsg: Fn(Value) -> Msg,
{
    let dom_id = id.to_dom_id();

    Subscription::EventListener(EventListener {
        id: dom_id.to_string(),
        listen_target: ListenTarget::Document,
        matchers: vec![EventMatcher::ExactSelector {
            selector: dom_id.selector(),
        }],
        event_type: EventType::Change,
        msg: SubscriptionMsg::effectful(to_msg, dom::get_element_json_value(&dom_id)),
        propagation: EventPropagation {
            stop_propagation: true,
            prevent_default: true,
        },
    })
}

pub fn on_change_string<Id, Msg, AppEffect, ToMsg>(
    id: &Id,
    to_msg: ToMsg,
) -> Subscription<Msg, AppEffect>
where
    Id: ToDomId,
    ToMsg: Fn(String) -> Msg,
{
    let dom_id = id.to_dom_id();

    Subscription::EventListener(EventListener {
        id: dom_id.to_string(),
        listen_target: ListenTarget::Document,
        matchers: vec![EventMatcher::ExactSelector {
            selector: dom_id.selector(),
        }],
        event_type: EventType::Change,
        msg: SubscriptionMsg::effectful(to_msg, dom::get_element_string_value(&dom_id)),
        propagation: EventPropagation {
            stop_propagation: true,
            prevent_default: true,
        },
    })
}

pub fn on_radio_change_string<Msg, AppEffect, ToMsg>(
    name: &str,
    to_msg: ToMsg,
) -> Subscription<Msg, AppEffect>
where
    ToMsg: Fn(String) -> Msg,
{
    let selector = Selector::radio_group(name);
    let effect = dom::get_radio_group_string_value(&selector);

    Subscription::EventListener(EventListener {
        id: format!("radio-{}", name),
        listen_target: ListenTarget::Document,
        matchers: vec![EventMatcher::ExactSelector { selector }],
        event_type: EventType::Change,
        msg: SubscriptionMsg::effectful(to_msg, effect),
        propagation: EventPropagation {
            stop_propagation: true,
            prevent_default: true,
        },
    })
}

pub fn on_radio_change_json<Id, Msg, AppEffect, ToMsg>(
    name: &str,
    to_msg: ToMsg,
) -> Subscription<Msg, AppEffect>
where
    Id: ToDomId,
    ToMsg: Fn(String) -> Msg,
{
    let selector = Selector::radio_group(name);
    let effect = dom::get_radio_group_json_value(&selector);

    Subscription::EventListener(EventListener {
        id: format!("radio-{}", name),
        listen_target: ListenTarget::Document,
        matchers: vec![EventMatcher::ExactSelector { selector }],
        event_type: EventType::Change,
        msg: SubscriptionMsg::effectful(to_msg, effect),
        propagation: EventPropagation {
            stop_propagation: true,
            prevent_default: true,
        },
    })
}

pub fn on_keyup_element<Id, Msg, AppEffect, ToMsg>(
    id: &Id,
    to_msg: ToMsg,
) -> Subscription<Msg, AppEffect>
where
    Id: ToDomId,
    ToMsg: Fn(String) -> Msg,
{
    let dom_id = id.to_dom_id();

    Subscription::EventListener(EventListener {
        id: dom_id.to_string(),
        listen_target: ListenTarget::Document,
        event_type: EventType::Keyup,
        matchers: vec![EventMatcher::ExactSelector {
            selector: dom_id.selector(),
        }],
        msg: SubscriptionMsg::effectful(to_msg, dom::get_element_string_value(&dom_id)),
        propagation: EventPropagation {
            stop_propagation: true,
            prevent_default: true,
        },
    })
}

pub fn on_keyup_document<Msg, AppEffect>(key: Key, msg: Msg) -> Subscription<Msg, AppEffect> {
    Subscription::EventListener(EventListener {
        id: format!("keyboard-key-{}", key),
        listen_target: ListenTarget::Document,
        event_type: EventType::Keyup,
        matchers: vec![EventMatcher::KeyboardKey { key }],
        msg: SubscriptionMsg::pure(msg),
        propagation: EventPropagation {
            stop_propagation: false,
            prevent_default: false,
        },
    })
}

pub fn on_window_resize<Msg, AppEffect, ToMsg>(to_msg: ToMsg) -> Subscription<Msg, AppEffect>
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
