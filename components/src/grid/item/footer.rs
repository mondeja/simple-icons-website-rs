use crate::controls::download::{
    download_pdf, download_svg, DownloadType, DownloadTypeSignal,
};
use crate::copy::copy_setting_copied_transition_in_element;
use crate::grid::item::details::fill_icon_details_modal_with_icon;
use crate::grid::CurrentIconViewSignal;
use crate::svg_defs::SVGDefs;
use i18n::{move_gettext, LocaleSignal};
use leptos::ev::MouseEvent;
use leptos::*;
use types::SimpleIcon;
use web_sys;

/// Icon grid item footer
///
/// Contains the buttons to copy color, view the expanded icon card and download the icon
#[component]
pub fn IconGridItemFooter(
    cx: Scope,
    /// The icon
    icon: &'static SimpleIcon,
) -> impl IntoView {
    // Hex color formatted for CSS
    let css_hex = &format!("#{}", icon.hex);

    // Controls context
    let download_type = use_context::<DownloadTypeSignal>(cx).unwrap().0;

    // Context to handle the opening state of detail modals
    let current_icon_view = use_context::<CurrentIconViewSignal>(cx).unwrap().0;

    // Locale context
    let locale_state = use_context::<LocaleSignal>(cx).unwrap().0;

    view! { cx,
        <div>
            <button
                title=move_gettext!(cx, "Copy hex color")
                class:dark=icon.hex_is_relatively_light
                style=format!("background: {}", css_hex)
                on:click=move |ev: MouseEvent| {
                    let target = event_target::<web_sys::HtmlElement>(&ev);
                    let value = target.text_content().unwrap();
                    spawn_local(copy_setting_copied_transition_in_element(value, target));
                }
            >
                {css_hex}
            </button>
            <button
                title=move_gettext!(cx, "View {}", icon.title)
                on:click=move |_| {
                    fill_icon_details_modal_with_icon(cx, icon, &locale_state());
                    current_icon_view.update(|state| *state = Some(icon));
                }
            >
                <svg viewBox="0 0 24 24">
                    <use_ href=format!("#{}", SVGDefs::ViewPath.id())></use_>
                </svg>
            </button>
            <button
                title=move_gettext!(cx, "Download")
                data-error-generating-pdf-msg-schema=move_gettext!(cx, "Error generating PDF with PDFKit library: {}")
                on:click=move |_| {
                    if download_type() == DownloadType::SVG {
                        download_svg(icon.slug);
                    } else {
                        download_pdf(icon.slug);
                    }
                }
            >
                <svg viewBox="0 0 24 24">
                    <use_ href=format!("#{}", SVGDefs::DownloadPath.id())></use_>
                </svg>
            </button>
        </div>
    }
}
