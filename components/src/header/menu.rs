use crate::header::{HeaderState, HeaderStateSignal};
use leptos::*;
use macros::{get_simple_icons_3rd_party_extensions, simple_icon_svg_path};
use types::SimpleIconsExtension;

static MENU_LINKS: &[(&str, &str, &str)] = &[
    (
        "Main Repository",
        "https://github.com/simple-icons/simple-icons",
        simple_icon_svg_path!("github.svg"),
    ),
    (
        "npm",
        "https://www.npmjs.com/package/simple-icons",
        simple_icon_svg_path!("npm.svg"),
    ),
    (
        "Packagist",
        "https://packagist.org/packages/simple-icons/simple-icons",
        simple_icon_svg_path!("packagist.svg"),
    ),
    (
        "jsDelivr (Content Delivery Network)",
        "https://www.jsdelivr.com/package/npm/simple-icons",
        simple_icon_svg_path!("jsdelivr.svg"),
    ),
    (
        "UNPKG (Content Delivery Network)",
        "https://unpkg.com/browse/simple-icons/",
        // UNPKG icon is not available in simple-icons
        // Requested at https://github.com/simple-icons/simple-icons/issues/8475
        "M12 0A12 12 0 0 0 0 12a12 12 0 0 0 12 12 12 12 0 0 0 12-12A12 12 0 0 0 12 0zm0 2.002A10.009 10.009 0 0 1 22.009 12.01 10.009 10.009 0 0 1 12 22.02 10.009 10.009 0 0 1 1.991 12.01 10.009 10.009 0 0 1 12 2.003zm2.141 3.928-.031 4.047-.034 4.046c-.73 2.109-3.414 2.109-3.992.007-.075-.31-.118-1.798-.118-4.282v-3.81H6.84l.043 4.142c.054 4.475.085 4.742.662 5.91.353.684 1.21 1.444 1.97 1.723 1.391.524 3.682.524 5.073 0 .75-.279 1.553-.976 1.938-1.681.589-1.103.654-1.627.654-6.016l.01-4.014-3.04-.065z",
    ),
    (
        "Open Collective",
        "https://opencollective.com/simple-icons",
        simple_icon_svg_path!("opencollective.svg"),
    ),
    (
        "Legal Disclaimer",
        "https://github.com/simple-icons/simple-icons/blob/master/DISCLAIMER.md",
        "m23.9 9.7-3.54-7.89-.005-.01a.542.542 0 0 0-.041-.076l-.014-.018a.533.533 0 0 0-.122-.122l-.015-.011a.528.528 0 0 0-.08-.044l-.024-.009a.527.527 0 0 0-.067-.02l-.028-.007a.524.524 0 0 0-.096-.01h-6.85c-1.02-1.52-1.02-1.54-2 0h-6.86a.543.543 0 0 0-.096.01l-.028.007a.516.516 0 0 0-.067.02l-.024.01a.537.537 0 0 0-.08.043l-.015.011a.51.51 0 0 0-.057.047l-.02.02a.543.543 0 0 0-.045.055l-.014.018a.522.522 0 0 0-.041.075l-.005.01v.001L.116 9.72a.531.531 0 0 0-.096.304c0 2.28 1.86 4.14 4.14 4.14s4.14-1.86 4.14-4.14a.53.53 0 0 0-.096-.304l-3.25-6.37 6.07-.023v18.2c-2.55.294-7.01.381-7 2.5h16c0-2.03-4.48-2.27-7-2.5v-18.1l5.69-.02-2.92 6.49c0 .002 0 .003-.002.005l-.006.018a.545.545 0 0 0-.023.075l-.005.02a.524.524 0 0 0-.01.092v.008c0 2.28 1.86 4.14 4.14 4.14 2.28 0 4.14-1.86 4.14-4.14a.528.528 0 0 0-.12-.332z",
    ),
];

/// Header menu
///
/// Menu with:
///
/// - Links to the different resources of the Simple Icons ecosystem built by [`HeaderMenuLink`].
/// - Button to open the menu on mobile devices built by [`HeaderMenuButton`].
/// - Button to open third party extensions table built by [`HeaderMenuButton`].
#[component]
pub fn HeaderMenu(cx: Scope) -> impl IntoView {
    let header_state = use_context::<HeaderStateSignal>(cx).unwrap().0;

    view! { cx,
        <ul class="flex space-x-3">
            { MENU_LINKS.iter().map(|(title, href, svg_path)| {
                view! { cx,
                    <HeaderMenuLink
                        title=title
                        href=href
                        svg_path=svg_path
                        class=move || {
                            if header_state.get().menu_open {
                                "block".to_string()
                            } else {
                                "hidden lg:block".to_string()
                            }
                        }/>
                }
            }).collect::<Vec<_>>()}

            // Burger button (only shown on mobile screens)
            <HeaderMenuBurgerButton />

            <ThirdPartyExtensions />

            // Close menu button (only shown on mobile screens)
            <HeaderMenuCloseButton />

            // TODO: language button
        </ul>
    }
}

