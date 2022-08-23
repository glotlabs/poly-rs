use crate::browser::Effect;
use crate::browser::Subscription;
use crate::browser::SubscriptionMsg;
use crate::browser::Value;
use std::time::Duration;

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Interval<Msg, AppEffect> {
    id: String,
    duration: u128,
    msg: SubscriptionMsg<Msg, AppEffect>,
}

pub fn interval<Msg, AppEffect>(
    duration: Duration,
    msg: Msg,
) -> Subscription<Msg, AppEffect> {
    Subscription::Interval(Interval {
        id: format!("interval-{}", duration.as_millis()),
        duration: duration.as_millis(),
        msg: SubscriptionMsg::pure(msg),
    })
}

pub fn interval_effect<Msg, AppEffect, ToMsg>(
    duration: Duration,
    to_msg: ToMsg,
    effect: Effect<Msg, AppEffect>,
) -> Subscription<Msg, AppEffect>
where
    ToMsg: Fn(Value) -> Msg,
{
    Subscription::Interval(Interval {
        id: format!("interval-{}", duration.as_millis()),
        duration: duration.as_millis(),
        msg: SubscriptionMsg::effectful(to_msg, effect),
    })
}
