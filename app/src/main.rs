//! Simple Icons website
//!
//! Built with Rust using [Leptos](https://docs.rs/leptos)
//! and [Tailwind CSS](https://tailwindcss.com/).
//!
//! See the contributing guide for instructions on how to
//! develop and build for production.
//!
pub(crate) mod app;
pub(crate) mod meta;

use crate::app::{App, AppProps};
use console_error_panic_hook;
use leptos::{document, mount_to, view};
use log::Level;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

pub fn main() {
    _ = console_log::init_with_level(Level::Debug);
    console_error_panic_hook::set_once();

    let html = document()
        .document_element()
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();
    mount_to(html, |cx| {
        view! { cx, <App/> }
    })
}
