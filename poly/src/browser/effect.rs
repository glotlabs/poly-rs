pub mod browser;
pub mod clipboard;
pub mod console;
pub mod dom;
pub mod effectful_msg;
pub mod local_storage;
pub mod navigation;
pub mod time;

use crate::browser::dom::Dom;
use crate::browser::effect::browser::*;
use crate::browser::effect::clipboard::*;
use crate::browser::effect::console::Console;
use crate::browser::effect::local_storage::*;
use crate::browser::effect::navigation::*;
use crate::browser::effectful_msg::EffectfulMsg;
use crate::browser::time::Time;

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
    Clipboard(Clipboard),
    Browser(Browser),
    App(AppEffect),
    Batch(Vec<Effect<Msg, AppEffect>>),
}

impl<Msg, AppEffect> Effect<Msg, AppEffect> {
    pub fn into_vec(self) -> Vec<Effect<Msg, AppEffect>> {
        match self {
            Effect::Batch(effects) => effects.into_iter().map(Self::into_vec).flatten().collect(),

            _ => vec![self],
        }
    }
}

pub fn none<Msg, AppEffect>() -> Effect<Msg, AppEffect> {
    Effect::None
}

pub fn batch<Msg, AppEffect>(effects: Vec<Effect<Msg, AppEffect>>) -> Effect<Msg, AppEffect> {
    Effect::Batch(effects)
}

pub fn app_effect<Msg, AppEffect>(effect: AppEffect) -> Effect<Msg, AppEffect> {
    Effect::App(effect)
}
