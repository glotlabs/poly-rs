use crate::browser::dom;
use crate::browser::keyboard::Key;
use crate::browser::selector::Selector;
use crate::browser::Button;
use crate::browser::Capture;
use crate::browser::DomId;
use crate::browser::Effect;
use crate::browser::Subscription;
use crate::browser::SubscriptionMsg;
use std::fmt;

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
    ExactSelector {
        selector: Selector,
    },
    ClosestSelector {
        selector: Selector,
    },
    MouseButton {
        button: Button,
    },
    #[serde(rename_all = "camelCase")]
    KeyboardKey {
        key: Key,
        requires_ctrl: bool,
        requires_meta: bool,
    },
}

#[derive(Clone)]
pub enum ModifierKey {
    None,
    Ctrl,
    Meta,
    Multiple(Vec<ModifierKey>),
}

impl ModifierKey {
    pub fn requires_ctrl(&self) -> bool {
        match self {
            ModifierKey::Ctrl => true,
            ModifierKey::Multiple(keys) => keys.iter().any(Self::requires_ctrl),
            _ => false,
        }
    }

    pub fn requires_meta(&self) -> bool {
        match self {
            ModifierKey::Meta => true,
            ModifierKey::Multiple(keys) => keys.iter().any(Self::requires_meta),
            _ => false,
        }
    }
}

impl fmt::Display for ModifierKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ModifierKey::None => write!(f, "no-modifier"),
            ModifierKey::Ctrl => write!(f, "ctrl"),
            ModifierKey::Meta => write!(f, "meta"),
            ModifierKey::Multiple(keys) => {
                let keys: Vec<String> = keys.iter().map(|key| key.to_string()).collect();
                write!(f, "{}", keys.join("-"))
            }
        }
    }
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

