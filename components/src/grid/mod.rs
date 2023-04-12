mod ad;
mod item;
pub mod more_icons;

use crate::controls::layout::{Layout, LayoutSignal};
use crate::controls::search::{
    fire_on_search_event, search_icons_and_returns_first_page,
};
use crate::grid::item::details::*;
pub use crate::grid::more_icons::*;
use crate::order::{sort_icons, OrderModeVariant};
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
    pub static ref FIRST_LOADED_ICONS: Vec<StaticSimpleIcon> =
        ICONS[..(CONFIG.icons_per_page as usize)].to_vec();
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
    pub fn new(search_value: &str, order_mode: &OrderModeVariant) -> Self {
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

    pub fn reset(&mut self) {
        self.icons = ICONS.to_vec();
        self.loaded_icons = FIRST_LOADED_ICONS.to_vec();
    }

    pub fn set_icons(&mut self, icons: Vec<StaticSimpleIcon>) {
        self.icons = icons;
    }

    pub fn set_loaded_icons(&mut self, loaded_icons: &Vec<StaticSimpleIcon>) {
        self.loaded_icons = loaded_icons.clone();
    }

    pub fn load_next_icons(&mut self) {
        let next_icons: Vec<StaticSimpleIcon> = self
            .icons
            .iter()
            .skip(self.loaded_icons.len())
            .take(CONFIG.icons_per_page as usize)
            .cloned()
            .collect();
        self.loaded_icons.extend(next_icons);
    }

    pub fn set_order_mode(&mut self, order_mode: &OrderModeVariant) {
        match order_mode {
            &OrderModeVariant::Alphabetic | &OrderModeVariant::Color => {
                sort_icons(order_mode, &mut self.icons);
                self.loaded_icons = vec![];
                self.load_next_icons()
            }
            &OrderModeVariant::SearchMatch => {
                // Fire a search event to update the grid
                fire_on_search_event();
            }
        }
    }
}

/// Signal to control the icons grid
#[derive(Copy, Clone)]
pub struct IconsGridSignal(pub RwSignal<IconsGrid>);

/// Signal to control the current detail view modal of icons
#[derive(Copy, Clone)]
pub struct CurrentIconViewSignal(pub RwSignal<Option<StaticSimpleIcon>>);

pub fn initial_icons_from_search_value_and_order_mode(
    search_value: &str,
    order_mode: &OrderModeVariant,
) -> (Vec<StaticSimpleIcon>, Vec<StaticSimpleIcon>) {
    if search_value.is_empty() {
        let mut icons: Vec<StaticSimpleIcon> = ICONS.to_vec();
        if order_mode != &OrderModeVariant::Alphabetic {
            // Alphabetical is the default order of the icons in the static array
            sort_icons(order_mode, &mut icons);
        }
        let mut loaded_icons =
            Vec::with_capacity(CONFIG.icons_per_page as usize);
        loaded_icons.extend(icons.iter().take(CONFIG.icons_per_page as usize));

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
            icons_grid()
                .loaded_icons
                .iter()
                .map(|icon: &StaticSimpleIcon| {
                    view! { cx, <IconGridItem icon=*icon/> }
                })
                .collect::<Vec<_>>()
        }}
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
        <ul class:layout-compact=move || layout() == Layout::Compact>
            <CarbonAdsAdGridItem/>
            <GridIcons/>
        </ul>
        <LoadMoreIconsButton/>
    }
}
