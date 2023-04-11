mod ad;
mod item;

use crate::controls::layout::{Layout, LayoutSignal};
use crate::controls::order::OrderMode;
use crate::controls::search::search_icons_and_returns_first_page;
use crate::grid::item::details::*;
use ad::*;
use config::CONFIG;
use item::*;
use lazy_static::lazy_static;
use leptos::*;
use macros::{get_number_of_icons, simple_icons_array};
use simple_icons::StaticSimpleIcon;

pub const ICONS: [StaticSimpleIcon;
    CONFIG.max_icons.unwrap_or(get_number_of_icons!())] = simple_icons_array!();

lazy_static! {
    pub static ref INITIAL_ICONS: Vec<StaticSimpleIcon> =
        ICONS[..CONFIG.icons_per_page].to_vec();
}

/// Icons grid
#[derive(Clone)]
pub struct IconsGrid {
    /// Icons currently loaded
    pub loaded_icons: Vec<StaticSimpleIcon>,
    /// Icons in order of the grid
    pub icons: Vec<StaticSimpleIcon>,
}

impl IconsGrid {
    pub fn new(search_value: &str, order_mode: &OrderMode) -> Self {
        let (icons, loaded_icons) =
            initial_icons_from_search_value_and_order_mode(
                search_value,
                order_mode,
            );
        Self {
            icons,
            loaded_icons,
        }
    }

    pub fn set_icons(&mut self, icons: Vec<StaticSimpleIcon>) {
        self.icons = icons;
    }

    pub fn set_loaded_icons(&mut self, loaded_icons: Vec<StaticSimpleIcon>) {
        self.loaded_icons = loaded_icons;
    }

    pub fn load_next_icons(&mut self) {
        let next_icons: Vec<StaticSimpleIcon> = self
            .icons
            .iter()
            .skip(self.loaded_icons.len())
            .take(CONFIG.icons_per_page)
            .cloned()
            .collect();
        self.loaded_icons.extend(next_icons);
    }
}

#[derive(Copy, Clone)]
pub struct IconsGridSignal(pub RwSignal<IconsGrid>);

#[derive(Copy, Clone)]
pub struct CurrentIconViewSignal(pub RwSignal<Option<StaticSimpleIcon>>);

pub fn initial_icons_from_search_value_and_order_mode(
    search_value: &str,
    order_mode: &OrderMode,
) -> (Vec<StaticSimpleIcon>, Vec<StaticSimpleIcon>) {
    if search_value.is_empty() {
        let mut loaded_icons: Vec<StaticSimpleIcon> = INITIAL_ICONS.to_vec();
        let mut icons: Vec<StaticSimpleIcon> = ICONS.to_vec();
        if order_mode != &OrderMode::Alphabetic {
            // Alphabetical is the default order of the icons in the static array
            order_mode.sort_icons(&mut loaded_icons);
            order_mode.sort_icons(&mut icons);
        }
        (icons, loaded_icons)
    } else {
        search_icons_and_returns_first_page(search_value)
    }
}

/// Icons grid
///
/// The icons grid items are lazy loaded with pagination. The first page is
/// loaded on the first render. The next pages are loaded when the user
/// scrolls to the footer. See the `IntersectionObserver` used inside the
/// `Footer` component.
#[component]
pub fn GridIcons(cx: Scope) -> impl IntoView {
    let icons_grid = use_context::<IconsGridSignal>(cx).unwrap().0;

    view! { cx,
        {move || {
            icons_grid().loaded_icons.iter().map(|icon: &StaticSimpleIcon| {
                view!{ cx, <IconGridItem icon=*icon/> }
            }).collect::<Vec<_>>()}
        }
    }
}

/// Main grid
///
/// Includes the Carbon Ads ad and the icons
#[component]
pub fn Grid(cx: Scope) -> impl IntoView {
    let layout = use_context::<LayoutSignal>(cx).unwrap().0;

    provide_context(cx, CurrentIconViewSignal(create_rw_signal(cx, None)));

    view! { cx,
        <IconDetailsModal/>
        <ul class:layout-compact=move||layout() == Layout::Compact>
            <CarbonAdsAdGridItem/>
            <GridIcons />
        </ul>
    }
}
