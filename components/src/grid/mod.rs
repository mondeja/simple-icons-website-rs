mod ad;
mod item;

use crate::controls::layout::{Layout, LayoutSignal};
use crate::controls::order::OrderMode;
use crate::controls::search::search_icons;
use crate::grid::item::details::*;
use ad::*;
use config::CONFIG;
use item::*;
use leptos::*;
use macros::{get_number_of_icons, simple_icons_array};
use simple_icons::StaticSimpleIcon;

pub const ICONS: [StaticSimpleIcon;
    CONFIG.max_icons.unwrap_or(get_number_of_icons!())] = simple_icons_array!();

#[derive(Copy, Clone)]
pub struct DisplayedIconsSignal(pub RwSignal<Vec<StaticSimpleIcon>>);

#[derive(Copy, Clone)]
pub struct CurrentIconViewSignal(pub RwSignal<Option<StaticSimpleIcon>>);

pub fn initial_displayed_icons_from_search_value_and_order_mode(
    search_value: &str,
    order_mode: &OrderMode,
) -> Vec<StaticSimpleIcon> {
    if search_value.is_empty() {
        let mut displayed_icons: Vec<StaticSimpleIcon> = ICONS.to_vec();
        order_mode.sort_icons(&mut displayed_icons);
        displayed_icons
    } else {
        search_icons(search_value, order_mode)
    }
}

/// Icons grid
#[component]
pub fn GridIcons(cx: Scope) -> impl IntoView {
    let displayed_icons = use_context::<DisplayedIconsSignal>(cx).unwrap().0;

    view! { cx,
        {move || {
            displayed_icons().iter().map(|icon: &StaticSimpleIcon| {
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
