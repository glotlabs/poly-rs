use crate::browser::effectful_msg::effectful_msg;
use crate::browser::Effect;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "config")]
#[serde(rename_all = "camelCase")]
pub enum Browser {
    #[serde(rename_all = "camelCase")]
    SetTimeout { duration: u64 },
}

pub fn set_timeout<Msg, AppEffect>(duration: u64, msg: Msg) -> Effect<Msg, AppEffect> {
    let effect = Effect::Browser(Browser::SetTimeout { duration });

    effectful_msg(msg, effect)
}
