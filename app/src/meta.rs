use crate::app::{LOGO_URL, TITLE, URL};
use leptos::*;
use leptos_meta::{Meta, MetaProps};

// TODO: There is a `link` tag with a 'color' attribute in the index.html
// Arbitrary attributes in Link components are not currently supported by
// leptos_meta

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
        <Meta name="og:image" content="./og.png"/>
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
        <Meta name="twitter:image:src" content="./og.png"/>
    }
}

/// JSON-LD metadata
/// See https://developers.google.com/search/docs/data-types/logo
#[component]
pub fn LdJSONMetadata(cx: Scope) -> impl IntoView {
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
