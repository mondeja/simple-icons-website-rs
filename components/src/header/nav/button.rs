use crate::header::HeaderStateSignal;
use leptos::*;

/// Header menu link
///
/// Each link of the header menu
#[component]
pub fn HeaderMenuLink<T>(
    cx: Scope,
    /// Title of the link
    title: T,
    /// URL of the link
    href: &'static str,
    /// SVG path of the icon
    svg_path: &'static str,
) -> impl IntoView
where
    T: Fn() -> String + 'static + Clone,
{
    let header_state = use_context::<HeaderStateSignal>(cx).unwrap().0;

    view! { cx,
        <li class=move || if header_state.get().menu_open {
            "block".to_string()
        } else {
            "hidden lg:block".to_string()
        }>
            <a title=title href=href>
                <svg role="img" viewBox="0 0 24 24">
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
pub fn HeaderMenuButton<C, T>(
    cx: Scope,
    /// Additional classes to add to the button
    additional_classes: C,
    /// Title of the button
    title: T,
    /// SVG path of the icon
    svg_path: &'static str,
) -> impl IntoView
where
    C: Fn() -> String + 'static + Clone,
    T: Fn() -> String + 'static + Clone,
{
    view! { cx,
        <li title=title class=additional_classes>
            <svg role="img" viewBox="0 0 24 24">
                <path d=svg_path/>
            </svg>
        </li>
    }
}
