use crate::browser::selector::Selector;
use std::fmt;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct DomId(String);

impl DomId {
    pub fn new(id: &str) -> DomId {
        DomId(id.into())
    }

    pub fn selector(&self) -> Selector {
        Selector::new(&format!("#{}", self))
    }
}

impl From<&str> for DomId {
    fn from(s: &str) -> Self {
        DomId(s.into())
    }
}

impl fmt::Display for DomId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
