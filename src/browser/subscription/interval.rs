use crate::browser::queue_strategy::QueueStrategy;
use crate::browser::Subscription;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Interval<Msg> {
    id: String,
    duration: u64,
    msg: Msg,
    queue_strategy: QueueStrategy,
}

pub fn interval<Msg>(id: &str, duration: u64, msg: Msg) -> Subscription<Msg> {
    Subscription::Interval(Interval {
        id: format!("{}-{}", id, duration),
        duration,
        msg,
        queue_strategy: QueueStrategy::DropOlder,
    })
}
