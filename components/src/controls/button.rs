use leptos::*;

/// Control button
#[component]
pub fn ControlButton(
    cx: Scope,
    /// Button title
    title: &'static str,
    /// Button icon SVG path
    svg_path: &'static str,
) -> impl IntoView {
    view! { cx,
        <button class="w-10 h-10 p-1.5" type="button" title=title>
            <svg
                role="img"
                viewBox="0 0 24 24"
                xmlns="http://www.w3.org/2000/svg"
            >
                <path d=svg_path/>
            </svg>
        </button>
    }
}
