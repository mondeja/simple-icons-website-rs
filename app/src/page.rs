use crate::app::{NUMBER_OF_ICONS, TITLE, URL};
use crate::meta::*;
use i18n::{gettext, move_gettext};
use leptos::*;
use leptos_meta::{Link, LinkProps, Meta, MetaProps, Title, TitleProps};

/// Each page of the application
///
/// This is only here in case that we want to add more
/// pages, but for now it's just a wrapper around the
/// main application component
#[component]
pub fn AppPage(cx: Scope, children: Children) -> impl IntoView {
    // TODO: onload change lang HTML element attribute
    //let locale = use_context::<LocaleStateSignal>(cx).wrap().0;

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
        <Link rel="apple-touch-icon" href="/apple-touch-icon.png"/>
        <Link
            rel="search"
            type_="application/opensearchdescription+xml"
            title=TITLE
            href="/opensearch.xml"
        />
        <Link rel="license" href="/license.txt"/>
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
        {children(cx)}
    }
}
