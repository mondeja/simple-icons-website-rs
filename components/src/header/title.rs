use crate::header::HeaderStateSignal;
use i18n::{gettext, move_gettext};
use leptos::*;
use macros::get_number_of_icons;

/// Header title
///
/// It includes the title and the description
/// about what is Simple Icons shown below the title.
#[component]
pub fn HeaderTitle() -> impl IntoView {
    let header_state = use_context::<HeaderStateSignal>().unwrap().0;

    view! {
        <div class:hidden=move || header_state().menu_open>
            <h1>"Simple Icons"</h1>
            <p inner_html=move_gettext!(
                 "{} free {} icons for popular brands", get_number_of_icons!() .to_string()
                .as_str(), & format!("<abbr title=\"{}\">{}</abbr>", gettext!(
                "Scalable Vector Graphic"), gettext!( "SVG"),)
            )></p>
        </div>
    }
}
