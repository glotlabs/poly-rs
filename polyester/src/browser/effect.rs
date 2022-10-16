pub mod console;
pub mod dom;
pub mod effectful_msg;
pub mod local_storage;
pub mod navigation;
pub mod time;

use crate::browser::dom::Dom;
use crate::browser::effect::console::Console;
use crate::browser::effect::local_storage::*;
use crate::browser::effect::navigation::*;
use crate::browser::effectful_msg::EffectfulMsg;
use crate::browser::time::Time;

pub type Effects<Msg, AppEffect> = Vec<Effect<Msg, AppEffect>>;

#[derive(Clone, serde::Serialize)]
#[serde(tag = "type", content = "config")]
#[serde(rename_all = "camelCase")]
pub enum Effect<Msg, AppEffect> {
    None,
    EffectfulMsg(Box<EffectfulMsg<Msg, AppEffect>>),
    Dom(Dom),
    Time(Time),
    Navigation(Navigation),
    LocalStorage(LocalStorage),
    Console(Console),
    App(AppEffect),
}

pub fn none<Msg, AppEffect>() -> Effect<Msg, AppEffect> {
    Effect::None
}

pub fn app_effect<Msg, AppEffect>(effect: AppEffect) -> Effect<Msg, AppEffect> {
    Effect::App(effect)
}
