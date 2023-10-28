use crate::page::Effects;
use crate::page::Page;
use serde::Serialize;
use wasm_bindgen::prelude::*;

const JSON_SERIALIZER: serde_wasm_bindgen::Serializer =
    serde_wasm_bindgen::Serializer::json_compatible();

pub fn init<P, Model, Msg, AppEffect, Markup>(page: &P) -> Result<JsValue, JsValue>
where
    P: Page<Model, Msg, AppEffect, Markup>,
    Model: serde::Serialize,
    Msg: serde::Serialize,
    AppEffect: serde::Serialize,
{
    let (model, effects) = page.init()?;
    encode_model_and_effects(&ModelAndEffects { model, effects })
}

pub fn view<P, Model, Msg, AppEffect, Markup>(
    page: &P,
    js_model: &JsValue,
) -> Result<String, JsValue>
where
    P: Page<Model, Msg, AppEffect, Markup>,
    Model: serde::de::DeserializeOwned,
{
    let model = decode_model(js_model)?;
    let markup = page.view(&model);

    Ok(page.render_page(markup))
}

pub fn view_body<P, Model, Msg, AppEffect, Markup>(
    page: &P,
    js_model: &JsValue,
) -> Result<String, JsValue>
where
    P: Page<Model, Msg, AppEffect, Markup>,
    Model: serde::de::DeserializeOwned,
{
    let model = decode_model(js_model)?;
    let markup = page.view(&model);

    Ok(page.render(markup.body))
}

pub fn get_subscriptions<P, Model, Msg, AppEffect, Markup>(
    page: &P,
    js_model: &JsValue,
) -> Result<JsValue, JsValue>
where
    P: Page<Model, Msg, AppEffect, Markup>,
    Model: serde::de::DeserializeOwned,
    Msg: serde::Serialize,
    AppEffect: serde::Serialize,
{
    let model = decode_model(js_model)?;
    let subscriptions = page.subscriptions(&model);
    encode_subscriptions(&subscriptions)
}

pub fn update<P, Model, Msg, AppEffect, Markup>(
    page: &P,
    js_msg: &JsValue,
    js_model: &JsValue,
) -> Result<JsValue, JsValue>
where
    P: Page<Model, Msg, AppEffect, Markup>,
    Msg: serde::Serialize,
    Msg: serde::de::DeserializeOwned,
    Model: serde::de::DeserializeOwned,
    Model: serde::Serialize,
    AppEffect: serde::Serialize,
{
    let msg = decode_msg(js_msg)?;
    let mut model = decode_model(js_model)?;
    let effects = page.update(&msg, &mut model)?;

    encode_model_and_effects(&ModelAndEffects { model, effects })
}

pub fn encode_js_value(value: impl Serialize) -> Result<JsValue, serde_wasm_bindgen::Error> {
    value.serialize(&JSON_SERIALIZER)
}

pub fn decode_js_value<T>(js_value: JsValue) -> Result<T, serde_wasm_bindgen::Error>
where
    T: serde::de::DeserializeOwned,
{
    serde_wasm_bindgen::from_value(js_value.clone())
}

fn encode_model_and_effects<Model, Msg, AppEffect>(
    model_and_effects: &ModelAndEffects<Model, Msg, AppEffect>,
) -> Result<JsValue, JsValue>
where
    Model: serde::Serialize,
    Msg: serde::Serialize,
    AppEffect: serde::Serialize,
{
    encode_js_value(model_and_effects)
        .map_err(|err| format!("Failed to encode model and effects: {}", err).into())
}

fn encode_subscriptions<Subscriptions>(subscriptions: Subscriptions) -> Result<JsValue, JsValue>
where
    Subscriptions: serde::Serialize,
{
    encode_js_value(subscriptions)
        .map_err(|err| format!("Failed to encode subscriptions: {}", err).into())
}

fn decode_model<Model>(js_model: &JsValue) -> Result<Model, JsValue>
where
    Model: serde::de::DeserializeOwned,
{
    decode_js_value(js_model.clone())
        .map_err(|err| format!("Failed to decode model: {}", err).into())
}

fn decode_msg<Msg>(js_msg: &JsValue) -> Result<Msg, JsValue>
where
    Msg: serde::de::DeserializeOwned,
{
    decode_js_value(js_msg.clone()).map_err(|err| format!("Failed to decode msg: {}", err).into())
}

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelAndEffects<Model, Msg, AppEffect> {
    pub model: Model,
    pub effects: Effects<Msg, AppEffect>,
}
