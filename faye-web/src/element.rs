use maud::Render;
use wasm_bindgen::prelude::*;

use crate::renderer;

#[derive(Clone)]
pub struct Element {
    pub inner: renderer::HtmlElement,
}

impl Element {
    pub const fn new(inner: renderer::HtmlElement) -> Self {
        Self { inner }
    }

    pub fn root() -> Self {
        Self::new(renderer::root())
    }

    pub fn push<S: Render>(&self, html: S) -> Self {
        Self::new(renderer::push_to(&self.inner, &html.render().into_string()))
    }

    pub fn update<S: Render>(&self, html: S) {
        self.inner.set_inner_html(&html.render().into_string());
    }

    pub fn text(&self) -> String {
        self.inner.inner_text()
    }

    pub fn listen<E: JsCast, F: FnMut(E) + 'static>(&self, event: &str, mut callback: F) {
        let cl = Closure::new(move |e: JsValue| callback(e.unchecked_into()));
        renderer::listen(event, &self.inner, &cl);
        std::mem::forget(cl); // leak memory to JS
    }

    pub fn scroll_into_view(&self) {
        renderer::scroll(&self.inner);
    }

    pub fn set_cursor(&self, position: usize) {
        renderer::set_cursor(&self.inner, position);
    }
}
