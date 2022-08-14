use crate::browser::Effect;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "config")]
#[serde(rename_all = "camelCase")]
pub enum Time {
    CurrentTime,
}

pub fn current_time<Msg, CustomEffect>() -> Effect<Msg, CustomEffect> {
    Effect::Time(Time::CurrentTime)
}
