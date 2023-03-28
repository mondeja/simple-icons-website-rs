use components::*;
use lazy_static::lazy_static;
use leptos::*;
use leptos_meta::*;
use macros::get_number_of_icons;

/// Number of icons available in the library
pub static NUMBER_OF_ICONS: usize = get_number_of_icons!();

/// Title of the page
pub static TITLE: &str = "Simple Icons";

/// URL of the website
pub static URL: &str = "https://simpleicons.org";

lazy_static! {
    /// Description of the website
    pub static ref DESCRIPTION: String = format!("{} free SVG icons for popular brands", NUMBER_OF_ICONS);
}

/// The main application component
#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! { cx,
        <Title text=TITLE/>
        <Meta charset="utf-8"/>
        <Meta
            content="width=device-width, initial-scale=1, shrink-to-fit=no"
            name="viewport"
        />
        <Meta name="description" content=DESCRIPTION.as_str()/>
        <Link rel="apple-touch-icon" href="/apple-touch-icon.png" />
        <Link
            rel="search"
            type_="application/opensearchdescription+xml"
            title="Simple Icons"
            href="/opensearch.xml"
        />
        <Link rel="license" href="/license.txt" />
        <Link rel="canonical" href=URL />
        // TODO: application/ld+json (structured data)

        <MetaOpenGraph/>
        <MetaTwitter/>
        <Meta name="msvalidate.01" content="14319924BC1F00DC15EF0EAA29E72404"/>
        <Meta name="yandex-verification" content="8b467a0b98aa2725"/>

        <AppBody/>
    }
}

/// Open graph meta tags
#[component]
pub fn MetaOpenGraph(cx: Scope) -> impl IntoView {
    view! { cx,
        <Meta name="og:type" content="website"/>
        <Meta name="og:title" content=TITLE/>
        <Meta name="og:description" content=DESCRIPTION.as_str()/>
        <Meta name="og:url" content=URL/>
        <Meta name="og:site_name" content=TITLE/>
        // Note that the image is linked for Trunk at index.html
        <Meta name="og:image" content="/og.png"/>
    }
}

/// Twitter meta tags
#[component]
pub fn MetaTwitter(cx: Scope) -> impl IntoView {
    view! { cx,
        <Meta name="twitter:card" content="summary_large_image"/>
        <Meta name="twitter:title" content=TITLE/>
        <Meta name="twitter:description" content=DESCRIPTION.as_str()/>
        <Meta name="twitter:url" content=URL/>
        <Meta name="twitter:image:src" content="/og.png"/>
    }
}

/// Body of the page
#[component]
pub fn AppBody(cx: Scope) -> impl IntoView {
    view! { cx,
        <Header/>
    }
}
