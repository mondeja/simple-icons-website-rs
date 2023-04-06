mod ad;
mod item;

use ad::*;
use item::*;
use leptos::*;

use crate::controls::ControlsStateSignal;
use simple_icons::FullStaticSimpleIcon;

/// Icons grid
#[component]
pub fn GridIcons(cx: Scope) -> impl IntoView {
    let controls_state = use_context::<ControlsStateSignal>(cx).unwrap().0;

    view! { cx,
        {move || {
            controls_state().shown_icons.iter().map(|icon: &FullStaticSimpleIcon| {
                view!{
                    cx,
                    <IconGridItem
                        slug={&icon.slug}
                        title={&icon.title}
                        hex={&icon.hex}
                        order_alpha_index={icon.order_alpha}
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
    view! { cx,
        <ul
            class="relative grid gap-3 mt-8"
            style="grid-template-columns: repeat(auto-fill, minmax(10rem, 1fr));"
        >
            <CarbonAdsAdGridItem/>
            <GridIcons />
        </ul>
    }
}
