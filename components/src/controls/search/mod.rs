mod fuzzy;

use crate::controls::order::{OrderMode, OrderModeSignal};
use crate::debounce;
use crate::grid::{DisplayedIconsSignal, ICONS, INITIAL_ICONS};
use crate::storage::LocalStorage;
use config::CONFIG;
use fuzzy::{rebuild_searcher, search};
use i18n::move_gettext;
use js_sys::JsString;
use leptos::leptos_dom::helpers::TimeoutHandle;
use leptos::*;
use rust_fuzzy_search::fuzzy_search;
use simple_icons::StaticSimpleIcon;
use wasm_bindgen::{JsCast, JsValue};
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

fn init_searcher() {
    let icon_titles =
        ICONS.iter().map(|icon| icon.title).collect::<Vec<&str>>();

    let icons_titles_ids = ICONS
        .iter()
        .map(|icon| (icon.title, icon.order_alpha))
        .collect::<Vec<(&str, usize)>>();

    let icon_titles_ids_js_array = js_sys::Array::new();
    for (icon_title, icon_order_alpha) in &icons_titles_ids {
        let icon_title_id_array = js_sys::Array::new_with_length(2);
        icon_title_id_array.set(0, (JsString::from(*icon_title)).into());
        icon_title_id_array
            .set(1, (js_sys::Number::from(*icon_order_alpha as u32)).into());

        icon_titles_ids_js_array.push(&icon_title_id_array);
    }
    rebuild_searcher(&icon_titles_ids_js_array);
}

pub fn initial_search_value() -> String {
    let search_value = initial_search_value_from_url_or_localstorage();
    init_searcher();
    search_value
}

#[derive(Copy, Clone)]
pub struct SearchValueSignal(pub RwSignal<String>);

pub fn search_icons_and_returns_first_page(
    search_value: &str,
) -> Vec<StaticSimpleIcon> {
    let res = js_sys::Array::from(&search(search_value));

    let response_length = res.length();

    let mut new_displayed_icons: Vec<StaticSimpleIcon> =
        Vec::with_capacity(response_length as usize);
    for i in 0..response_length {
        let res_array = js_sys::Array::from(&res.get(i));
        let icon_order_alpha = res_array.get(1).as_f64().unwrap() as usize;
        let icon = ICONS[icon_order_alpha as usize];
        new_displayed_icons.push(icon);
        if new_displayed_icons.len() >= CONFIG.max_icons_per_page {
            break;
        }
    }
    new_displayed_icons
}

pub fn search_icons(
    search_value: &str,
    displayed_icons_signal: &RwSignal<Vec<StaticSimpleIcon>>,
) {
    let res = js_sys::Array::from(&search(search_value));

    let response_length = res.length();

    let mut new_displayed_icons: Vec<StaticSimpleIcon> =
        Vec::with_capacity(response_length as usize);
    for i in 0..response_length {
        let res_array = js_sys::Array::from(&res.get(i));
        let icon_order_alpha = res_array.get(1).as_f64().unwrap() as usize;
        let icon = ICONS[icon_order_alpha as usize];
        new_displayed_icons.push(icon);
        if new_displayed_icons.len() >= CONFIG.max_icons_per_page {
            break;
        }
    }

    displayed_icons_signal.update(move |state| {
        *state = new_displayed_icons;
    });
}

#[component]
pub fn SearchControl(cx: Scope) -> impl IntoView {
    let displayed_icons = use_context::<DisplayedIconsSignal>(cx).unwrap().0;
    let search = use_context::<SearchValueSignal>(cx).unwrap().0;
    let order_mode = use_context::<OrderModeSignal>(cx).unwrap().0;

    let on_search_input = move |event: web_sys::Event| {
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
                    *state = INITIAL_ICONS.to_vec();
                });
                *state = value;
                return;
            }

            search_icons(&value, &displayed_icons);

            *state = value;
        });
    };

    view! { cx,
        <div class="control">
            <label for="search">{move_gettext!(cx, "Search")}</label>
            <input
                id="search"
                type="search"
                placeholder=move_gettext!(cx, "Search by brand...")
                on:input=on_search_input
                value=search
            />
        </div>
    }
}
