mod menu;
mod title;

use leptos::*;
use menu::*;
use title::*;

/// State of the header
#[derive(Copy, Clone, Default)]
pub struct HeaderState {
    /// Indicates if the menu is currently open (only used on mobile screens)
    pub menu_open: bool,
}

#[derive(Copy, Clone)]
struct HeaderStateSignal(RwSignal<HeaderState>);

/// Header at the top of the page
#[component]
pub fn Header(cx: Scope) -> impl IntoView {
    let header_state = create_rw_signal(cx, HeaderState::default());
    provide_context(cx, HeaderStateSignal(header_state));

    view! { cx,
        <header class="flex flex-row columns-2 justify-between">
            <HeaderTitle/>
            <HeaderMenu/>
        </header>
    }
}
