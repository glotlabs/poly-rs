use crate::browser::Capture;
use crate::browser::Effect;
use crate::browser::Subscription;
use crate::browser::SubscriptionMsg;
use std::time::Duration;

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Interval<Msg, AppEffect> {
    id: String,
    duration: u128,
    msg: SubscriptionMsg<Msg, AppEffect>,
}

pub fn interval<Msg, AppEffect>(duration: Duration, msg: Msg) -> Subscription<Msg, AppEffect> {
    Subscription::Interval(Interval {
        id: format!("interval-{}", duration.as_millis()),
        duration: duration.as_millis(),
        msg: SubscriptionMsg::pure(msg),
    })
}

pub fn interval_effect<Msg, AppEffect, ToMsg, T>(
    duration: Duration,
    to_msg: ToMsg,
    effect: Effect<Msg, AppEffect>,
) -> Subscription<Msg, AppEffect>
where
    ToMsg: Fn(Capture<T>) -> Msg,
    T: Default,
{
    Subscription::Interval(Interval {
        id: format!("interval-{}", duration.as_millis()),
        duration: duration.as_millis(),
        msg: SubscriptionMsg::effectful(to_msg, effect),
    })
}
