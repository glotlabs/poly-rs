pub mod wasm;

use crate::browser::DomId;
use crate::browser::Effects;
use crate::browser::Subscriptions;
use maud::html;

pub trait Page<Model, Msg, AppEffect, Markup> {
    fn id(&self) -> &'static dyn DomId;
    fn init(&self) -> (Model, Effects<Msg, AppEffect>);
    fn subscriptions(&self, model: &Model) -> Subscriptions<Msg, AppEffect>;
    fn update(&self, msg: &Msg, model: &mut Model) -> Result<Effects<Msg, AppEffect>, String>;
    fn view(&self, model: &Model) -> PageMarkup<Markup>;
    fn render(&self, markup: Markup) -> String;
    fn render_page(&self, markup: PageMarkup<Markup>) -> String;
}

pub struct PageMarkup<Html> {
    pub head: Html,
    pub body: Html,
}

pub fn render_page_maud(markup: PageMarkup<maud::Markup>) -> String {
    (html! {
        (maud::DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                (markup.head)
            }
            body {
                (markup.body)
            }
        }
    })
    .into_string()
}
