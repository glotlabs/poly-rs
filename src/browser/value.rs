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

    pub fn empty() -> Value {
        Value(serde_json::Value::Null)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn to_value<T>(value: T) -> Value
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
