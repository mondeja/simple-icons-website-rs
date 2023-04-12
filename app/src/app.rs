use crate::page::*;
use components::controls::color_scheme::{
    provide_color_scheme_context, ColorScheme,
};
use components::controls::download::provide_download_type_context;
use components::controls::layout::provide_layout_context;
use components::controls::order::provide_order_mode_context;
use components::controls::search::provide_search_context;
use components::grid::provide_icons_grid_contexts;
use components::header::nav::language_selector::provide_language_context;
use components::*;
use leptos::*;
use leptos_meta::provide_meta_context;
use leptos_router::{
    Route, RouteProps, Router, RouterProps, Routes, RoutesProps,
};
use macros::get_number_of_icons;

macro_rules! url {
    () => {
        "https://simpleicons.org"
    };
}

/// Number of icons available in the library
pub static NUMBER_OF_ICONS: usize = get_number_of_icons!();

/// Title of the page
pub static TITLE: &str = "Simple Icons";

/// URL of the website
pub static URL: &str = url!();

/// URL of Simple Icons logo
pub static LOGO_URL: &str = concat!(url!(), "/icons/simpleicons.svg");

/// The main application component
#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! { cx,
        <Router>
            <Routes>
                <Route
                    path="/"
                    view=move |cx| {
                        view! { cx, <AppIndex/> }
                    }
                />
            </Routes>
        </Router>
    }
}

#[component]
fn AppIndex(cx: Scope) -> impl IntoView {
    provide_language_context(cx);

    view! { cx,
        <AppPage>
            <AppBody/>
        </AppPage>
    }
}

/// Body of the page
///
/// Initializes the color scheme context
#[component]
pub fn AppBody(cx: Scope) -> impl IntoView {
    let color_scheme = provide_color_scheme_context(cx).0;

    view! { cx,
        <body class=move || match color_scheme() {
            ColorScheme::Dark => "dark",
            ColorScheme::Light => "light",
            ColorScheme::System => {
                if web_sys::window()
                    .unwrap()
                    .match_media("(prefers-color-scheme: dark)")
                    .unwrap()
                    .unwrap()
                    .matches()
                {
                    "dark"
                } else {
                    "light"
                }
            }
        }>
            <AppBodyContent/>
        </body>
    }
}

/// Content of the body of the page
///
/// Initializes the top level contexts for the application in order
/// to be used by the child components.
#[component]
fn AppBodyContent(cx: Scope) -> impl IntoView {
    let initial_search_value = provide_search_context(cx);
    let initial_order_mode =
        provide_order_mode_context(cx, &initial_search_value);
    provide_download_type_context(cx);
    provide_layout_context(cx);
    provide_icons_grid_contexts(cx, &initial_search_value, &initial_order_mode);

    view! { cx,
        <SVGDefsDefinition/>
        <Header/>
        <ScrollToHeaderButton/>
        <main>
            <Controls/>
            <Grid/>
        </main>
        <Footer/>
        <ScrollToFooterButton/>
    }
}
