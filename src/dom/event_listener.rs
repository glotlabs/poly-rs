use crate::dom::dom_id::DomId;
use crate::dom::keyboard::Key;
use crate::dom::keyboard::KeyCombo;
use crate::dom::queue_strategy::QueueStrategy;
use crate::dom::selector::Selector;
use crate::dom::value::Value;

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

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventListener<Msg> {
    pub id: String,
    pub listen_target: ListenTarget,
    pub event_type: EventType,
    pub matchers: Vec<EventMatcher>,
    pub msg: Msg,
    pub propagation: EventPropagation,
    pub queue_strategy: QueueStrategy,
}

pub fn on_click<Msg>(id: &DomId, msg: Msg) -> EventListener<Msg> {
    EventListener {
        id: id.to_string(),
        listen_target: ListenTarget::Document,
        matchers: vec![EventMatcher::ExactSelector {
            selector: id.selector(),
        }],
        event_type: EventType::Click,
        msg,
        propagation: EventPropagation {
            stop_propagation: true,
            prevent_default: true,
        },
        queue_strategy: QueueStrategy::Fifo,
    }
}

pub fn on_click_closest<Msg>(id: &DomId, msg: Msg) -> EventListener<Msg> {
    EventListener {
        id: id.to_string(),
        listen_target: ListenTarget::Document,
        matchers: vec![EventMatcher::ClosestSelector {
            selector: id.selector(),
        }],
        event_type: EventType::Click,
        msg,
        propagation: EventPropagation {
            stop_propagation: true,
            prevent_default: true,
        },
        queue_strategy: QueueStrategy::Fifo,
    }
}

pub fn on_input<Msg>(id: &DomId, msg: Msg) -> EventListener<Msg> {
    EventListener {
        id: id.to_string(),
        listen_target: ListenTarget::Document,
        matchers: vec![EventMatcher::ExactSelector {
            selector: id.selector(),
        }],
        event_type: EventType::Input,
        msg: msg,
        propagation: EventPropagation {
            stop_propagation: true,
            prevent_default: true,
        },
        queue_strategy: QueueStrategy::DropOlder,
    }
}

pub fn on_change<Msg>(id: &DomId, msg: Msg) -> EventListener<Msg> {
    EventListener {
        id: id.to_string(),
        listen_target: ListenTarget::Document,
        matchers: vec![EventMatcher::ExactSelector {
            selector: id.selector(),
        }],
        event_type: EventType::Change,
        msg: msg,
        propagation: EventPropagation {
            stop_propagation: true,
            prevent_default: true,
        },
        queue_strategy: QueueStrategy::DropOlder,
    }
}

pub fn on_keyup<Msg>(id: &DomId, msg: Msg) -> EventListener<Msg> {
    EventListener {
        id: id.to_string(),
        listen_target: ListenTarget::Document,
        event_type: EventType::Keyup,
        matchers: vec![EventMatcher::ExactSelector {
            selector: id.selector(),
        }],
        msg,
        propagation: EventPropagation {
            stop_propagation: true,
            prevent_default: true,
        },
        queue_strategy: QueueStrategy::DropOlder,
    }
}

pub fn on_keyup_global<Msg>(key: Key, msg: Msg) -> EventListener<Msg> {
    EventListener {
        id: format!("keyboard-key-{}", key),
        listen_target: ListenTarget::Document,
        event_type: EventType::Keyup,
        matchers: vec![EventMatcher::KeyboardKey { key: key }],
        msg,
        propagation: EventPropagation {
            stop_propagation: false,
            prevent_default: false,
        },
        queue_strategy: QueueStrategy::DropOlder,
    }
}

pub fn on_window_resize<Msg, ToMsg>(to_msg: ToMsg) -> EventListener<Msg>
where
    ToMsg: FnOnce(Value) -> Msg,
{
    EventListener {
        id: "window-resize".to_string(),
        listen_target: ListenTarget::Window,
        event_type: EventType::Resize,
        matchers: vec![],
        msg: to_msg(Value::capture_window_size()),
        propagation: EventPropagation {
            stop_propagation: false,
            prevent_default: false,
        },
        queue_strategy: QueueStrategy::DropOlder,
    }
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
