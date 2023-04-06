use crate::header::HeaderStateSignal;
use i18n::{gettext, move_gettext};
use leptos::*;
use macros::get_number_of_icons;

/// Header title
///
/// It includes the title and the description
/// about what is Simple Icons shown below the title.
#[component]
pub fn HeaderTitle(cx: Scope) -> impl IntoView {
    let header_state = use_context::<HeaderStateSignal>(cx).unwrap().0;

    // This is the most complex example of translation in the project.
    // TODO: separate the localization system in a third party package
    // for Leptos and use in the project, using this as the most complex example
    let description = move_gettext!(
        cx,
        "{} free {} icons for popular brands",
        get_number_of_icons!().to_string().as_str(),
        &format!(
            "<abbr title=\"{}\">{}</abbr>",
            gettext!(cx, "Scalable Vector Graphic"),
            gettext!(cx, "SVG"),
        )
    );

    view! { cx,
        <div class=move || {
            let mut cls = "flex flex-col columns-2".to_string();
            if header_state.get().menu_open {
                cls.push_str(" hidden");
            }
            cls
        }>
            <h1 class="text-[1.7rem] font-semibold">"Simple Icons"</h1>
            <p inner_html=description></p>
        </div>
    }
}
