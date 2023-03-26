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

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|cx| {
        view! { cx,
            <App/>
        }
    })
}
