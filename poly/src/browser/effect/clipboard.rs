use crate::browser::Effect;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "config")]
#[serde(rename_all = "camelCase")]
pub enum Clipboard {
    WriteText {
        text: String,
        result_msg_name: String,
    },
}

pub fn write_text<Msg, AppEffect>(s: &str, result_msg_name: &str) -> Effect<Msg, AppEffect> {
    Effect::Clipboard(Clipboard::WriteText {
        text: s.to_string(),
        result_msg_name: result_msg_name.to_string(),
    })
}
