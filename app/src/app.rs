use components::controls::color_scheme::{
    initial_color_scheme_from_localstorage, ColorScheme, ColorSchemeSignal,
};
use components::controls::download::{
    initial_download_type_from_localstorage, DownloadTypeSignal,
};
use components::controls::layout::{
    initial_layout_from_localstorage, LayoutSignal,
};
use components::controls::order::{
    initial_order_mode_from_localstorage, OrderModeSignal,
};
use components::controls::search::{initial_search_value, SearchValueSignal};
use components::grid::{IconsGrid, IconsGridSignal};
use components::*;
use i18n::{gettext, move_gettext};
use i18n::{LocaleState, LocaleStateSignal};
use leptos::*;
use leptos_meta::{
    provide_meta_context, Link, LinkProps, Meta, MetaProps, Title, TitleProps,
};
use macros::get_number_of_icons;

macro_rules! url {
    () => {
        "https://simpleicons.org"
    };
}

/// Number of icons available in the library
static NUMBER_OF_ICONS: usize = get_number_of_icons!();

/// Title of the page
static TITLE: &str = "Simple Icons";

/// URL of the website
static URL: &str = url!();

/// URL of Simple Icons logo
static LOGO_URL: &str = concat!(url!(), "/icons/simpleicons.svg");

/// The main application component
#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    // Localization context
    provide_context(
        cx,
        LocaleStateSignal(create_rw_signal(cx, LocaleState::new())),
    );

    let description = move_gettext!(
        cx,
        "{} free {} icons for popular brands",
        NUMBER_OF_ICONS.to_string().as_str(),
        &gettext!(cx, "SVG")
    );

    view! { cx,
        <Title text=TITLE/>
        <Meta charset="utf-8"/>
        <Meta
            content="width=device-width, initial-scale=1, shrink-to-fit=no"
            name="viewport"
        />
        <Meta name="description" content=description/>
        <Link rel="apple-touch-icon" href="/apple-touch-icon.png" />
        <Link
            rel="search"
            type_="application/opensearchdescription+xml"
            title=TITLE
            href="/opensearch.xml"
        />
        <Link rel="license" href="/license.txt" />
        <Link rel="canonical" href=URL />

        // Fonts from Google API
        <Link rel="preconnect" href="https://fonts.gstatic.com" />
        <Link
            rel="stylesheet"
            href="https://fonts.googleapis.com/css2?family=Open+Sans:wght@400&family=Roboto+Mono:wght@400;600"
        />

        <MetaOpenGraph description=description/>
        <MetaTwitter description=description/>
        <Meta name="msvalidate.01" content="14319924BC1F00DC15EF0EAA29E72404"/>
        <Meta name="yandex-verification" content="8b467a0b98aa2725"/>

        <LdJSONMetadata/>

        <AppBody/>
    }
}

/// Open graph meta tags
#[component]
pub fn MetaOpenGraph<F>(
    cx: Scope,
    /// Site description
    description: F,
) -> impl IntoView
where
    F: Fn() -> String + 'static,
{
    view! { cx,
        <Meta name="og:type" content="website"/>
        <Meta name="og:title" content=TITLE/>
        <Meta name="og:description" content=description/>
        <Meta name="og:url" content=URL/>
        <Meta name="og:site_name" content=TITLE/>
        // Note that the image is linked for Trunk at index.html
        <Meta name="og:image" content="/og.png"/>
    }
}

/// Twitter meta tags
#[component]
pub fn MetaTwitter<F>(
    cx: Scope,
    /// Site description
    description: F,
) -> impl IntoView
where
    F: Fn() -> String + 'static,
{
    view! { cx,
        <Meta name="twitter:card" content="summary_large_image"/>
        <Meta name="twitter:title" content=TITLE/>
        <Meta name="twitter:description" content=description/>
        <Meta name="twitter:url" content=URL/>
        <Meta name="twitter:image:src" content="/og.png"/>
    }
}

/// JSON-LD metadata
/// See https://developers.google.com/search/docs/data-types/logo
#[component]
pub fn LdJSONMetadata(cx: Scope) -> impl IntoView {
    view! { cx,
        <script type="application/ld+json">
            {{
                serde_json::json!({
                    "@context": "https://schema.org",
                    "@type": "Organization",
                    "name": TITLE,
                    "url": URL,
                    "logo": LOGO_URL,
                    "image": LOGO_URL,
                    "potentialAction": {
                        "@type": "SearchAction",
                        "target": URL.to_owned() + "/?q={search-term}",
                        "query-input": "required name=search-term",
                    },
                }).to_string()
            }}
        </script>
    }
}

/// Body of the page
///
/// Initializes the top level contexts for the application in order
/// to be used by the child components.
#[component]
pub fn AppBody(cx: Scope) -> impl IntoView {
    // Color scheme context
    provide_context(
        cx,
        ColorSchemeSignal(create_rw_signal(
            cx,
            initial_color_scheme_from_localstorage(),
        )),
    );
    let color_scheme = use_context::<ColorSchemeSignal>(cx).unwrap().0;

    // Donwload type context
    provide_context(
        cx,
        DownloadTypeSignal(create_rw_signal(
            cx,
            initial_download_type_from_localstorage(),
        )),
    );

    // Order mode context
    let initial_order_mode = initial_order_mode_from_localstorage();
    provide_context(
        cx,
        OrderModeSignal(create_rw_signal(cx, initial_order_mode)),
    );

    // Search context
    let initial_search_value = initial_search_value();
    provide_context(
        cx,
        SearchValueSignal(create_rw_signal(cx, initial_search_value.clone())),
    );

    // Layout context
    provide_context(
        cx,
        LayoutSignal(create_rw_signal(cx, initial_layout_from_localstorage())),
    );

    // Displayed icons context
    provide_context(
        cx,
        IconsGridSignal(create_rw_signal(
            cx,
            IconsGrid::new(&initial_search_value, &initial_order_mode),
        )),
    );

    view! { cx,
        <body class=move||{
            let mut class = concat!(
                "font-mono flex flex-col px-6 md:px-12 min-h-[100vh]",
                " bg-custom-background-color",
                " text-custom-text-default-color"
            ).to_string();
            match color_scheme() {
                ColorScheme::Dark => class.push_str(" dark"),
                ColorScheme::Light => class.push_str(" light"),
                ColorScheme::System => {
                    if web_sys::window().unwrap().match_media("(prefers-color-scheme: dark)").unwrap().unwrap().matches() {
                        class.push_str(" dark");
                    } else {
                        class.push_str(" light");
                    }
                },
            }
            class
        }>
            <SVGDefsDefinition/>
            <Header/>
            <main>
                <Controls/>
                <Grid/>
            </main>
            <Footer/>
        </body>
    }
}
