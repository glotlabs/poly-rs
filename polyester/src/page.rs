pub mod wasm;

use crate::browser::DomId;
use crate::browser::Effects;
use crate::browser::Subscriptions;
use maud::html;
use maud::Markup;

pub struct PageMarkup {
    pub head: Markup,
    pub body: Markup,
}

impl PageMarkup {
    pub fn to_markup(&self) -> Markup {
        html! {
            (maud::DOCTYPE)
            html {
                head {
                    meta charset="utf-8";
                    (self.head)
                }
                body {
                    (self.body)
                }
            }
        }
    }
}

pub trait Page<Model, Msg, AppEffect> {
    fn id(&self) -> DomId;
    fn init(&self) -> (Model, Effects<Msg, AppEffect>);
    fn subscriptions(&self, model: &Model) -> Subscriptions<Msg, AppEffect>;
    fn update(&self, msg: &Msg, model: &mut Model) -> Result<Effects<Msg, AppEffect>, String>;
    fn view(&self, model: &Model) -> PageMarkup;
}
