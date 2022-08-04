use std::fmt;
use std::str::FromStr;

pub const CAPTURE_VALUE: &'static str = ":VALUE";
pub const VALUE_FROM_ID: &'static str = "VALUE_FROM_ID";

#[derive(Clone, Default, serde::Deserialize)]
pub struct Value(String);

impl Value {
    pub fn parse<T>(&self) -> Result<T, T::Err>
    where
        T: FromStr,
        T::Err: fmt::Display,
    {
        self.0.parse::<T>()
    }

    pub fn from_id(id: &DomId) -> Value {
        Value(format!("{}:{}", VALUE_FROM_ID, id))
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl serde::Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

#[derive(Clone, Default, serde::Deserialize)]
pub struct JsValue {
    pub kind: String,
    pub value: serde_json::Value,
}

impl JsValue {
    pub fn from_value<T>(self) -> Result<T, serde_json::Error>
    where
        T: serde::de::DeserializeOwned,
    {
        serde_json::from_value(self.value)
    }
}

impl serde::Serialize for JsValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(CAPTURE_VALUE)
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Logic<Msg> {
    pub event_listeners: Vec<EventListener<Msg>>,
    pub intervals: Vec<Interval<Msg>>,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Interval<Msg> {
    id: DomId,
    duration: u64,
    msg: Msg,
    queue_strategy: QueueStrategy,
}

pub fn interval<Msg>(id: DomId, duration: u64, msg: Msg) -> Interval<Msg> {
    Interval {
        id,
        duration,
        msg,
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
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyboardCombo {
    key: Key,
    alt_key: bool,
    ctrl_key: bool,
    meta_key: bool,
    shift_key: bool,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Key {
    Any,
    Escape,
    Key(String),
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebounceConfig {
    delay: u32,
    leading: bool,
    trailing: bool,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum QueueStrategy {
    Fifo,
    DropOlder,
}

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
    KeyboardCombo { combo: KeyboardCombo },
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventListener<Msg> {
    pub id: String,
    pub listen_target: ListenTarget,
    pub listen_event: EventType,
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
        listen_event: EventType::Click,
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
        listen_event: EventType::Click,
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
        listen_event: EventType::Input,
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
        listen_event: EventType::Change,
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
        listen_event: EventType::Keyup,
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

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Selector(String);

impl Selector {
    pub fn new(selector: &str) -> Selector {
        Selector(selector.to_string())
    }
}

impl fmt::Display for Selector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct DomId(String);

impl DomId {
    pub fn new(id: &str) -> DomId {
        DomId(id.into())
    }

    pub fn selector(&self) -> Selector {
        Selector::new(&format!("#{}", self))
    }
}

impl From<&str> for DomId {
    fn from(s: &str) -> Self {
        DomId(s.into())
    }
}

impl fmt::Display for DomId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
