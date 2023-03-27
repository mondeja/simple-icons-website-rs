use leptos::*;

/// Header title
///
/// It includes the title and the description
/// about what is Simple Icons shown below the title.
#[component]
fn HeaderTitle(
    cx: Scope,
    /// Number of icons available in the library
    number_of_icons: u32,
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

/// Header at the top of the page
#[component]
pub fn Header(
    cx: Scope,
    /// Number of icons available in the library
    number_of_icons: u32,
) -> impl IntoView {
    view! { cx,
        <header class="flex flex-row columns-2">
            <HeaderTitle number_of_icons=number_of_icons/>
            <menu class="flex justify-end">
                <p>"Hello"</p>
            </menu>
        </header>
    }
}
