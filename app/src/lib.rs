//! Simple Icons website
//!
//! Built with Rust using [Leptos](https://docs.rs/leptos)
//! and [Tailwind CSS](https://tailwindcss.com/).
//!
//! See the contributing guide for instructions on how to
//! develop and build for production.
//!
pub(crate) mod app;

use crate::app::App;
use leptos::{mount::mount_to_body, prelude::document};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    // Remove #loader and body background
    if let Some(loader) = document().get_element_by_id("loader") {
        loader.remove();
    }
    if let Some(body) = document().body() {
        body.remove_attribute("style").ok();
    }

    mount_to_body(App);

    Ok(())
}
