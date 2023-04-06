use crate::controls::download::{
    pdf::download_pdf, svg::download_svg, DownloadType,
};
use crate::controls::ControlsStateSignal;
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
                    <path d="m23.136 20.694-4.41-4.413a1.93 1.93 0 0 0-1.186-.551 9.632 9.632 0 0 0 2.13-6.044C19.67 4.344 15.325 0 9.983 0 4.642 0 .297 4.344.297 9.686c0 5.34 4.344 9.685 9.685 9.685 2.016 0 3.89-.62 5.44-1.677.01.48.195.957.563 1.325l4.413 4.413c.377.38.874.568 1.369.568s.992-.189 1.369-.568a1.935 1.935 0 0 0 0-2.738zm-13.154-4.55a6.465 6.465 0 0 1-6.458-6.458 6.465 6.465 0 0 1 6.458-6.458 6.465 6.465 0 0 1 6.458 6.458 6.465 6.465 0 0 1-6.458 6.458z"/>
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
                    <path d="M11.2 0a.8.8 0 0 0-.8.8v11.4L7.26 9.44a.803.803 0 0 0-1.13.074l-1.05 1.2a.8.8 0 0 0 .073 1.13l6.33 5.54a.795.795 0 0 0 1.05 0l6.32-5.54a.8.8 0 0 0 .074-1.13l-1.05-1.2a.804.804 0 0 0-1.13-.074l-3.14 2.76V.8a.8.8 0 0 0-.8-.8zm-8 20.8a.8.8 0 0 0-.8.8v1.6a.8.8 0 0 0 .8.8h17.6a.8.8 0 0 0 .8-.8v-1.6a.8.8 0 0 0-.8-.8z"/>
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
    /// Alphabetic order index
    order_alpha_index: usize,
) -> impl IntoView {
    view! { cx,
        // The grid items are styled in item.css
        <li o=format!("{}-", order_alpha_index)>
            <IconGridItemPreview slug=slug title=title />
            <IconGridItemTitle title=title/>
            <IconGridItemFooter slug=slug hex=hex title=title/>
        </li>
    }
}
