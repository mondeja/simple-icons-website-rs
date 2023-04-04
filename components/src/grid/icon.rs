use crate::controls::download::{
    pdf::download_pdf, svg::download_svg, DownloadType,
};
use crate::controls::ControlsStateSignal;
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
        <div class="m-auto pt-6">
            <button
                class=""
                title=format!("{} SVG", title)
            >
                <img
                    class="h-14"
                    src=format!("/icons/{}.svg", slug)
                    loading="lazy"
                    alt=format!("{} icon", title)
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
        <h2 class="pl-3 pt-2">{title}</h2>
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
        <div class="flex flex-row h-[26px]">
            // Hex color
            <button class="p-0.5 w-1/2" style=format!("background-color:{}", css_hex)>
                {css_hex}
            </button>

            // Open card
            <button class="w-1/4" title=format!("View {}", title)>
                <svg
                    class="h-4 m-auto"
                    viewBox="0 0 24 24"
                    xmlns="http://www.w3.org/2000/svg"
                >
                    <path d="m23.136 20.694-4.41-4.413a1.93 1.93 0 0 0-1.186-.551 9.632 9.632 0 0 0 2.13-6.044C19.67 4.344 15.325 0 9.983 0 4.642 0 .297 4.344.297 9.686c0 5.34 4.344 9.685 9.685 9.685 2.016 0 3.89-.62 5.44-1.677.01.48.195.957.563 1.325l4.413 4.413c.377.38.874.568 1.369.568s.992-.189 1.369-.568a1.935 1.935 0 0 0 0-2.738zm-13.154-4.55a6.465 6.465 0 0 1-6.458-6.458 6.465 6.465 0 0 1 6.458-6.458 6.465 6.465 0 0 1 6.458 6.458 6.465 6.465 0 0 1-6.458 6.458z"/>
                </svg>
            </button>

            // Download
            <button
                class="w-1/4"
                title="Download"
                on:click=move |_| {
                    if controls_state().download_type == DownloadType::SVG {
                        download_svg(slug);
                    } else {
                        download_pdf(slug);
                    }
                }
            >
                <svg
                    class="h-4 mt-1 m-auto"
                    viewBox="0 0 24 24"
                    xmlns="http://www.w3.org/2000/svg"
                >
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
) -> impl IntoView {
    view! { cx,
        <div class="inline-flex flex-col border-2 space-y-2">
            <IconGridItemPreview slug=slug title=title />
            <IconGridItemTitle title=title/>
            <IconGridItemFooter slug=slug hex=hex title=title/>
        </div>
    }
}
