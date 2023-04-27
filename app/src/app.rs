use crate::head::{Head, HeadProps};
use crate::pages::{Index, IndexProps};
use components::controls::color_scheme::{
    provide_color_scheme_context, ColorScheme,
};
use components::footer::{Footer, FooterProps};
use components::header::{
    nav::language_selector::provide_language_context, Header, HeaderProps,
};
use leptos::{
    document, html::Footer as FooterHtmlElement, provide_context, window, *,
};
use leptos_router::{
    Route, RouteProps, Router, RouterProps, Routes, RoutesProps,
};
use wasm_bindgen::JsCast;

macro_rules! url {
    () => {
        "https://simpleicons.org"
    };
}

/// Title of the page
pub static TITLE: &str = "Simple Icons";

/// URL of the website
pub static URL: &str = url!();

/// URL of Simple Icons logo
pub static LOGO_URL: &str = concat!(url!(), "/icons/simpleicons.svg");

/// The main application component
#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let color_scheme = provide_color_scheme_context(cx).0;

    create_effect(cx, move |_| {
        let body = document()
            .get_elements_by_tag_name("body")
            .get_with_index(0)
            .unwrap()
            .dyn_into::<web_sys::Element>()
            .unwrap();
        let body_class_list = body.class_list();
        body_class_list.remove_2("dark", "light").unwrap();
        body_class_list
            .add_1(match color_scheme() {
                ColorScheme::Dark => "dark",
                ColorScheme::Light => "light",
                ColorScheme::System => {
                    if window()
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
            })
            .unwrap();
    });

    let locale_signal = provide_language_context(cx).0;
    create_effect(cx, move |_| {
        let html = document()
            .document_element()
            .unwrap()
            .dyn_into::<web_sys::HtmlHtmlElement>()
            .unwrap();
        html.set_lang(locale_signal().code);
    });

    // Create a context to store a node reference to the footer
    // to use it in other components of pages
    let footer_ref = create_node_ref::<FooterHtmlElement>(cx);
    provide_context::<NodeRef<FooterHtmlElement>>(cx, footer_ref);

    view! { cx,
        <Head/>
        <Header/>
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
        <Footer container_ref=footer_ref/>
    }
}
