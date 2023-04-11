use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/controls/search/fuzzy.js")]
extern "C" {
    pub fn build_searcher(candidates: &JsValue);
    pub fn search(query: &str) -> JsValue;
}
