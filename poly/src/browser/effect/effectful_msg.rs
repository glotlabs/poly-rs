use crate::browser::Effect;

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EffectfulMsg<Msg, AppEffect> {
    pub msg: Msg,
    pub effect: Effect<Msg, AppEffect>,
}

pub fn effectful_msg<Msg, AppEffect>(
    msg: Msg,
    effect: Effect<Msg, AppEffect>,
) -> Effect<Msg, AppEffect> {
    Effect::EffectfulMsg(Box::new(EffectfulMsg { msg, effect }))
}
