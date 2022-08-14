use crate::browser::queue_strategy::QueueStrategy;
use crate::browser::Subscription;
use crate::browser::SubscriptionMsg;

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Interval<Msg, CustomEffect> {
    id: String,
    duration: u64,
    msg: SubscriptionMsg<Msg, CustomEffect>,
    queue_strategy: QueueStrategy,
}

pub fn interval<Msg, CustomEffect>(
    id: &str,
    duration: u64,
    msg: SubscriptionMsg<Msg, CustomEffect>,
) -> Subscription<Msg, CustomEffect> {
    Subscription::Interval(Interval {
        id: format!("{}-{}", id, duration),
        duration,
        msg,
        queue_strategy: QueueStrategy::DropOlder,
    })
}
