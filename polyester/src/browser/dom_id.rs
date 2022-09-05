use crate::browser::selector::Selector;
use std::fmt::Display;

pub trait DomId: Display {
    fn selector(&self) -> Selector {
        Selector::id(&self.to_string())
    }
}
