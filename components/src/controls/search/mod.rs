mod fuzzy;

use crate::controls::order::{
    set_order_mode, OrderMode, OrderModeSignal, OrderModeVariant,
};
use crate::grid::{IconsGrid, IconsGridSignal, ICONS};
use crate::storage::LocalStorage;
use crate::Ids;
use crate::Url;
use config::CONFIG;
use fuzzy::{build_searcher, search};
use i18n::move_gettext;
use js_sys::JsString;
use leptos::{document, html::Input, window, *};
use types::SimpleIcon;
use web_sys;

#[derive(Copy, Clone)]
pub struct SearchValueSignal(pub RwSignal<String>);

pub fn provide_search_context(cx: Scope) -> String {
    let initial_search_value = initial_search_value(cx);
    provide_context(
        cx,
        SearchValueSignal(create_rw_signal(cx, initial_search_value.clone())),
    );

    initial_search_value
}

fn initial_search_value(cx: Scope) -> String {
    let search_value = match Url::params::get(&Url::params::Names::Search) {
        Some(value) => {
            set_search_value_on_localstorage(value.as_str());
            value
        }
        None => match initial_search_value_from_localstorage() {
            Some(value) => {
                Url::params::update(
                    cx,
                    &Url::params::Names::Search,
                    value.as_str(),
                );
                set_search_value_on_localstorage(value.as_str());
                value
            }
            None => String::new(),
        },
    };

    init_searcher();
    search_value
}

fn initial_search_value_from_localstorage() -> Option<String> {
    let local_storage = window().local_storage().unwrap().unwrap();

    match local_storage.get_item(LocalStorage::Keys::SearchValue.as_str()) {
        Ok(Some(search_value)) => match search_value.is_empty() {
            true => None,
            false => Some(search_value),
        },
        _ => None,
    }
}

pub fn set_search_value_on_localstorage(search_value: &str) {
    let local_storage = window().local_storage().unwrap().unwrap();
    local_storage
        .set_item(LocalStorage::Keys::SearchValue.as_str(), search_value)
        .unwrap();
}

pub fn fire_on_search_event() {
    let input = document()
        .get_element_by_id(Ids::SearchInput.as_str())
        .unwrap();
    let event = web_sys::Event::new_with_event_init_dict(
        "input",
        web_sys::EventInit::new().bubbles(true),
    )
    .unwrap();
    input.dispatch_event(&event).unwrap();
}

fn init_searcher() {
    let icons_candidates_ids = ICONS
        .iter()
        .map(|icon| {
            let mut candidates: Vec<&str> = vec![icon.title, icon.slug];
            candidates.extend(icon.plain_aliases);
            (candidates, icon.order_alpha)
        })
        .collect::<Vec<(Vec<&str>, usize)>>();

    let icon_candidates_ids_js_array = js_sys::Array::new();
    for (icon_candidates, icon_order_alpha) in &icons_candidates_ids {
        let candidates_array = js_sys::Array::new();
        for icon_title in icon_candidates {
            candidates_array.push(&JsString::from(*icon_title).into());
        }

        let icon_title_id_array = js_sys::Array::of2(
            &candidates_array,
            &js_sys::Number::from(*icon_order_alpha as u32).into(),
        );
        icon_candidates_ids_js_array.push(&icon_title_id_array);
    }

    build_searcher(&icon_candidates_ids_js_array);
}

#[inline(always)]
fn new_displayed_icons_from_search_result(
    search_result: &js_sys::Array,
    search_result_length: &u32,
) -> Vec<&'static SimpleIcon> {
    let mut new_displayed_icons: Vec<&'static SimpleIcon> = Vec::new();
    for i in 0..*search_result_length {
        let result_icon_array = js_sys::Array::from(&search_result.get(i));
        let icon_order_alpha = result_icon_array.get(1).as_f64().unwrap();
        new_displayed_icons.push(&ICONS[icon_order_alpha as usize]);
        if new_displayed_icons.len() >= (CONFIG.icons_per_page as usize) {
            break;
        }
    }

    new_displayed_icons
}

