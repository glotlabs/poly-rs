pub mod event_listener;
pub mod interval;

use crate::browser::subscription::event_listener::*;
use crate::browser::subscription::interval::*;
use crate::browser::Effect;

pub type Subscriptions<Msg, CustomEffect> = Vec<Subscription<Msg, CustomEffect>>;

#[derive(Clone, serde::Serialize)]
#[serde(tag = "type", content = "config")]
#[serde(rename_all = "camelCase")]
pub enum Subscription<Msg, CustomEffect> {
    None,
    EventListener(EventListener<Msg, CustomEffect>),
    Interval(Interval<Msg, CustomEffect>),
}

pub fn no_subscription<Msg, CustomEffect>() -> Subscription<Msg, CustomEffect> {
    Subscription::None
}

#[derive(Clone, serde::Serialize)]
#[serde(tag = "type", content = "config")]
#[serde(rename_all = "camelCase")]
pub enum SubscriptionMsg<Msg, CustomEffect> {
    Pure(Msg),
    Effectful {
        msg: Msg,
        effect: Effect<Msg, CustomEffect>,
    },
}

impl<Msg, CustomEffect> SubscriptionMsg<Msg, CustomEffect> {
    pub fn pure(msg: Msg) -> SubscriptionMsg<Msg, CustomEffect> {
        SubscriptionMsg::Pure(msg)
    }

    pub fn effectful<ToMsg, T>(
        to_msg: ToMsg,
        effect: Effect<Msg, CustomEffect>,
    ) -> SubscriptionMsg<Msg, CustomEffect>
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
