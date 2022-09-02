use std::fmt;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Selector(String);

impl Selector {
    pub fn id(id: &str) -> Selector {
        Selector(format!("#{}", id))
    }

    pub fn radio_group(name: &str) -> Selector {
        Selector(format!("input[type=radio][name={}]", name))
    }
}

impl fmt::Display for Selector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
