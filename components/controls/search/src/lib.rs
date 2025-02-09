use fast_fuzzy::search;
use simple_icons_website_grid_constants::ICONS;
use simple_icons_website_ids::Ids;
use simple_icons_website_types::SimpleIcon;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

pub fn get_search_input() -> HtmlInputElement {
    leptos::prelude::document()
        .get_element_by_id(Ids::SearchInput.as_str())
        .unwrap()
        .unchecked_into::<HtmlInputElement>()
}

/// Set the focus on the search bar
pub fn focus_search_bar() {
    let input = get_search_input();
    _ = input.blur();
    _ = input.focus();
}

pub fn search_icons_and_returns_first_page(
    search_value: &str,
    icons_per_page: usize,
) -> (Vec<&'static SimpleIcon>, Vec<&'static SimpleIcon>) {
    let search_result = js_sys::Array::from(&search(search_value));
    let search_result_length = search_result.length();

    let new_displayed_icons = new_displayed_icons_from_search_result(
        &search_result,
        &search_result_length,
        icons_per_page,
    );

    let mut new_icons = Vec::with_capacity(search_result_length as usize);
    new_icons.extend(new_displayed_icons.clone());
    extend_new_icons_with_search_result(
        &search_result,
        &search_result_length,
        &mut new_icons,
        icons_per_page as u32,
    );

    (new_icons, new_displayed_icons)
}

pub fn new_displayed_icons_from_search_result(
    search_result: &js_sys::Array,
    search_result_length: &u32,
    icons_per_page: usize,
) -> Vec<&'static SimpleIcon> {
    let mut new_displayed_icons: Vec<&'static SimpleIcon> = Vec::new();
    for i in 0..*search_result_length {
        let result_icon_array = js_sys::Array::from(&search_result.get(i));
        let icon_order_alpha = result_icon_array.get(1).as_f64().unwrap();
        new_displayed_icons.push(&ICONS[icon_order_alpha as usize]);
        if new_displayed_icons.len() >= icons_per_page {
            break;
        }
    }

    new_displayed_icons
}

pub fn extend_new_icons_with_search_result(
    search_result: &js_sys::Array,
    search_result_length: &u32,
    new_icons: &mut Vec<&'static SimpleIcon>,
    icons_per_page: u32,
) {
    if *search_result_length > icons_per_page {
        for i in icons_per_page..*search_result_length {
            let result_icon_array = js_sys::Array::from(&search_result.get(i));
            let icon_order_alpha = result_icon_array.get(1).as_f64().unwrap();
            new_icons.push(&ICONS[icon_order_alpha as usize]);
        }
    }
}
