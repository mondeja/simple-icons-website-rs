use crate::controls::download::{
    pdf::download_pdf, svg::download_svg, DownloadType, DownloadTypeSignal,
};
use crate::copy::copy_setting_copied_transition_in_element;
use crate::svg_defs::SVGDefs;
use i18n::{gettext, move_gettext};
use leptos::ev::MouseEvent;
use leptos::*;
use web_sys::HtmlElement;

/// Icon grid item footer
///
/// Contains the buttons to copy color, view the expanded icon card and download the icon
#[component]
pub fn IconGridItemFooter(
    cx: Scope,
    /// Icon slug
    slug: &'static str,
    /// Brand title
    title: &'static str,
    /// Brand color
    hex: &'static str,
    /// Hex color is relatively light, which means that the text should be dark
    hex_is_relatively_light: bool,
) -> impl IntoView {
    // Hex color formatted for CSS
    let css_hex = format!("#{}", hex);
    let css_hex_copy = css_hex.clone();

    // Controls context
    let download_type = use_context::<DownloadTypeSignal>(cx).unwrap().0;

    view! { cx,
        // TODO: use defs SVG tags to optimize size
        <div>
            // Hex color
            // TODO: short hexes when possible
            <button
                title=move_gettext!(cx, "Copy hex color", title)
                class:dark=hex_is_relatively_light
                style=format!("background: {}", css_hex)
                on:click=move |ev: MouseEvent| {
                    let target = event_target::<HtmlElement>(&ev);
                    let value = target.text_content().unwrap();
                    spawn_local(copy_setting_copied_transition_in_element(value, target));
                }
            >
                {css_hex_copy}
            </button>

            // Open card
            <button title=move_gettext!(cx, "View {}", title)>
                <svg viewBox="0 0 24 24">
                    <use_ href=format!("#{}", SVGDefs::ViewPath.id()) />
                </svg>
            </button>

            // Download
            <button
                title=move_gettext!(cx, "Download")
                on:click=move |_| {
                    if download_type() == DownloadType::SVG {
                        download_svg(slug);
                    } else {
                        download_pdf(slug, gettext!(cx, "Error generating PDF with PDFKit library: {}"));
                    }
                }
            >
                <svg viewBox="0 0 24 24">
                    <use_ href=format!("#{}", SVGDefs::DownloadPath.id()) />
                </svg>
            </button>
        </div>
    }
}
