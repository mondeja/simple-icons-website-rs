//! App controls

mod button;
pub mod download;
pub mod order;
pub mod search;

use button::*;
use config::CONFIG;
use download::*;
use i18n::move_gettext;
use leptos::*;
use macros::{get_number_of_icons, simple_icons_array};
use order::*;
use rust_fuzzy_search::fuzzy_search;
use search::*;
use simple_icons::FullStaticSimpleIcon;

const SIMPLE_ICONS: [FullStaticSimpleIcon;
    CONFIG.max_icons.unwrap_or(get_number_of_icons!())] = simple_icons_array!();

/// State of the controls
#[derive(Clone, Default)]
pub struct ControlsState {
    /// Current active download type when the download button of
    /// each icon is clicked
    pub download_type: DownloadType,
    /// Order mode selected
    pub order_mode: OrderMode,
    /// Shown icons
    pub shown_icons: Vec<FullStaticSimpleIcon>,
    /// Search value currently active
    pub search_value: String,
}

impl ControlsState {
    /// Creates a new `ControlsState`
    pub fn new() -> Self {
        let order_mode = initial_order_mode_from_localstorage();
        let mut simple_icons = SIMPLE_ICONS.to_vec();
        order_shown_icons(&mut simple_icons, order_mode);
        Self {
            download_type: initial_download_type_from_localstorage(),
            order_mode,
            shown_icons: simple_icons,
            search_value: initial_search_value_from_url_or_localstorage(),
        }
    }

    pub fn set_order_mode(&mut self, order_mode: OrderMode) {
        order_shown_icons(&mut self.shown_icons, order_mode);
        set_order_mode_on_localstorage(order_mode);
        self.order_mode = order_mode;
    }

    pub fn set_search_value(&mut self, search_value: &str) {
        set_search_value_on_localstorage(search_value);
        self.search_value = search_value.to_string();

        let icon_titles = SIMPLE_ICONS
            .iter()
            .map(|icon| icon.title)
            .collect::<Vec<&str>>();
        let res: Vec<(&str, f32)> = fuzzy_search(search_value, &icon_titles);

        let mut new_shown_icons: Vec<FullStaticSimpleIcon> =
            Vec::with_capacity(SIMPLE_ICONS.len());
        for (i, (t, score)) in res.iter().enumerate() {
            if *score > CONFIG.min_search_score {
                new_shown_icons.push(SIMPLE_ICONS[i].clone())
            }
        }
        self.shown_icons = new_shown_icons;
    }
}

fn order_shown_icons(
    icons: &mut Vec<FullStaticSimpleIcon>,
    order_mode: OrderMode,
) {
    match order_mode {
        OrderMode::Alphabetic => {
            icons.sort_by(|a, b| a.order_alpha.cmp(&b.order_alpha));
        }
        OrderMode::Color => {
            icons.sort_by(|a, b| a.order_color.cmp(&b.order_color));
        }
        _ => todo!(),
    }
}

#[derive(Copy, Clone)]
pub struct ControlsStateSignal(pub RwSignal<ControlsState>);

#[component]
pub fn Controls(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex flex-row space-x-4">
            <SearchControl/>
            <OrderControl/>
            <ColorSchemeControl/>
            <DownloadFileTypeControl/>
            <LayoutControl/>
        </div>
    }
}

