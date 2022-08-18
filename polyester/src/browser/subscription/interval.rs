use crate::browser::Effect;
use crate::browser::Subscription;
use crate::browser::SubscriptionMsg;
use crate::browser::Value;
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
    msg: Msg,
) -> Subscription<Msg, CustomEffect> {
    Subscription::Interval(Interval {
        id: format!("interval-{}", duration.as_millis()),
        duration: duration.as_millis(),
        msg: SubscriptionMsg::pure(msg),
    })
}

pub fn interval_effect<Msg, CustomEffect, ToMsg>(
    duration: Duration,
    to_msg: ToMsg,
    effect: Effect<Msg, CustomEffect>,
) -> Subscription<Msg, CustomEffect>
where
    ToMsg: Fn(Value) -> Msg,
{
    Subscription::Interval(Interval {
        id: format!("interval-{}", duration.as_millis()),
        duration: duration.as_millis(),
        msg: SubscriptionMsg::effectful(to_msg, effect),
    })
}
