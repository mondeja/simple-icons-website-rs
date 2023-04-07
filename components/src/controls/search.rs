use crate::controls::ControlsStateSignal;
use crate::debounce;
use crate::storage::LocalStorage;
use i18n::move_gettext;
use leptos::leptos_dom::helpers::TimeoutHandle;
use leptos::*;
use wasm_bindgen::JsCast;
use web_sys;

fn initial_search_value_from_localstorage() -> Option<String> {
    let window = web_sys::window().unwrap();
    let local_storage = window.local_storage().unwrap().unwrap();

    match local_storage.get_item(LocalStorage::Keys::SearchValue.as_str()) {
        Ok(Some(search_value)) => Some(search_value),
        _ => None,
    }
}

pub fn initial_search_value_from_url_or_localstorage() -> String {
    let search_value: Option<String> =
        match web_sys::window().unwrap().location().search() {
            Ok(search) => {
                let search = search.trim_start_matches('?');
                let search = search.trim_start_matches("search=");
                if search.is_empty() {
                    None
                } else {
                    Some(search.to_string())
                }
            }
            Err(_) => None,
        };

    match search_value {
        Some(search_value) => search_value,
        None => match initial_search_value_from_localstorage() {
            Some(search_value) => search_value,
            None => String::new(),
        },
    }
}

pub fn set_search_value_on_localstorage(search_value: &str) {
    let window = web_sys::window().unwrap();
    let local_storage = window.local_storage().unwrap().unwrap();
    local_storage
        .set_item(LocalStorage::Keys::SearchValue.as_str(), search_value)
        .unwrap();
}

#[component]
pub fn SearchControl(cx: Scope) -> impl IntoView {
    let controls_state = use_context::<ControlsStateSignal>(cx).unwrap().0;

    // timeout for search debouncing
    //
    // TODO: improve rendering time to remove debouncing
    let mut timeout: Option<TimeoutHandle> = None;

    view! { cx,
        <div class="flex flex-col">
            <label for="search">{move_gettext!(cx, "Search")}</label>
            <input
                id="search"
                type="search"
                class="border px-2 py-1 h-10"
                placeholder=move_gettext!(cx, "Search by brand...")
                on:input=move|event: web_sys::Event| {
                    debounce(
                        &mut timeout,
                        400,
                        Box::new(move || {
                            let value = event.clone().target().unwrap().unchecked_into::<web_sys::HtmlInputElement>()
                                .value();
                            controls_state.update(
                                move|state| state.set_search_value(&value),
                            );
                        })
                    );
                }
            />
        </div>
    }
}
