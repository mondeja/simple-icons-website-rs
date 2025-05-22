mod button;
pub mod language_selector;
mod third_party_extensions;

use super::HeaderStateSignal;
use button::{HeaderMenuButton, HeaderMenuLink};
use icondata::{ChCross, ChMenuHamburger};
use language_selector::LanguageSelector;
use leptos::prelude::*;
use leptos_fluent::move_tr;
use simple_icons_macros::get_simple_icon_svg_path;
use third_party_extensions::ThirdPartyExtensions;

static LEGAL_DISCLAIMER_SVG_PATH: &str = "m23.9 9.7-3.54-7.89-.005-.01a.542.542 0 0 0-.041-.076l-.014-.018a.533.533 0 0 0-.122-.122l-.015-.011a.528.528 0 0 0-.08-.044l-.024-.009a.527.527 0 0 0-.067-.02l-.028-.007a.524.524 0 0 0-.096-.01h-6.85c-1.02-1.52-1.02-1.54-2 0h-6.86a.543.543 0 0 0-.096.01l-.028.007a.516.516 0 0 0-.067.02l-.024.01a.537.537 0 0 0-.08.043l-.015.011a.51.51 0 0 0-.057.047l-.02.02a.543.543 0 0 0-.045.055l-.014.018a.522.522 0 0 0-.041.075l-.005.01v.001L.116 9.72a.531.531 0 0 0-.096.304c0 2.28 1.86 4.14 4.14 4.14s4.14-1.86 4.14-4.14a.53.53 0 0 0-.096-.304l-3.25-6.37 6.07-.023v18.2c-2.55.294-7.01.381-7 2.5h16c0-2.03-4.48-2.27-7-2.5v-18.1l5.69-.02-2.92 6.49c0 .002 0 .003-.002.005l-.006.018a.545.545 0 0 0-.023.075l-.005.02a.524.524 0 0 0-.01.092v.008c0 2.28 1.86 4.14 4.14 4.14 2.28 0 4.14-1.86 4.14-4.14a.528.528 0 0 0-.12-.332z";

/// Header menu
#[component]
pub fn HeaderMenu() -> impl IntoView {
    view! {
        <nav>
            <ul>
                <HeaderMenuLink
                    title=move_tr!("main-repository")
                    href="https://github.com/simple-icons/simple-icons"
                    icon=get_simple_icon_svg_path!("github")
                />
                <HeaderMenuLink
                    title="npm"
                    href="https://www.npmjs.com/package/simple-icons"
                    icon=get_simple_icon_svg_path!("npm")
                />
                <HeaderMenuLink
                    title="Packagist"
                    href="https://packagist.org/packages/simple-icons/simple-icons"
                    icon=get_simple_icon_svg_path!("packagist")
                />
                <HeaderMenuLink
                    title=move_tr!("jsdelivr")
                    href="https://www.jsdelivr.com/package/npm/simple-icons"
                    icon=get_simple_icon_svg_path!("jsdelivr")
                />
                <HeaderMenuLink
                    title=move_tr!("unpkg")
                    href="https://unpkg.com/browse/simple-icons/"
                    icon=get_simple_icon_svg_path!("unpkg")
                />
                <HeaderMenuLink
                    title=move_tr!("discord")
                    href="https://discord.gg/vUXFa7t5xJ"
                    icon=get_simple_icon_svg_path!("discord")
                />
                <HeaderMenuLink
                    title=move_tr!("open-collective")
                    href="https://opencollective.com/simple-icons"
                    icon=get_simple_icon_svg_path!("opencollective")
                />
                <HeaderMenuLink
                    title=move_tr!("legal-disclaimer")
                    href="https://github.com/simple-icons/simple-icons/blob/master/DISCLAIMER.md"
                    icon=LEGAL_DISCLAIMER_SVG_PATH
                />
                <ThirdPartyExtensions />
                <LanguageSelector />
            </ul>
            <ul>
                <HeaderMenuBurgerButton />
                <HeaderMenuCloseButton />
            </ul>
        </nav>
    }
}

/// Header menu burger button
///
/// Button to open the menu on mobile devices
#[component]
pub fn HeaderMenuBurgerButton() -> impl IntoView {
    let header_state = expect_context::<HeaderStateSignal>().0;

    view! {
        <HeaderMenuButton
            on:click=move |_| header_state.update(|state| state.toggle_menu())
            icon=ChMenuHamburger
            attr:class=move || if header_state().menu_open { "hidden" } else { "block lg:hidden" }
            attr:title=move_tr!("open-menu")
        />
    }
}

/// Button to close the menu on mobile devices
#[component]
pub fn HeaderMenuCloseButton() -> impl IntoView {
    let header_state = expect_context::<HeaderStateSignal>().0;

    view! {
        <HeaderMenuButton
            attr:class=move || if header_state().menu_open { "block" } else { "hidden" }
            icon=ChCross
            on:click=move |_| header_state.update(|state| state.toggle_menu())
            attr:title=move_tr!("close-menu")
        />
    }
}
