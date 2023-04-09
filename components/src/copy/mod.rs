use leptos::leptos_dom::helpers::set_timeout_with_handle;
use log::error;
use std::time::Duration;
use wasm_bindgen_futures;
use web_sys::{window, HtmlElement};

/// Copy a value to the clipboard and sets a transition in copy button
/// to properly show the user that the value has been copied.
///
/// See the `.copy-button` class component in stylesheet.
pub async fn copy_setting_copied_transition_in_element(
    value: String,
    button: HtmlElement,
) {
    let navigator_clipboard =
        window().unwrap().navigator().clipboard().unwrap();
    match wasm_bindgen_futures::JsFuture::from(
        navigator_clipboard.write_text(&value),
    )
    .await
    {
        Ok(_) => {
            button.class_list().add_1("copied").unwrap();
            let _ = set_timeout_with_handle(
                move || {
                    button.class_list().remove_1("copied").unwrap();
                },
                Duration::from_millis(1000),
            );
        }
        Err(err) => {
            error!("Error copying value '{}' to clipboard: {:?}", &value, err);
        }
    }
}
