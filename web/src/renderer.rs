use wasm_bindgen::prelude::*;
pub use web_sys::HtmlElement;

#[wasm_bindgen(module = "/assets/render.js")]
extern "C" {
    #[wasm_bindgen]
    pub fn root() -> HtmlElement;

    #[wasm_bindgen]
    pub fn push_to(el: &HtmlElement, html: &str) -> HtmlElement;

    #[wasm_bindgen]
    pub fn listen(event: &str, el: &HtmlElement, callback: &Closure<dyn FnMut(JsValue)>);

    #[wasm_bindgen]
    pub fn scroll(el: &HtmlElement);
}
