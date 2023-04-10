use crate::controls::download::{
    download_pdf, download_svg, DownloadType, DownloadTypeSignal,
};
use crate::copy::copy_setting_copied_transition_in_element;
use crate::grid::item::details::fill_icon_details_modal_with_icon;
use crate::grid::CurrentIconViewSignal;
use crate::svg_defs::SVGDefs;
use i18n::move_gettext;
use leptos::ev::MouseEvent;
use leptos::*;
use simple_icons::StaticSimpleIcon;
use web_sys::HtmlElement;

/// Icon grid item footer
///
/// Contains the buttons to copy color, view the expanded icon card and download the icon
#[component]
pub fn IconGridItemFooter(
    cx: Scope,
    /// The icon
    icon: StaticSimpleIcon,
) -> impl IntoView {
    // Hex color formatted for CSS
    let css_hex = format!("#{}", icon.hex);
    let css_hex_copy = css_hex.clone();

    // Controls context
    let download_type = use_context::<DownloadTypeSignal>(cx).unwrap().0;

    // Context to handle the opening state of detail modals
    let current_icon_view = use_context::<CurrentIconViewSignal>(cx).unwrap().0;

    view! { cx,
        // TODO: use defs SVG tags to optimize size
        <div>
            // Hex color
            // TODO: short hexes when possible
            <button
                title=move_gettext!(cx, "Copy hex color")
                class:dark=icon.hex_is_relatively_light
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
            <button
                title=move_gettext!(cx, "View {}", icon.title)
                on:click=move |_| {
                    fill_icon_details_modal_with_icon(icon);
                    current_icon_view.update(|state| *state = Some(icon));
                }
            >
                <svg viewBox="0 0 24 24">
                    <use_ href=format!("#{}", SVGDefs::ViewPath.id()) />
                </svg>
            </button>

            // Download
            <button
                title=move_gettext!(cx, "Download")
                // Error in data attribute to handle localization without JS
                // TODO: any better way to do this?
                data-error-generating-pdf-msg-schema=move_gettext!(
                    cx,
                    "Error generating PDF with PDFKit library: {}"
                )
                on:click=move |_| {
                    if download_type() == DownloadType::SVG {
                        download_svg(icon.slug);
                    } else {
                        download_pdf(icon.slug);
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
