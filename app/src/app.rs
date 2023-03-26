use components::*;
use leptos::*;

/// The main application component
#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <Header/>
    }
}
