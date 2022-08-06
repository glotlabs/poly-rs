use crate::browser::EventListener;
use crate::browser::Interval;

pub type Subscriptions<Msg> = Vec<Subscription<Msg>>;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "config")]
#[serde(rename_all = "camelCase")]
pub enum Subscription<Msg> {
    None,
    EventListener(EventListener<Msg>),
    Interval(Interval<Msg>),
}

pub fn no_subscription<Msg>() -> Subscription<Msg> {
    Subscription::None
}
