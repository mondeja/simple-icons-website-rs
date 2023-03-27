use components::*;
use leptos::*;
use macros::number_of_icons;

/// The main application component
#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <Header number_of_icons=number_of_icons!()/>
    }
}
