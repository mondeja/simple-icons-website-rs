mod footer;
mod icon_preview;
mod title;

use footer::*;
use icon_preview::*;
use title::*;

use leptos::*;

/// Icon grid item
///
/// Each icon displayed in the icons grid
#[component]
pub fn IconGridItem(
    cx: Scope,
    /// Icon slug
    slug: &'static str,
    /// Brand title
    title: &'static str,
    /// Brand color
    hex: &'static str,
    /// Hex color is relatively light, which means that the text should be dark
    hex_is_relatively_light: bool,
) -> impl IntoView {
    view! { cx,
        // The grid items are styled in item.css
        <li>
            <IconGridItemPreview slug=slug title=title />
            <IconGridItemTitle title=title slug=slug/>
            <IconGridItemFooter
                slug=slug
                title=title
                hex=hex
                hex_is_relatively_light=hex_is_relatively_light
            />
        </li>
    }
}
