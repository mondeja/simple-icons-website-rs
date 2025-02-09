//! Header at the top of the page

pub mod nav;
mod title;

use leptos::prelude::*;
use nav::HeaderMenu;
use title::HeaderTitle;

/// State of the header
#[derive(Copy, Clone, Default, Debug)]
pub struct HeaderState {
    /// Indicates if the menu is currently open (only used on tablet and mobile screens)
    pub menu_open: bool,
}

impl HeaderState {
    /// Toggles the menu open state
    pub fn toggle_menu(&mut self) {
        self.menu_open = !self.menu_open;
    }
}

#[derive(Copy, Clone)]
pub struct HeaderStateSignal(pub RwSignal<HeaderState>);

/// Header at the top of the page
#[component]
pub fn Header() -> impl IntoView {
    let header_state = RwSignal::new(HeaderState::default());
    provide_context(HeaderStateSignal(header_state));

    view! {
        <header>
            <Show when=move || !header_state().menu_open>
                <HeaderTitle />
            </Show>
            <HeaderMenu />
        </header>
    }
}
