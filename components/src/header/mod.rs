mod menu;
mod title;

use leptos::*;
use menu::*;
use title::*;

/// Header at the top of the page
#[component]
pub fn Header(
    cx: Scope,
    /// Number of icons available in the library
    number_of_icons: usize,
) -> impl IntoView {
    view! { cx,
        <header class="flex flex-row columns-2">
            <HeaderTitle number_of_icons=number_of_icons/>
            <HeaderMenu/>
        </header>
    }
}
