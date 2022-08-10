use crate::browser;
use crate::browser::Effect;
use crate::browser::Value;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "config")]
#[serde(rename_all = "camelCase")]
pub enum LocalStorage<Msg> {
    GetItem { key: String, msg: Msg },
    SetItem { key: String, value: Value },
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetItem<Msg> {
    pub key: String,
    pub msg: Msg,
}

pub fn get_item<Msg, CustomEffect, ToMsg>(key: &str, to_msg: ToMsg) -> Effect<Msg, CustomEffect>
where
    ToMsg: Fn(Value) -> Msg,
{
    Effect::LocalStorage(LocalStorage::GetItem {
        key: key.to_string(),
        msg: to_msg(Value::capture_from_local_storage(key)),
    })
}

pub fn set_item<Msg, CustomEffect, V>(key: &str, value: V) -> Effect<Msg, CustomEffect>
where
    V: serde::Serialize,
{
    Effect::LocalStorage(LocalStorage::SetItem {
        key: key.to_string(),
        value: browser::to_value(value),
    })
}
