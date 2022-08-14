pub mod dom;
pub mod effectful_msg;
pub mod local_storage;
pub mod navigation;
pub mod time;

use crate::browser::dom::Dom;
use crate::browser::effect::local_storage::*;
use crate::browser::effect::navigation::*;
use crate::browser::effectful_msg::EffectfulMsg;
use crate::browser::time::Time;

pub type Effects<Msg, CustomEffect> = Vec<Effect<Msg, CustomEffect>>;

#[derive(Clone, serde::Serialize)]
#[serde(tag = "type", content = "config")]
#[serde(rename_all = "camelCase")]
pub enum Effect<Msg, CustomEffect> {
    None,
    EffectfulMsg(Box<EffectfulMsg<Msg, CustomEffect>>),
    Dom(Dom),
    Time(Time),
    Navigation(Navigation),
    LocalStorage(LocalStorage),
    Custom(CustomEffect),
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
