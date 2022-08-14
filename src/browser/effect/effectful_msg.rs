use crate::browser::Effect;

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EffectfulMsg<Msg, CustomEffect> {
    pub msg: Msg,
    pub effect: Effect<Msg, CustomEffect>,
}

pub fn effectful_msg<Msg, CustomEffect>(
    msg: Msg,
    effect: Effect<Msg, CustomEffect>,
) -> Effect<Msg, CustomEffect> {
    Effect::EffectfulMsg(Box::new(EffectfulMsg {
        msg: msg,
        effect: effect,
    }))
}
