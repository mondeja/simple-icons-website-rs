use crate::app::{LOGO_URL, TITLE, URL};
use i18n::{gettext, move_gettext};
use leptos::*;
use leptos_meta::*;
use macros::get_number_of_icons;

#[component]
pub fn Head(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    let description = move_gettext!(
        cx,
        "{} free {} icons for popular brands",
        get_number_of_icons!().to_string().as_str(),
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
    }
}

/// Open graph meta tags
#[component]
fn MetaOpenGraph<F>(
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
        <Meta name="og:image" content="./og.png"/>
    }
}

/// Twitter meta tags
#[component]
fn MetaTwitter<F>(
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
        <Meta name="twitter:image:src" content="./og.png"/>
    }
}

/// JSON-LD metadata
/// See https://developers.google.com/search/docs/data-types/logo
#[component]
fn LdJSONMetadata(cx: Scope) -> impl IntoView {
    view! { cx,
        <script type="application/ld+json">
            {{
                serde_json::json!(
                    { "@context" : "https://schema.org", "@type" : "Organization", "name" : TITLE,
                    "url" : URL, "logo" : LOGO_URL, "image" : LOGO_URL, "potentialAction" : { "@type"
                    : "SearchAction", "target" : URL.to_owned() + "/?q={search-term}", "query-input"
                    : "required name=search-term", }, }
                )
                    .to_string()
            }}
        </script>
    }
}
