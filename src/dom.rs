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
pub struct EventConfig {
    pub stop_propagation: bool,
    pub prevent_default: bool,
    pub match_parent_elements: bool,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "config")]
#[serde(rename_all = "camelCase")]
pub enum Event {
    Click(EventConfig),
    Input(EventConfig),
    Change(EventConfig),
    Keyup(KeyboardEventConfig),
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyboardEventConfig {
    key: Key,
    alt_key: bool,
    ctrl_key: bool,
    meta_key: bool,
    shift_key: bool,
    debounce: DebounceConfig,
    event: EventConfig,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Key {
    Any,
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
pub struct EventListener<Msg> {
    pub id: DomId,
    pub selector: Selector,
    pub event: Event,
    pub msg: Msg,
    pub queue_strategy: QueueStrategy,
}

impl<Msg> EventListener<Msg> {
    pub fn match_parent_elements(mut self) -> Self {
        match self.event {
            Event::Click(ref mut event) => {
                event.match_parent_elements = true;
            }
            Event::Input(ref mut event) => {
                event.match_parent_elements = true;
            }
            Event::Change(ref mut event) => {
                event.match_parent_elements = true;
            }
            Event::Keyup(ref _event) => {}
        }

        self
    }
}

pub fn on_click<Msg>(id: &DomId, msg: Msg) -> EventListener<Msg> {
    EventListener {
        id: id.clone(),
        selector: id.selector(),
        event: Event::Click(EventConfig {
            stop_propagation: true,
            prevent_default: true,
            match_parent_elements: false,
        }),
        msg,
        queue_strategy: QueueStrategy::Fifo,
    }
}

pub fn on_input<Msg>(id: &DomId, msg: Msg) -> EventListener<Msg> {
    let selector = id.selector();

    EventListener {
        id: id.clone(),
        selector: selector,
        event: Event::Input(EventConfig {
            stop_propagation: true,
            prevent_default: true,
            match_parent_elements: false,
        }),
        msg: msg,
        queue_strategy: QueueStrategy::DropOlder,
    }
}

pub fn on_change<Msg>(id: &DomId, msg: Msg) -> EventListener<Msg> {
    let selector = id.selector();

    EventListener {
        id: id.clone(),
        selector: selector,
        event: Event::Change(EventConfig {
            stop_propagation: true,
            prevent_default: true,
            match_parent_elements: false,
        }),
        msg: msg,
        queue_strategy: QueueStrategy::DropOlder,
    }
}

pub fn on_keyup<Msg>(id: &DomId, msg: Msg) -> EventListener<Msg> {
    let selector = id.selector();

    EventListener {
        id: id.clone(),
        selector: selector,
        event: Event::Keyup(KeyboardEventConfig {
            key: Key::Any,
            alt_key: false,
            ctrl_key: false,
            meta_key: false,
            shift_key: false,
            debounce: DebounceConfig {
                delay: 150,
                leading: true,
                trailing: true,
            },
            event: EventConfig {
                stop_propagation: true,
                prevent_default: true,
                match_parent_elements: false,
            },
        }),
        msg,
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