/// Header menu link
///
/// Each link of the header menu
#[component]
pub fn HeaderMenuLink<F>(
    cx: Scope,
    /// Title of the link
    title: &'static str,
    /// URL of the link
    href: &'static str,
    /// SVG path of the icon
    svg_path: &'static str,
    /// Additional classes to add to the link
    class: F,
) -> impl IntoView
where
    F: Fn() -> String + 'static + Clone,
{
    view! { cx,
        <li class=move || format!("w-12 h-12 {}", class())>
            <a title=title href=href>
                <svg role="img" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                    <path d=svg_path/>
                </svg>
            </a>
        </li>
    }
}

/// Header menu button
///
/// Each button of the header menu that is not a link
#[component]
pub fn HeaderMenuButton<F>(
    cx: Scope,
    /// Additional classes to add to the button
    class: F,
    /// Title of the button
    title: &'static str,
    /// SVG path of the icon
    svg_path: &'static str,
) -> impl IntoView
where
    F: Fn() -> String + 'static + Clone,
{
    view! { cx,
        <li title=title class=move || {
            format!("w-12 h-12 cursor-pointer {}", class())
        }>
            <svg role="img" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                <path d=svg_path/>
            </svg>
        </li>
    }
}

/// Header menu burger button
///
/// Button to open the menu on mobile devices
#[component]
pub fn HeaderMenuBurgerButton(cx: Scope) -> impl IntoView {
    let header_state = use_context::<HeaderStateSignal>(cx).unwrap().0;

    view! { cx,
        <HeaderMenuButton
            // When the burger button is clicked, open the menu
            on:click=move |_| {
                header_state.update(
                    |state: &mut HeaderState| state.menu_open = true
                );
            }
            title="Menu"
            class=move || {
                // If the menu is open, hide the burger button
                if !header_state.get().menu_open {
                    "block lg:hidden".to_string()
                } else {
                    "hidden".to_string()
                }
            }
            svg_path="M1.412 3.53A1.412 1.412 0 0 0 0 4.94a1.412 1.412 0 0 0 1.412 1.412h21.176A1.412 1.412 0 0 0 24 4.94a1.412 1.412 0 0 0-1.412-1.412Zm0 7.058A1.412 1.412 0 0 0 0 12a1.412 1.412 0 0 0 1.412 1.412h21.176A1.412 1.412 0 0 0 24 12a1.412 1.412 0 0 0-1.412-1.412Zm0 7.06A1.412 1.412 0 0 0 0 19.057a1.412 1.412 0 0 0 1.412 1.413h21.176A1.412 1.412 0 0 0 24 19.059a1.412 1.412 0 0 0-1.412-1.412Z"
        />
    }
}

/// Button to close the menu on mobile devices
#[component]
pub fn HeaderMenuCloseButton(cx: Scope) -> impl IntoView {
    let header_state = use_context::<HeaderStateSignal>(cx).unwrap().0;

    view! { cx,
        <HeaderMenuButton
            title="Close menu"
            on:click=move |_| {
                header_state.update(
                    |state: &mut HeaderState| state.menu_open = false
                );
            }
            class=move || {
                // If the menu is open, show the close menu button
                if header_state.get().menu_open {
                    "block".to_string()
                } else {
                    "hidden".to_string()
                }
            }
            svg_path="M12 10.586l5.657-5.657a1 1 0 1 1 1.414 1.414L13.414 12l5.657 5.657a1 1 0 0 1-1.414 1.414L12 13.414l-5.657 5.657a1 1 0 0 1-1.414-1.414L10.586 12 4.93 6.343a1 1 0 0 1 1.414-1.414L12 10.586z"
        />
    }
}

/// Third party extensions button and table
#[component]
pub fn ThirdPartyExtensions(cx: Scope) -> impl IntoView {
    let header_state = use_context::<HeaderStateSignal>(cx).unwrap().0;
    let extensions: &[SimpleIconsExtension] =
        get_simple_icons_3rd_party_extensions!();

    view! { cx,
        <HeaderMenuButton
            title="Third party extensions"
            class=move || {
                if header_state.get().menu_open {
                    "block".to_string()
                } else {
                    "hidden lg:block".to_string()
                }
            }
            svg_path="M16.513 23.996a.9.9 0 0 0 .885-.907v-4.972c.303-2.68 1.42-1.884 2.734-1.055 3.178 2.003 5.29-3.266 2.72-4.891-2.015-1.276-2.888.917-4.364.69-.57-.088-.967-.72-1.092-1.68V7.59c0-.5-.398-.907-.885-.907h-4.064c-3.355-.436-.377-2.339-.377-4.11C12.072 1.152 10.816 0 9.267 0 7.721 0 6.301 1.152 6.301 2.573c0 1.67 3.082 3.674-.32 4.11H.884A.898.898 0 0 0 0 7.59v3.583c.26 1.528 1.268 1.882 2.559.874.435-.341 1.17-.738 1.7-.738 1.385 0 2.51 1.285 2.51 2.871s-1.123 3.221-2.51 3.221c-.493 0-.954-.164-1.345-.45 0 .121-2.422-2.232-2.914.648v5.494c0 .5.398.907.885.907 2.728 0 5.453 0 8.18-.002.107-.525-.243-1.125-.571-1.646-2.582-4.1 7.463-4.508 4.88.128-.126.228-.253.45-.35.666-.124.27-.206.599-.188.852z"
        />
    }
}
