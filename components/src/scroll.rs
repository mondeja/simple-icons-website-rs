use crate::grid::more_icons::GridIconsLoaderSignal;
use i18n::move_gettext;
use leptos::{ev::MouseEvent, *};

// TODO: When the user scrolls to the bottom manually, not using the
// scroll button, the button should be hidden also. This is a very strange
// case though, so it's not a priority.

#[component]
pub fn ScrollButton<H, T, C>(
    cx: Scope,
    /// Whether the button should be hidden or visible
    hidden: H,
    /// The title of the button
    title: T,
    /// The callback to be called when the button is clicked
    on_click: C,
    /// The SVG path of the icon
    svg_path: &'static str,
    /// Additional classes to be added to the button
    class: &'static str,
) -> impl IntoView
where
    H: Fn() -> bool + 'static,
    T: Fn() -> String + 'static,
    C: Fn(MouseEvent) + 'static,
{
    view! { cx,
        <button
            class=format!("scroll-button {}", class)
            class:hidden=hidden
            title=title
            on:click=on_click
        >
            <svg viewBox="0 0 24 24">
                <path d=svg_path />
            </svg>
        </button>
    }
}

#[component]
pub fn ScrollToHeaderButton(cx: Scope) -> impl IntoView {
    let grid_icons_loader = use_context::<GridIconsLoaderSignal>(cx).unwrap().0;

    view! { cx,
        <ScrollButton
            class="top-2"
            hidden=move|| grid_icons_loader().load_more_icons
            title=move_gettext!(cx, "Go to header")
            on_click=move|_|{
                let footer = document().query_selector("header").unwrap().unwrap();
                footer.scroll_into_view();
            }
            svg_path="M12 20c-4.41 0-8-3.59-8-8s3.59-8 8-8s8 3.59 8 8s-3.59 8-8 8m0 2c5.52 0 10-4.48 10-10S17.52 2 12 2S2 6.48 2 12s4.48 10 10 10zm-1-10v3c0 .55.45 1 1 1s1-.45 1-1v-3h1.79c.45 0 .67-.54.35-.85l-2.79-2.79c-.2-.2-.51-.2-.71 0l-2.79 2.79a.5.5 0 0 0 .36.85H11z"
        />
    }
}

#[component]
pub fn ScrollToFooterButton(cx: Scope) -> impl IntoView {
    let grid_icons_loader = use_context::<GridIconsLoaderSignal>(cx).unwrap().0;

    view! { cx,
        <ScrollButton
            class="bottom-2"
            hidden=move|| !grid_icons_loader().load_more_icons
            title=move_gettext!(cx, "Go to footer")
            on_click=move|_|{
                grid_icons_loader.update(|loader| loader.load_more_icons = false);
                let footer = document().query_selector("footer").unwrap().unwrap();
                footer.scroll_into_view();
            }
            svg_path="M12 4c4.41 0 8 3.59 8 8s-3.59 8-8 8s-8-3.59-8-8s3.59-8 8-8m0-2C6.48 2 2 6.48 2 12s4.48 10 10 10s10-4.48 10-10S17.52 2 12 2zm1 10V9c0-.55-.45-1-1-1s-1 .45-1 1v3H9.21c-.45 0-.67.54-.35.85l2.79 2.79c.2.2.51.2.71 0l2.79-2.79a.5.5 0 0 0-.35-.85H13z"
        />
    }
}