pub fn on_click<Id, Msg, AppEffect>(id: Id, msg: Msg) -> Subscription<Msg, AppEffect>
where
    Id: DomId,
{
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

pub fn on_click_closest<Id, Msg, AppEffect>(id: Id, msg: Msg) -> Subscription<Msg, AppEffect>
where
    Id: DomId,
{
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

pub fn on_click_selector<Msg, AppEffect, ToMsg, T>(
    selector: Selector,
    effect: Effect<Msg, AppEffect>,
    to_msg: ToMsg,
) -> Subscription<Msg, AppEffect>
where
    ToMsg: Fn(Capture<T>) -> Msg,
    T: Default,
{
    Subscription::EventListener(EventListener {
        id: format!("on-click-selector-{}", selector),
        listen_target: ListenTarget::Document,
        matchers: vec![EventMatcher::ExactSelector { selector }],
        event_type: EventType::Click,
        msg: SubscriptionMsg::effectful(to_msg, effect),
        propagation: EventPropagation {
            stop_propagation: true,
            prevent_default: true,
        },
    })
}

pub fn on_click_selector_closest<Msg, AppEffect, ToMsg, T>(
    selector: Selector,
    effect: Effect<Msg, AppEffect>,
    to_msg: ToMsg,
) -> Subscription<Msg, AppEffect>
where
    ToMsg: Fn(Capture<T>) -> Msg,
    T: Default,
{
    Subscription::EventListener(EventListener {
        id: format!("selector-closest-{}", selector),
        listen_target: ListenTarget::Document,
        matchers: vec![EventMatcher::ClosestSelector { selector }],
        event_type: EventType::Click,
        msg: SubscriptionMsg::effectful(to_msg, effect),
        propagation: EventPropagation {
            stop_propagation: true,
            prevent_default: true,
        },
    })
}

pub fn on_mouse_down<Id, Msg, AppEffect>(id: Id, msg: Msg) -> Subscription<Msg, AppEffect>
where
    Id: DomId,
{
    Subscription::EventListener(EventListener {
        id: id.to_string(),
        listen_target: ListenTarget::Document,
        matchers: vec![
            EventMatcher::ExactSelector {
                selector: id.selector(),
            },
            EventMatcher::MouseButton {
                button: Button::Main,
            },
        ],
        event_type: EventType::Mousedown,
        msg: SubscriptionMsg::pure(msg),
        propagation: EventPropagation {
            stop_propagation: true,
            prevent_default: true,
        },
    })
}

pub fn on_input<Id, Msg, AppEffect, ToMsg>(id: Id, to_msg: ToMsg) -> Subscription<Msg, AppEffect>
where
    Id: DomId,
    ToMsg: Fn(Capture<String>) -> Msg,
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

pub fn on_change<Id, Msg, AppEffect, ToMsg, T>(
    id: Id,
    to_msg: ToMsg,
) -> Subscription<Msg, AppEffect>
where
    Id: DomId,
    ToMsg: Fn(Capture<T>) -> Msg,
    T: Default,
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

pub fn on_change_string<Id, Msg, AppEffect, ToMsg>(
    id: Id,
    to_msg: ToMsg,
) -> Subscription<Msg, AppEffect>
where
    Id: DomId,
    ToMsg: Fn(Capture<String>) -> Msg,
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

pub fn on_change_selector<Msg, AppEffect, ToMsg, T>(
    selector: Selector,
    to_msg: ToMsg,
    effect: Effect<Msg, AppEffect>,
) -> Subscription<Msg, AppEffect>
where
    ToMsg: Fn(Capture<T>) -> Msg,
    T: Default,
{
    Subscription::EventListener(EventListener {
        id: format!("on-change-selector-{}", selector),
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

pub fn on_radio_change_string<Msg, AppEffect, ToMsg>(
    name: &str,
    to_msg: ToMsg,
) -> Subscription<Msg, AppEffect>
where
    ToMsg: Fn(Capture<String>) -> Msg,
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
    Id: DomId,
    ToMsg: Fn(Capture<String>) -> Msg,
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

pub fn on_submit<Id, Msg, AppEffect>(id: Id, msg: Msg) -> Subscription<Msg, AppEffect>
where
    Id: DomId,
{
    Subscription::EventListener(EventListener {
        id: id.to_string(),
        listen_target: ListenTarget::Document,
        matchers: vec![EventMatcher::ExactSelector {
            selector: id.selector(),
        }],
        event_type: EventType::Submit,
        msg: SubscriptionMsg::pure(msg),
        propagation: EventPropagation {
            stop_propagation: true,
            prevent_default: true,
        },
    })
}

pub fn on_keyup_element<Id, Msg, AppEffect, ToMsg>(
    id: Id,
    to_msg: ToMsg,
) -> Subscription<Msg, AppEffect>
where
    Id: DomId,
    ToMsg: Fn(Capture<String>) -> Msg,
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

pub fn on_keyup<Msg, AppEffect>(key: Key, msg: Msg) -> Subscription<Msg, AppEffect> {
    Subscription::EventListener(EventListener {
        id: format!("keyboard-keyup-{}", key),
        listen_target: ListenTarget::Document,
        event_type: EventType::Keyup,
        matchers: vec![EventMatcher::KeyboardKey {
            key,
            requires_ctrl: false,
            requires_meta: false,
        }],
        msg: SubscriptionMsg::pure(msg),
        propagation: EventPropagation {
            stop_propagation: false,
            prevent_default: false,
        },
    })
}

pub fn on_keydown<Msg, AppEffect>(
    key: Key,
    modifier: ModifierKey,
    msg: Msg,
) -> Subscription<Msg, AppEffect> {
    Subscription::EventListener(EventListener {
        id: format!("keyboard-keydown-{}-{}", key, modifier),
        listen_target: ListenTarget::Document,
        event_type: EventType::Keydown,
        matchers: vec![EventMatcher::KeyboardKey {
            key,
            requires_ctrl: modifier.requires_ctrl(),
            requires_meta: modifier.requires_meta(),
        }],
        msg: SubscriptionMsg::pure(msg),
        propagation: EventPropagation {
            stop_propagation: false,
            prevent_default: false,
        },
    })
}

pub fn on_window_resize<Msg, AppEffect, ToMsg, T>(to_msg: ToMsg) -> Subscription<Msg, AppEffect>
where
    ToMsg: Fn(Capture<T>) -> Msg,
    T: Default,
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
    Mousedown,
    Input,
    Change,
    Submit,
    Keyup,
    Keydown,
    Resize,
}
