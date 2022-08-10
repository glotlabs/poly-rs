pub mod local_storage;
pub mod navigation;

use crate::browser::effect::local_storage::*;
use crate::browser::effect::navigation::*;

pub type Effects<Msg, CustomEffect> = Vec<Effect<Msg, CustomEffect>>;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "config")]
#[serde(rename_all = "camelCase")]
pub enum Effect<Msg, CustomEffect> {
    None,
    Navigation(Navigation),
    Custom(CustomEffect),
    LocalStorage(LocalStorage<Msg>),
}

pub fn none<Msg, CustomEffect>() -> Effect<Msg, CustomEffect> {
    Effect::None
}

pub fn no_effects<Msg, CustomEffect>() -> Result<Effects<Msg, CustomEffect>, String> {
    Ok(vec![none()])
}

pub fn custom_effect<Msg, CustomEffect>(effect: CustomEffect) -> Effect<Msg, CustomEffect> {
    Effect::Custom(effect)
}
