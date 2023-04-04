//! Header at the top of the page

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
    /// Indicates if the extensions table is currently open
    pub extensions_open: bool,
    /// Indicates if the languages list is currently open
    pub languages_open: bool,
}

impl HeaderState {
    /// Toggles the menu open state
    pub fn toggle_menu(&mut self) {
        self.menu_open = !self.menu_open;
        self.languages_open = false;
        self.extensions_open = false;
    }

    /// Toggles the extensions table open state
    pub fn toggle_extensions(&mut self) {
        self.extensions_open = !self.extensions_open;
        if self.extensions_open && self.languages_open {
            self.languages_open = false;
        }
    }

    /// Toggles the languages list open state
    pub fn toggle_languages(&mut self) {
        self.languages_open = !self.languages_open;
        if self.languages_open && self.extensions_open {
            self.extensions_open = false;
        }
    }
}

#[derive(Copy, Clone)]
struct HeaderStateSignal(RwSignal<HeaderState>);

/// Header at the top of the page
#[component]
pub fn Header(cx: Scope) -> impl IntoView {
    let header_state = create_rw_signal(cx, HeaderState::default());
    provide_context(cx, HeaderStateSignal(header_state));

    view! { cx,
        <header class="flex flex-row columns-2 justify-between py-20">
            <HeaderTitle/>
            <HeaderMenu/>
        </header>
    }
}
