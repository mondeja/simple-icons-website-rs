//! Simple Icons website
//!
//! Built in Rust using [Leptos](https://docs.rs/leptos)
//! and [Tailwind CSS](https://tailwindcss.com/).
//!
//! See the [contributing guide](https://github.com/mondeja/simple-icons-website-rs/blob/master/CONTRIBUTING.md)
//! for instructions on how to develop and build for production.

mod app;

use crate::app::*;
use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    let html = document()
        .document_element()
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();
    mount_to(html, |cx| {
        view! { cx,
            <App/>
        }
    })
}
