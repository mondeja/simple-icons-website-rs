mod ad;
mod item;

use crate::controls::layout::{Layout, LayoutSignal};
use ad::*;
use config::CONFIG;
use item::*;
use leptos::*;
use macros::{get_number_of_icons, simple_icons_array};
use simple_icons::FullStaticSimpleIcon;

pub const ICONS: [FullStaticSimpleIcon;
    CONFIG.max_icons.unwrap_or(get_number_of_icons!())] = simple_icons_array!();

#[derive(Copy, Clone)]
pub struct DisplayedIconsSignal(pub RwSignal<Vec<FullStaticSimpleIcon>>);

/// Icons grid
#[component]
pub fn GridIcons(cx: Scope) -> impl IntoView {
    let displayed_icons = use_context::<DisplayedIconsSignal>(cx).unwrap().0;

    view! { cx,
        {move || {
            displayed_icons().iter().map(|icon: &FullStaticSimpleIcon| {
                view!{
                    cx,
                    <IconGridItem
                        slug=&icon.slug
                        title=&icon.title
                        hex=&icon.hex
                        hex_is_relatively_light=icon.hex_is_relatively_light
                    />
                }
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

    view! { cx,
        <ul class:layout-compact=move||layout() == Layout::Compact>
            <CarbonAdsAdGridItem/>
            <GridIcons />
        </ul>
    }
}
