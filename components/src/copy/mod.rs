use crate::Ids;
use leptos::leptos_dom::helpers::set_timeout_with_handle;
use leptos::*;
use log;
use std::time::Duration;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures;
use web_sys;

fn on_copied(button: web_sys::HtmlElement) {
    button.class_list().add_1("copied").unwrap();
    _ = set_timeout_with_handle(
        move || {
            button.class_list().remove_1("copied").ok();

            // Unset focus
            button.blur().ok();
        },
        Duration::from_millis(1000),
    );
}

fn on_error(value: &str, err: &JsValue) {
    log::error!("Error copying value '{}' to clipboard: {:?}", value, err);
}

/// Copy a value to the clipboard and sets a transition in copy button
/// to properly show the user that the value has been copied.
///
/// See the `.copy-button-*` classes components in stylesheet.
pub async fn copy_setting_copied_transition_in_element(
    value: String,
    button: web_sys::HtmlElement,
) {
    match web_sys::window().unwrap().navigator().clipboard() {
        Some(navigator_clipboard) => {
            #[cfg(debug_assertions)]
            log::debug!(
                "Copying value '{}' to clipboard using Navigator.Clipboard API",
                &value
            );

            match wasm_bindgen_futures::JsFuture::from(
                navigator_clipboard.write_text(&value),
            )
            .await
            {
                Ok(_) => on_copied(button),
                Err(err) => on_error(&value, &err),
            }
        }
        None => {
            #[cfg(debug_assertions)]
            log::debug!(
                "Copying value '{}' to clipboard using Document.execCommand",
                &value
            );

            let document = web_sys::window().unwrap().document().unwrap();
            let copy_input = document
                .get_element_by_id(Ids::CopyInput.as_str())
                .unwrap()
                .dyn_into::<web_sys::HtmlInputElement>()
                .unwrap();
            copy_input.set_value(&value);
            copy_input.select();
            match document
                .document_element()
                .unwrap()
                .dyn_into::<web_sys::HtmlDocument>()
                .unwrap()
                .exec_command("copy")
            {
                Ok(_) => on_copied(button),
                Err(err) => on_error(&value, &err),
            }
        }
    }
}

#[component]
pub fn CopyInput(cx: Scope) -> impl IntoView {
    view! { cx, <input aria-hidden=true class="hidden" id=Ids::CopyInput.as_str() tabindex=-1/> }
}