#[component]
pub fn ColorSchemeControl(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex flex-col">
            <label>{move_gettext!(cx, "Theme")}</label>
            <div class="flex flex-row">
                <ControlButtonSVGPath
                    title=move_gettext!(cx, "Light color scheme")
                    svg_path="M12 18c-3.309 0-6-2.691-6-6s2.691-6 6-6 6 2.691 6 6-2.691 6-6 6zm0-10c-2.206 0-4 1.794-4 4s1.794 4 4 4 4-1.794 4-4-1.794-4-4-4zm0-4a1 1 0 0 1-1-1V1a1 1 0 0 1 2 0v2a1 1 0 0 1-1 1zm0 20a1 1 0 0 1-1-1v-2a1 1 0 1 1 2 0v2a1 1 0 0 1-1 1zM5.64 6.64a.997.997 0 0 1-.707-.293l-1.42-1.42a.999.999 0 1 1 1.414-1.414l1.42 1.42A.999.999 0 0 1 5.64 6.64zm14.139 14.139a.997.997 0 0 1-.707-.293l-1.42-1.42a.999.999 0 1 1 1.414-1.414l1.42 1.42a.999.999 0 0 1-.707 1.707zM3 13H1a1 1 0 1 1 0-2h2a1 1 0 0 1 0 2zm20 0h-2a1 1 0 1 1 0-2h2a1 1 0 1 1 0 2zM4.22 20.779a.999.999 0 0 1-.707-1.707l1.42-1.42a.999.999 0 1 1 1.414 1.414l-1.42 1.42a.993.993 0 0 1-.707.293zM18.359 6.64a.999.999 0 0 1-.707-1.707l1.42-1.42a.999.999 0 1 1 1.414 1.414l-1.42 1.42a.997.997 0 0 1-.707.293z"
                    active=||{false}
                />
                <ControlButtonSVGPath
                    title=move_gettext!(cx, "Dark color scheme")
                    svg_path="M12.048 21.963c-.308 0-.618-.015-.93-.043-2.66-.246-5.064-1.513-6.771-3.567s-2.512-4.651-2.266-7.311a10.004 10.004 0 0 1 9.038-9.038 1 1 0 0 1 .896 1.589 6.008 6.008 0 0 0 1.258 8.392c2.078 1.536 5.055 1.536 7.133 0a1 1 0 0 1 1.591.896 9.951 9.951 0 0 1-9.949 9.082zM9.315 4.438a8.006 8.006 0 0 0-5.244 6.787 7.954 7.954 0 0 0 1.813 5.849 7.95 7.95 0 0 0 5.417 2.854 7.95 7.95 0 0 0 8.266-5.243 8.01 8.01 0 0 1-2.729.476 7.946 7.946 0 0 1-4.755-1.565C9.174 11.443 8.145 7.68 9.315 4.438z"
                    active=||{false}
                />
                <ControlButtonSVGPath
                    title=move_gettext!(cx, "System color scheme")
                    svg_path="M12 4a1 1 0 0 1-1-1V1a1 1 0 0 1 2 0v2a1 1 0 0 1-1 1zM4.933 6.348a.997.997 0 0 0 1.414 0 .999.999 0 0 0 0-1.414l-1.42-1.42a.999.999 0 1 0-1.414 1.414l1.42 1.42zM1 13h2a1 1 0 1 0 0-2H1a1 1 0 0 0 0 2zm19.486-8.072 3.221-3.221A.999.999 0 1 0 22.293.293l-3.221 3.221-1.42 1.42-2.19 2.19A5.955 5.955 0 0 0 12 6c-3.309 0-6 2.691-6 6 0 1.258.406 2.453 1.124 3.462l-2.105 2.105c-.026.021-.058.03-.083.055s-.033.056-.055.082l-1.368 1.368-.001.002-3.219 3.219a.999.999 0 1 0 1.414 1.414l3.987-3.987a10.03 10.03 0 0 0 6.332 2.262c5.103-.001 9.473-3.902 9.951-9.081a1 1 0 0 0-1.591-.896 5.96 5.96 0 0 1-7.037.06l5.717-5.716 1.42-1.421zm-.945 9.78c-1.21 3.337-4.564 5.587-8.257 5.238a8.019 8.019 0 0 1-4.165-1.651l4.802-4.802c.05.038.093.082.144.12a7.955 7.955 0 0 0 7.476 1.095zm-10.979-.684A3.968 3.968 0 0 1 8 12c0-2.206 1.794-4 4-4a3.98 3.98 0 0 1 2.025.561l-5.463 5.463z"
                    active=||{false}
                />
            </div>
        </div>
    }
}

#[component]
pub fn LayoutControl(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex flex-col">
            <label>{move_gettext!(cx, "Layout")}</label>
            <div class="flex flex-row">
                <ControlButtonSVGPath
                    title=move_gettext!(cx, "Comfortable")
                    svg_path="M19 2a2 2 0 0 1 2 2v2a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h14zm0 4V4H5v2h14zm0 10a2 2 0 0 1 2 2v2a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-2a2 2 0 0 1 2-2h14zm0 4v-2H5v2h14zm0-11a2 2 0 0 1 2 2v2a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-2a2 2 0 0 1 2-2h14zm0 4v-2H5v2h14z"
                    active=||{false}
                />
                <ControlButtonSVGPath
                    title=move_gettext!(cx, "Compact")
                    svg_path="M2 5.5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v13a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-13zm9 0H4v3h7v-3zm2 0v3h7v-3h-7zm7 5h-7v3h7v-3zm0 5h-7v3h7v-3zm-9 3v-3H4v3h7zm-7-5h7v-3H4v3z"
                    active=||{false}
                />
            </div>
        </div>
    }
}
