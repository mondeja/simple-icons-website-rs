use crate::head::Head;
use crate::pages::{AllIconsIndex, DeprecationsIndex, Error404, Preview};
use fluent_templates::static_loader;
use leptos::{html::Footer as FooterHtmlElement, prelude::*};
use leptos_fluent::leptos_fluent;
use leptos_hotkeys::{provide_hotkeys_context, scopes};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, WildcardSegment,
};
use leptos_use::{
    use_color_mode_with_options, ColorMode, UseColorModeOptions,
    UseColorModeReturn,
};
use simple_icons_website_footer::Footer;
use simple_icons_website_header::Header;
use simple_icons_website_modal::provide_modal_open_context;
use simple_icons_website_storage::LocalStorage;
use simple_icons_website_svg_defs::SVGDefsDefinition;
use simple_icons_website_url as Url;

/// Title of the page
pub static TITLE: &str = "Simple Icons";

static_loader! {
    static TRANSLATIONS = {
        locales: "./locales",
        fallback_language: "en-US",
        customise: |bundle| bundle.set_use_isolating(false),
    };
}

#[component]
fn I18n(children: Children) -> impl IntoView {
    leptos_fluent! {
        children: children(),
        locales: "./locales",
        translations: [TRANSLATIONS],
        #[cfg(debug_assertions)]
        check_translations: "../{app,components}/**/*.rs",
        sync_html_tag_lang: true,
        sync_html_tag_dir: true,
        url_param: Url::params::Names::Language.as_str(),
        initial_language_from_url_param: true,
        initial_language_from_url_param_to_localstorage: true,
        localstorage_key: LocalStorage::Keys::Language.as_str(),
        initial_language_from_localstorage: true,
        set_language_to_localstorage: true,
        initial_language_from_navigator: true,
        initial_language_from_navigator_to_localstorage: true,
    }
}

/// The main application component
#[component]
pub fn App() -> impl IntoView {
    let UseColorModeReturn {
        mode: color_mode,
        set_mode: set_color_mode,
        ..
    } = use_color_mode_with_options(
        UseColorModeOptions::default()
            .storage_enabled(true)
            .storage(leptos_use::storage::StorageType::Local)
            .storage_key(LocalStorage::Keys::ColorScheme.as_str())
            .target("body")
            .attribute("class")
            .emit_auto(true)
            .initial_value_from_url_param(
                Url::params::Names::ColorScheme.as_str(),
            )
            .initial_value_from_url_param_to_storage(true),
    );

    provide_context::<(Signal<ColorMode>, WriteSignal<ColorMode>)>((
        color_mode,
        set_color_mode,
    ));

    // Create a context to store a node reference to the footer
    // to use it in other components of pages
    let footer_ref = NodeRef::new();
    provide_context::<NodeRef<FooterHtmlElement>>(footer_ref);

    // Create a context to store the current opened modal
    provide_modal_open_context();

    // Create a context to store keyboard shortcuts
    let main_ref = NodeRef::new();

    // Provide context for keyboard shortcuts
    provide_hotkeys_context(main_ref, false, scopes!());

    // TODO: Note that we're not using the `fallback` prop of `Routes` here.
    // Because it doesn't works on production. Instead, we're using a `WildcardSegment`
    // to render 404 pages. This is a workaround until the issue is reported to leptos
    // and fixed.
    #[allow(clippy::unit_arg, clippy::unused_unit)]
    let empty_view = || view!();

    view! {
        <I18n>
            <Head />
            <Header />
            <SVGDefsDefinition />
            <main node_ref=main_ref>
                <Router>
                    <Routes fallback=empty_view>
                        <Route path=StaticSegment("/preview") view=Preview />
                        <Route path=StaticSegment("/deprecations") view=DeprecationsIndex />
                        <Route path=StaticSegment("/404") view=Error404 />
                        <Route path=StaticSegment("/") view=AllIconsIndex />
                        <Route path=WildcardSegment("*") view=Error404 />
                    </Routes>
                </Router>
            </main>
            <Footer container_ref=footer_ref />
        </I18n>
    }
}