#[inline(always)]
fn extend_new_icons_with_search_result(
    search_result: &js_sys::Array,
    search_result_length: &u32,
    new_icons: &mut Vec<&'static SimpleIcon>,
) {
    if *search_result_length > CONFIG.icons_per_page {
        for i in CONFIG.icons_per_page..*search_result_length {
            let result_icon_array = js_sys::Array::from(&search_result.get(i));
            let icon_order_alpha = result_icon_array.get(1).as_f64().unwrap();
            new_icons.push(&ICONS[icon_order_alpha as usize]);
        }
    }
}

pub fn search_icons_and_returns_first_page(
    search_value: &str,
) -> (Vec<&'static SimpleIcon>, Vec<&'static SimpleIcon>) {
    let search_result = js_sys::Array::from(&search(search_value));
    let search_result_length = search_result.length();

    let new_displayed_icons = new_displayed_icons_from_search_result(
        &search_result,
        &search_result_length,
    );

    let mut new_icons = Vec::with_capacity(search_result_length as usize);
    new_icons.extend(new_displayed_icons.clone());
    extend_new_icons_with_search_result(
        &search_result,
        &search_result_length,
        &mut new_icons,
    );

    (new_icons, new_displayed_icons)
}

pub async fn search_icons(
    search_value: String,
    icons_grid_signal: RwSignal<IconsGrid>,
) {
    let search_result = js_sys::Array::from(&search(&search_value));
    let search_result_length = search_result.length();

    let new_displayed_icons = new_displayed_icons_from_search_result(
        &search_result,
        &search_result_length,
    );
    let new_displayed_icons_for_signal = new_displayed_icons.clone();

    icons_grid_signal.update(move |grid| {
        grid.loaded_icons = new_displayed_icons_for_signal;
    });

    let mut new_icons = Vec::with_capacity(search_result_length as usize);
    new_icons.extend(new_displayed_icons);
    extend_new_icons_with_search_result(
        &search_result,
        &search_result_length,
        &mut new_icons,
    );

    icons_grid_signal.update(move |grid| grid.icons = new_icons);
}

async fn on_search(
    cx: Scope,
    search_input_ref: NodeRef<Input>,
    search_signal: RwSignal<String>,
    icons_grid_signal: RwSignal<IconsGrid>,
    order_mode_signal: RwSignal<OrderMode>,
) {
    let value = search_input_ref.get().unwrap().value();
    search_signal.update(move |state| {
        Url::params::update(cx, &Url::params::Names::Search, &value);

        if value.is_empty() {
            // Reset grid
            icons_grid_signal.update(|grid| {
                grid.icons = ICONS.iter().collect();
                grid.loaded_icons = grid
                    .icons
                    .iter()
                    .take(CONFIG.icons_per_page as usize)
                    .copied()
                    .collect();
            });
            // Set new order mode
            set_order_mode(
                &order_mode_signal().favorite,
                &order_mode_signal,
                &icons_grid_signal,
                true,
            );
            set_search_value_on_localstorage(&value);
            *state = value;
            return;
        }

        let search_value_copy = value.clone();
        spawn_local(search_icons(value, icons_grid_signal));
        set_search_value_on_localstorage(&search_value_copy);
        set_order_mode(
            &OrderModeVariant::SearchMatch,
            &order_mode_signal,
            &icons_grid_signal,
            false,
        );
        *state = search_value_copy;
    });
}

#[component]
pub fn SearchControl(cx: Scope) -> impl IntoView {
    let icons_grid = use_context::<IconsGridSignal>(cx).unwrap().0;
    let search = use_context::<SearchValueSignal>(cx).unwrap().0;
    let search_input_ref = create_node_ref::<Input>(cx);
    let order_mode = use_context::<OrderModeSignal>(cx).unwrap().0;

    view! { cx,
        <div class="control">
            <label for=Ids::SearchInput.as_str()>{move_gettext!(cx, "Search")}</label>
            <div class="search">
                <input
                    _ref=search_input_ref
                    id=Ids::SearchInput.as_str()
                    type="search"
                    autocomplete="off"
                    placeholder=move_gettext!(cx, "Search by brand...")
                    value=search
                    on:input=move |_| { spawn_local(on_search(cx, search_input_ref, search, icons_grid, order_mode)) }
                />
                <span
                    class:hidden=move || search().is_empty()
                    title=move_gettext!(cx, "Clear search")
                    on:click=move |_| {
                        search_input_ref.get().unwrap().set_value("");
                        fire_on_search_event();
                    }
                >
                    "×"
                </span>
            </div>
        </div>
    }
}
