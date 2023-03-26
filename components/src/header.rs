use leptos::*;

/// Header title
///
/// It includes the title and the description
/// about what is Simple Icons shown below the title.
#[component]
fn HeaderTitle(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1 class="text-3xl">"Simple Icons"</h1>
    }
}

/// Header at the top of the page
#[component]
pub fn Header(cx: Scope) -> impl IntoView {
    view! { cx,
        <header class="columns-2">
            <HeaderTitle/>
            <menu class="flex justify-end">
                <p>"Hello"</p>
            </menu>
        </header>
    }
}
