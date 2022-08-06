pub mod dom_id;
pub mod effect;
pub mod event_listener;
pub mod interval;
pub mod keyboard;
pub mod queue_strategy;
pub mod selector;
pub mod value;

pub use crate::browser::dom_id::*;
pub use crate::browser::effect::*;
pub use crate::browser::event_listener::*;
pub use crate::browser::interval::*;
pub use crate::browser::keyboard::*;
pub use crate::browser::queue_strategy::*;
pub use crate::browser::selector::*;
pub use crate::browser::value::*;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebounceConfig {
    delay: u32,
    leading: bool,
    trailing: bool,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowSize {
    pub width: u32,
    pub height: u32,
}