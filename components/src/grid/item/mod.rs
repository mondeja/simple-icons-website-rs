use crate::controls::download::{
    pdf::download_pdf, svg::download_svg, DownloadType,
};
use crate::controls::ControlsStateSignal;
use crate::svg_defs::SVGDef;
use i18n::{gettext, move_gettext};
use leptos::*;

/// Icon grid item preview
///
/// The icon preview in the grid.
/// Contains the lazy loaded logo of the brand.
#[component]
pub fn IconGridItemPreview(
    cx: Scope,
    /// Icon slug
    slug: &'static str,
    /// Brand title
    title: &'static str,
) -> impl IntoView {
    view! { cx,
        <div>
            <button
                class=""
                title=move_gettext!(cx, "{} SVG", title)
            >
                <img
                    src=format!("/icons/{}.svg", slug)
                    loading="lazy"
                    alt=move_gettext!(cx, "{} icon", title)
                />
            </button>
        </div>
    }
}

/// Icon grid item title
#[component]
pub fn IconGridItemTitle(
    cx: Scope,
    /// Brand title
    title: &'static str,
) -> impl IntoView {
    view! { cx,
        <h2>{title}</h2>
    }
}

/// Icon grid item footer
///
/// Contains the buttons to copy color, view the expanded icon card and download the icon
#[component]
pub fn IconGridItemFooter(
    cx: Scope,
    /// Icon slug
    slug: &'static str,
    /// Brand color
    hex: &'static str,
    /// Brand title
    title: &'static str,
) -> impl IntoView {
    // Hex color formatted for CSS
    let css_hex = format!("#{}", hex);

    // Controls context
    let controls_state = use_context::<ControlsStateSignal>(cx).unwrap().0;

    view! { cx,
        // TODO: use defs SVG tags to optimize size
        <div>
            // Hex color
            <button style=format!("background: {}", css_hex)>
                {css_hex}
            </button>

            // Open card
            <button title=move_gettext!(cx, "View {}", title)>
                <svg viewBox="0 0 24 24">
                    <use_ href=format!("#{}", SVGDef::ViewPath.id()) />
                </svg>
            </button>

            // Download
            <button
                title=move_gettext!(cx, "Download")
                on:click=move |_| {
                    if controls_state().download_type == DownloadType::SVG {
                        download_svg(slug);
                    } else {
                        download_pdf(slug, gettext!(cx, "Error generating PDF with PDFKit library: {}"));
                    }
                }
            >
                <svg viewBox="0 0 24 24">
                    <use_ href=format!("#{}", SVGDef::DownloadPath.id()) />
                </svg>
            </button>
        </div>
    }
}

/// Icon grid item
///
/// Each icon displayed in the icons grid
#[component]
pub fn IconGridItem(
    cx: Scope,
    /// Icon slug
    slug: &'static str,
    /// Brand title
    title: &'static str,
    /// Brand color
    hex: &'static str,
) -> impl IntoView {
    view! { cx,
        // The grid items are styled in item.css
        <li>
            <IconGridItemPreview slug=slug title=title />
            <IconGridItemTitle title=title/>
            <IconGridItemFooter slug=slug hex=hex title=title/>
        </li>
    }
}
