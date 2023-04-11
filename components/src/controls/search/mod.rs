mod fuzzy;

use crate::grid::{IconsGrid, IconsGridSignal, ICONS, INITIAL_ICONS};
use crate::storage::LocalStorage;
use config::CONFIG;
use fuzzy::{build_searcher, search};
use i18n::move_gettext;
use js_sys::JsString;
use leptos::{html::Input, *};
use simple_icons::StaticSimpleIcon;
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
    // TODO: include aliases in the search
    let icons_titles_ids = ICONS
        .iter()
        .map(|icon| (icon.title, icon.order_alpha))
        .collect::<Vec<(&str, usize)>>();

    // TODO: `js_sys::Array::new_with_length` generates an array with a first
    // undefined value (investigate why)
    let icon_titles_ids_js_array = js_sys::Array::new();
    for (icon_title, icon_order_alpha) in &icons_titles_ids {
        let icon_title_id_array = js_sys::Array::of2(
            &JsString::from(*icon_title).into(),
            &js_sys::Number::from(*icon_order_alpha as u32).into(),
        );
        icon_titles_ids_js_array.push(&icon_title_id_array);
    }
    build_searcher(&icon_titles_ids_js_array);
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
) -> (Vec<StaticSimpleIcon>, Vec<StaticSimpleIcon>) {
    let search_result = js_sys::Array::from(&search(search_value));

    let search_result_length = search_result.length();

    let mut new_displayed_icons: Vec<StaticSimpleIcon> =
        Vec::with_capacity(search_result_length as usize);
    for i in 0..search_result_length {
        let result_icon_array = js_sys::Array::from(&search_result.get(i));
        let icon_order_alpha =
            result_icon_array.get(1).as_f64().unwrap() as usize;
        let icon = ICONS[icon_order_alpha as usize];
        new_displayed_icons.push(icon);
        if new_displayed_icons.len() >= CONFIG.icons_per_page {
            break;
        }
    }

    let mut new_icons = Vec::with_capacity(search_result_length as usize);
    if search_result_length > CONFIG.icons_per_page as u32 {
        for i in (CONFIG.icons_per_page as u32)..search_result_length {
            let result_icon_array = js_sys::Array::from(&search_result.get(i));
            let icon_order_alpha =
                result_icon_array.get(1).as_f64().unwrap() as usize;
            let icon = ICONS[icon_order_alpha as usize];
            new_icons.push(icon);
        }
    }

    (new_icons, new_displayed_icons)
}

pub fn search_icons(
    search_value: &str,
    icons_grid_signal: &RwSignal<IconsGrid>,
) {
    let search_result = js_sys::Array::from(&search(search_value));

    let search_result_length = search_result.length();

    let mut new_displayed_icons: Vec<StaticSimpleIcon> = Vec::new();
    for i in 0..search_result_length {
        let result_icon_array = js_sys::Array::from(&search_result.get(i));
        let icon_order_alpha =
            result_icon_array.get(1).as_f64().unwrap() as usize;
        let icon = ICONS[icon_order_alpha as usize];
        new_displayed_icons.push(icon);
        if new_displayed_icons.len() >= CONFIG.icons_per_page {
            break;
        }
    }

    icons_grid_signal
        .update(move |grid| grid.set_loaded_icons(new_displayed_icons));

    let mut new_icons = Vec::with_capacity(search_result_length as usize);
    if search_result_length > CONFIG.icons_per_page as u32 {
        for i in (CONFIG.icons_per_page as u32)..search_result_length {
            let result_icon_array = js_sys::Array::from(&search_result.get(i));
            let icon_order_alpha =
                result_icon_array.get(1).as_f64().unwrap() as usize;
            let icon = ICONS[icon_order_alpha as usize];
            new_icons.push(icon);
        }
    }

    icons_grid_signal.update(move |grid| grid.set_icons(new_icons));
}

#[component]
pub fn SearchControl(cx: Scope) -> impl IntoView {
    let icons_grid = use_context::<IconsGridSignal>(cx).unwrap().0;
    let search = use_context::<SearchValueSignal>(cx).unwrap().0;
    let search_input_ref = create_node_ref::<Input>(cx);

    view! { cx,
        <div class="control">
            <label for="search">{move_gettext!(cx, "Search")}</label>
            <input
                _ref=search_input_ref
                id="search"
                type="search"
                placeholder=move_gettext!(cx, "Search by brand...")
                on:input=move |_| {
                    let value = search_input_ref.get().unwrap().value();
                    search.update(move |state| {
                        if value.is_empty() {
                            icons_grid.update(move |grid| {
                                grid.set_loaded_icons(INITIAL_ICONS.to_vec());
                            });
                            set_search_value_on_localstorage(&value);
                            *state = value;
                            return;
                        }

                        search_icons(&value, &icons_grid);
                        set_search_value_on_localstorage(&value);
                        *state = value;
                    });
                }
                value=search
            />
        </div>
    }
}
