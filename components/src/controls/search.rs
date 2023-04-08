use crate::debounce;
use crate::grid::{DisplayedIconsSignal, ICONS};
use crate::storage::LocalStorage;
use config::CONFIG;
use i18n::move_gettext;
use leptos::leptos_dom::helpers::TimeoutHandle;
use leptos::*;
use rust_fuzzy_search::fuzzy_search;
use simple_icons::FullStaticSimpleIcon;
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

#[derive(Copy, Clone)]
pub struct SearchValueSignal(pub RwSignal<String>);

#[component]
pub fn SearchControl(cx: Scope) -> impl IntoView {
    let displayed_icons = use_context::<DisplayedIconsSignal>(cx).unwrap().0;
    let search = use_context::<SearchValueSignal>(cx).unwrap().0;

    // timeout for search debouncing
    //
    // TODO: improve rendering time to remove debouncing
    let mut timeout: Option<TimeoutHandle> = None;

    let on_search_input = move |event: web_sys::Event| {
        debounce(
            &mut timeout,
            400,
            Box::new(move || {
                let value = event
                    .clone()
                    .target()
                    .unwrap()
                    .unchecked_into::<web_sys::HtmlInputElement>()
                    .value();
                search.update(move |state| {
                    set_search_value_on_localstorage(&value);

                    if value.is_empty() {
                        displayed_icons.update(move |state| {
                            *state = ICONS.to_vec();
                        });
                        *state = value;
                        return;
                    }

                    let icon_titles = ICONS
                        .iter()
                        .map(|icon| icon.title)
                        .collect::<Vec<&str>>();
                    let res: Vec<(&str, f32)> =
                        fuzzy_search(&value, &icon_titles);

                    let mut new_displayed_icons: Vec<FullStaticSimpleIcon> =
                        Vec::with_capacity(ICONS.len());
                    for (i, (_title, score)) in res.iter().enumerate() {
                        if *score > CONFIG.min_search_score {
                            new_displayed_icons.push(ICONS[i].clone())
                        }
                    }
                    displayed_icons.update(move |state| {
                        *state = new_displayed_icons;
                    });

                    *state = value;
                });
            }),
        );
    };

    view! { cx,
        <div class="control">
            <label for="search">{move_gettext!(cx, "Search")}</label>
            <input
                id="search"
                type="search"
                placeholder=move_gettext!(cx, "Search by brand...")
                on:input=on_search_input
            />
        </div>
    }
}
