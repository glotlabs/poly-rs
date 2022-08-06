use crate::page::Page;
use wasm_bindgen::prelude::*;

pub fn initial_model<P, Model, Msg>(page: &P) -> Result<JsValue, JsValue>
where
    P: Page<Model, Msg>,
    Model: serde::Serialize,
{
    let model = page.initial_model();
    encode_model(&model)
}

pub fn view_body<P, Model, Msg>(page: &P, js_model: &JsValue) -> Result<String, JsValue>
where
    P: Page<Model, Msg>,
    Model: serde::de::DeserializeOwned,
{
    let model = decode_model(js_model)?;
    let page_markup = page.view(&model);

    return Ok(page_markup.body.into_string());
}

pub fn get_subscriptions<P, Model, Msg>(page: &P, js_model: &JsValue) -> Result<JsValue, JsValue>
where
    P: Page<Model, Msg>,
    Model: serde::de::DeserializeOwned,
    Msg: serde::Serialize,
{
    let model = decode_model(js_model)?;
    let subscriptions = page.subscriptions(&model);
    encode_subscriptions(&subscriptions)
}

pub fn update<P, Model, Msg>(
    page: &P,
    js_msg: &JsValue,
    js_model: &JsValue,
) -> Result<JsValue, JsValue>
where
    P: Page<Model, Msg>,
    Msg: serde::de::DeserializeOwned,
    Model: serde::de::DeserializeOwned,
    Model: serde::Serialize,
{
    let msg = decode_msg(js_msg)?;
    let mut model = decode_model(js_model)?;
    page.update(&msg, &mut model)?;

    encode_model(&model)
}

fn encode_model<Model>(model: Model) -> Result<JsValue, JsValue>
where
    Model: serde::Serialize,
{
    JsValue::from_serde(&model).map_err(|err| format!("Failed to encode model: {}", err).into())
}

fn encode_subscriptions<Subscriptions>(subscriptions: Subscriptions) -> Result<JsValue, JsValue>
where
    Subscriptions: serde::Serialize,
{
    JsValue::from_serde(&subscriptions)
        .map_err(|err| format!("Failed to encode subscriptions: {}", err).into())
}

fn decode_model<Model>(js_model: &JsValue) -> Result<Model, JsValue>
where
    Model: serde::de::DeserializeOwned,
{
    js_model
        .into_serde()
        .map_err(|err| format!("Failed to decode model: {}", err).into())
}

fn decode_msg<Msg>(js_msg: &JsValue) -> Result<Msg, JsValue>
where
    Msg: serde::de::DeserializeOwned,
{
    js_msg
        .into_serde()
        .map_err(|err| format!("Failed to decode msg: {}", err).into())
}
