//! Simple Icons website
//!
//! Built in Rust using [Leptos](https://docs.rs/leptos)
//! and [Tailwind CSS](https://tailwindcss.com/).
//!
//! See the contributing guide for instructions on how to
//! develop and build for production.

mod app;

use crate::app::{App, AppProps};
use console_error_panic_hook;
use console_log;
use leptos::{mount_to, view};
use log::Level;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlElement};

pub fn main() {
    _ = console_log::init_with_level(Level::Debug);
    console_error_panic_hook::set_once();
    let html = window()
        .unwrap()
        .document()
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();
    mount_to(html, |cx| {
        view! { cx,
            <App/>
        }
    })
}
