use crate::browser;
use crate::browser::effectful_msg::effectful_msg;
use crate::browser::Capture;
use crate::browser::Effect;
use crate::browser::Value;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "config")]
#[serde(rename_all = "camelCase")]
pub enum LocalStorage {
    GetItem { key: String },
    SetItem { key: String, value: Value },
}

pub fn get_item<Msg, AppEffect, ToMsg, T>(key: &str, to_msg: ToMsg) -> Effect<Msg, AppEffect>
where
    ToMsg: Fn(Capture<T>) -> Msg,
    T: Default,
{
    let msg = to_msg(Default::default());
    let effect = Effect::LocalStorage(LocalStorage::GetItem {
        key: key.to_string(),
    });

    effectful_msg(msg, effect)
}

pub fn set_item<Msg, AppEffect, V, ToMsg>(
    key: &str,
    value: V,
    to_msg: ToMsg,
) -> Effect<Msg, AppEffect>
where
    V: serde::Serialize,
    ToMsg: Fn(Capture<bool>) -> Msg,
{
    let effect = Effect::LocalStorage(LocalStorage::SetItem {
        key: key.to_string(),
        value: browser::to_value(value),
    });

    let msg = to_msg(Default::default());
    effectful_msg(msg, effect)
}
