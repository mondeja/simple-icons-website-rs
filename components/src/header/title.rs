use crate::header::HeaderStateSignal;
use leptos::*;
use macros::get_number_of_icons;

/// Header title
///
/// It includes the title and the description
/// about what is Simple Icons shown below the title.
#[component]
pub fn HeaderTitle(cx: Scope) -> impl IntoView {
    let header_state = use_context::<HeaderStateSignal>(cx).unwrap().0;

    view! { cx,
        <div class=move || {
            let mut cls = "flex flex-col columns-2".to_string();
            if header_state.get().menu_open {
                cls.push_str(" hidden");
            }
            cls
        }>
            <h1 class="text-3xl">"Simple Icons"</h1>
            <p>{format!("{} free ", get_number_of_icons!())}
                <abbr title="Scalable Vector Graphic">"SVG"</abbr>
            " icons for popular brands"</p>
        </div>
    }
}
