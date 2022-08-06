use crate::dom::dom_id::DomId;
use std::fmt;

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct Value(serde_json::Value);

impl Value {
    pub fn parse<T>(&self) -> Result<T, serde_json::Error>
    where
        T: serde::de::DeserializeOwned,
    {
        serde_json::from_value(self.0.clone())
    }

    pub fn capture_from_element(id: &DomId) -> Value {
        to_value(CaptureType::ValueFromElement {
            element_id: id.clone(),
        })
    }

    pub fn capture_window_size() -> Value {
        to_value(CaptureType::WindowSize)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[serde(tag = "type", content = "config")]
#[serde(rename_all = "camelCase")]
pub enum CaptureType {
    #[serde(rename_all = "camelCase")]
    ValueFromElement {
        element_id: DomId,
    },
    WindowSize,
}

fn to_value<T>(value: T) -> Value
where
    T: serde::Serialize,
{
    match serde_json::to_value(value) {
        Ok(json_value) => Value(json_value),

        Err(err) => Value(serde_json::Value::String(format!(
            "Failed to serialize value: {}",
            err
        ))),
    }
}
