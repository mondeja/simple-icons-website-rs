pub mod details;
mod footer;
mod icon_preview;
mod links;
mod title;

use footer::*;
use icon_preview::*;
use links::*;
use simple_icons::StaticSimpleIcon;
use title::*;

use leptos::*;

/// Icon grid item
///
/// Each icon displayed in the icons grid
#[component]
pub fn IconGridItem(
    cx: Scope,
    /// Icon
    icon: StaticSimpleIcon,
) -> impl IntoView {
    view! { cx,
        // The grid items are styled in item.css
        <li>
            <IconGridItemPreview slug=icon.slug title=icon.title />
            <IconGridItemLinks
                guidelines_url=icon.guidelines_url
                license_url=icon.license_url
                license_type=icon.license_type
            />
            <IconGridItemTitle title=icon.title slug=icon.slug/>
            <IconGridItemFooter icon=icon/>
        </li>
    }
}
