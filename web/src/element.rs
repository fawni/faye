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

    pub fn push<S: Into<String>>(&self, html: S) -> Self {
        Self::new(renderer::push_to(&self.inner, &html.into()))
    }

    pub fn listen<E: JsCast, F: FnMut(E) + 'static>(&self, event: &str, mut callback: F) {
        let cl = Closure::new(move |e: JsValue| callback(e.unchecked_into()));
        renderer::listen(event, &self.inner, &cl);
        std::mem::forget(cl); // leak memory to JS
    }

    pub fn scroll_into_view(&self) {
        renderer::scroll(&self.inner);
    }
}
