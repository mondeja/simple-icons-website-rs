use crate::meta::*;
use components::controls::color_scheme::{
    provide_color_scheme_context, ColorScheme,
};
use components::controls::download::provide_download_type_context;
use components::controls::layout::provide_layout_context;
use components::controls::order::provide_order_mode_context;
use components::controls::search::provide_search_context;
use components::copy::*;
use components::grid::provide_icons_grid_contexts;
use components::header::nav::language_selector::provide_language_context;
use components::*;
use i18n::{gettext, move_gettext};
use leptos::*;
use leptos_meta::*;
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
    view! { cx,
        <Router>
            <Routes>
                <Route
                    path="/"
                    view=move |cx| {
                        view! { cx, <Index/> }
                    }
                />
            </Routes>
        </Router>
    }
}

#[component]
fn Index(cx: Scope) -> impl IntoView {
    provide_language_context(cx);
    provide_meta_context(cx);
    let color_scheme = provide_color_scheme_context(cx).0;

    let initial_search_value = provide_search_context(cx);
    let initial_order_mode =
        provide_order_mode_context(cx, &initial_search_value);
    provide_download_type_context(cx);
    provide_layout_context(cx);
    provide_icons_grid_contexts(cx, &initial_search_value, &initial_order_mode);

    let description = move_gettext!(
        cx,
        "{} free {} icons for popular brands",
        NUMBER_OF_ICONS.to_string().as_str(),
        &gettext!(cx, "SVG")
    );

    view! { cx,
        <Title text=TITLE/>
        <Meta charset="utf-8"/>
        <Meta content="width=device-width, initial-scale=1, shrink-to-fit=no" name="viewport"/>
        <Meta name="description" content=description/>
        <Link rel="apple-touch-icon" href="./apple-touch-icon.png"/>
        <Link
            rel="search"
            type_="application/opensearchdescription+xml"
            title=TITLE
            href="./opensearch.xml"
        />
        <Link rel="license" href="./license.txt"/>
        <Link rel="canonical" href=URL/>
        <Link rel="preconnect" href="https://fonts.gstatic.com"/>
        <Link
            rel="stylesheet"
            href="https://fonts.googleapis.com/css2?family=Open+Sans:wght@400&family=Roboto+Mono:wght@400;600"
        />
        <MetaOpenGraph description=description/>
        <MetaTwitter description=description/>
        <Meta name="msvalidate.01" content="14319924BC1F00DC15EF0EAA29E72404"/>
        <Meta name="yandex-verification" content="8b467a0b98aa2725"/>
        <LdJSONMetadata/>
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
            <SVGDefsDefinition/>
            <CopyInput/>
            <Header/>
            <ScrollToHeaderButton/>
            <main>
                <Controls/>
                <Grid/>
            </main>
            <Footer/>
            <ScrollToFooterButton/>
        </body>
    }
}
