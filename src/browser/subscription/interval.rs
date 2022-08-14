use crate::browser::Subscription;
use crate::browser::SubscriptionMsg;
use std::time::Duration;

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Interval<Msg, CustomEffect> {
    id: String,
    duration: u128,
    msg: SubscriptionMsg<Msg, CustomEffect>,
}

pub fn interval<Msg, CustomEffect>(
    duration: Duration,
    msg: SubscriptionMsg<Msg, CustomEffect>,
) -> Subscription<Msg, CustomEffect> {
    Subscription::Interval(Interval {
        id: format!("interval-{}", duration.as_millis()),
        duration: duration.as_millis(),
        msg,
    })
}
