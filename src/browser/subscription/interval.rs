use crate::browser::Subscription;
use crate::browser::SubscriptionMsg;

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Interval<Msg, CustomEffect> {
    id: String,
    duration: u64,
    msg: SubscriptionMsg<Msg, CustomEffect>,
}

pub fn interval<Msg, CustomEffect>(
    duration: u64,
    msg: SubscriptionMsg<Msg, CustomEffect>,
) -> Subscription<Msg, CustomEffect> {
    Subscription::Interval(Interval {
        id: format!("interval-{}", duration),
        duration,
        msg,
    })
}
