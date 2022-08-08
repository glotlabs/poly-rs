use crate::browser::navigation::Navigation;

pub type Effects<Msg> = Vec<Effect<Msg>>;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "config")]
#[serde(rename_all = "camelCase")]
pub enum Effect<Msg> {
    None,
    Navigation(Navigation),
    Dummy(Msg),
}

pub fn no_effect<Msg>() -> Effect<Msg> {
    Effect::None
}
