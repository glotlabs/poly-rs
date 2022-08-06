use crate::browser::EventListener;
use crate::browser::Interval;

pub type Effects<Msg> = Vec<Effect<Msg>>;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "config")]
#[serde(rename_all = "camelCase")]
pub enum Effect<Msg> {
    None,
    EventListener(EventListener<Msg>),
    Interval(Interval<Msg>),
}

pub fn no_effect<Msg>() -> Effect<Msg> {
    Effect::None
}
