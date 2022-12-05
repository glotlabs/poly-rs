pub mod event_listener;
pub mod interval;

use crate::browser::subscription::event_listener::*;
use crate::browser::subscription::interval::*;
use crate::browser::Effect;

pub type Subscriptions<Msg, AppEffect> = Vec<Subscription<Msg, AppEffect>>;

#[derive(Clone, serde::Serialize)]
#[serde(tag = "type", content = "config")]
#[serde(rename_all = "camelCase")]
pub enum Subscription<Msg, AppEffect> {
    None,
    EventListener(EventListener<Msg, AppEffect>),
    Interval(Interval<Msg, AppEffect>),
}

pub fn no_subscription<Msg, AppEffect>() -> Subscription<Msg, AppEffect> {
    Subscription::None
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
