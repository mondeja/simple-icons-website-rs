use leptos::*;

/// Header title
///
/// It includes the title and the description
/// about what is Simple Icons shown below the title.
#[component]
pub fn HeaderTitle(
    cx: Scope,
    /// Number of icons available in the library
    number_of_icons: usize,
) -> impl IntoView {
    view! { cx,
        <div class="flex flex-col columns-2">
            <h1 class="text-3xl">"Simple Icons"</h1>
            <p>{format!("{} free ", number_of_icons)}
                <abbr title="Scalable Vector Graphic">"SVG"</abbr>
            " icons for popular brands"</p>
        </div>
    }
}
