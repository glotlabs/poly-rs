pub mod dom_id;
pub mod effect;
pub mod event_listener;
pub mod interval;
pub mod keyboard;
pub mod queue_strategy;
pub mod selector;
pub mod value;

pub use crate::dom::dom_id::*;
pub use crate::dom::effect::*;
pub use crate::dom::event_listener::*;
pub use crate::dom::interval::*;
pub use crate::dom::keyboard::*;
pub use crate::dom::queue_strategy::*;
pub use crate::dom::selector::*;
pub use crate::dom::value::*;

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
