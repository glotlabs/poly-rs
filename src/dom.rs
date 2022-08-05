use std::fmt;

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[serde(tag = "type", content = "config")]
#[serde(rename_all = "camelCase")]
pub enum CaptureType {
    #[serde(rename_all = "camelCase")]
    ValueFromElement {
        element_id: DomId,
    },
    WindowSize,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct Value(serde_json::Value);

impl Value {
    pub fn parse<T>(&self) -> Result<T, serde_json::Error>
    where
        T: serde::de::DeserializeOwned,
    {
        serde_json::from_value(self.0.clone())
    }

    pub fn from_id(id: &DomId) -> Value {
        to_value(CaptureType::ValueFromElement {
            element_id: id.clone(),
        })
    }

    pub fn window_size() -> Value {
        to_value(CaptureType::WindowSize)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
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

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Key::Any => write!(f, "any"),
            Key::Escape => write!(f, "escape"),
            Key::Key(key) => write!(f, "{}", key),
        }
    }
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

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowSize {
    pub width: u32,
    pub height: u32,
}

fn to_value<T>(value: T) -> Value
where
    T: serde::Serialize,
{
    match serde_json::to_value(value) {
        Ok(json_value) => Value(json_value),

        Err(err) => Value(serde_json::Value::String(format!(
            "Failed to serialize value: {}",
            err
        ))),
    }
}
