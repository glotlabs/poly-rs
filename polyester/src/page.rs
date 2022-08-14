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

pub trait Page<Model, Msg, CustomEffect> {
    fn id(&self) -> DomId;
    fn init(&self) -> (Model, Effects<Msg, CustomEffect>);
    fn subscriptions(&self, model: &Model) -> Subscriptions<Msg, CustomEffect>;
    fn update(&self, msg: &Msg, model: &mut Model) -> Result<Effects<Msg, CustomEffect>, String>;
    fn view(&self, model: &Model) -> PageMarkup;
}
