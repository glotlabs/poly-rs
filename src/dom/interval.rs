use crate::dom::dom_id::DomId;
use crate::dom::queue_strategy::QueueStrategy;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Interval<Msg> {
    id: DomId,
    duration: u64,
    msg: Msg,
    queue_strategy: QueueStrategy,
}

pub fn interval<Msg>(id: DomId, duration: u64, msg: Msg) -> Interval<Msg> {
    Interval {
        id,
        duration,
        msg,
        queue_strategy: QueueStrategy::DropOlder,
    }
}
