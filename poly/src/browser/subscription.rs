pub mod event_listener;
pub mod interval;

use crate::browser::effect::Effect;
use crate::browser::subscription::event_listener::EventListener;
use crate::browser::subscription::interval::Interval;

#[derive(Clone, serde::Serialize)]
#[serde(tag = "type", content = "config")]
#[serde(rename_all = "camelCase")]
pub enum Subscription<Msg, AppEffect> {
    None,
    EventListener(EventListener<Msg, AppEffect>),
    Interval(Interval<Msg, AppEffect>),
    Batch(Vec<Subscription<Msg, AppEffect>>),
}

impl<Msg, AppEffect> Subscription<Msg, AppEffect> {
    pub fn into_vec(self) -> Vec<Subscription<Msg, AppEffect>> {
        match self {
            Subscription::Batch(subscriptions) => {
                subscriptions.into_iter().flat_map(Self::into_vec).collect()
            }

            _ => vec![self],
        }
    }
}

pub fn none<Msg, AppEffect>() -> Subscription<Msg, AppEffect> {
    Subscription::None
}

pub fn batch<Msg, AppEffect>(
    subscriptions: Vec<Subscription<Msg, AppEffect>>,
) -> Subscription<Msg, AppEffect> {
    Subscription::Batch(subscriptions)
}

#[derive(Clone, serde::Serialize)]
#[serde(tag = "type", content = "config")]
#[serde(rename_all = "camelCase")]
pub enum SubscriptionMsg<Msg, AppEffect> {
    Pure(Msg),
    Effectful {
        msg: Msg,
        effect: Effect<Msg, AppEffect>,
    },
}

impl<Msg, AppEffect> SubscriptionMsg<Msg, AppEffect> {
    pub fn pure(msg: Msg) -> SubscriptionMsg<Msg, AppEffect> {
        SubscriptionMsg::Pure(msg)
    }

    pub fn effectful<ToMsg, T>(
        to_msg: ToMsg,
        effect: Effect<Msg, AppEffect>,
    ) -> SubscriptionMsg<Msg, AppEffect>
    where
        ToMsg: Fn(T) -> Msg,
        T: Default,
    {
        SubscriptionMsg::Effectful {
            msg: to_msg(T::default()),
            effect,
        }
    }
}
