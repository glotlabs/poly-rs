use crate::browser::Effect;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "config")]
#[serde(rename_all = "camelCase")]
pub enum Clipboard {
    WriteText {
        text: String,
        success_msg_name: String,
        failure_msg_name: String,
    },
}

pub fn write_text<Msg, AppEffect>(
    s: &str,
    success_msg_name: &str,
    failure_msg_name: &str,
) -> Effect<Msg, AppEffect> {
    Effect::Clipboard(Clipboard::WriteText {
        text: s.to_string(),
        success_msg_name: success_msg_name.to_string(),
        failure_msg_name: failure_msg_name.to_string(),
    })
}
