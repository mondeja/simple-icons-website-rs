use i18n::{gettext, move_gettext};
use leptos::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <div class="footer-about">
            <p inner_html=move_gettext!(
                "A {} project maintained by the {}.",
                &format!("<a href=\"https://github.com/simple-icons/simple-icons/blob/develop/LICENSE.md\">{}</a>",
                gettext!("CCO")),
                &format!("<a href=\"https://github.com/simple-icons/simple-icons\">{}</a>",
                gettext!("Simple Icons contributors"))
            )></p>
            <p inner_html=move_gettext!(
                "Use {} for requests, corrections and contributions.",
                &format!("<a href=\"https://github.com/simple-icons/simple-icons\">{}</a>",
                gettext!("GitHub"))
            )></p>
            <p inner_html=move_gettext!(
                "Kindly supported by your donations at {}.",
                &format!("<a href=\"https://opencollective.com/simple-icons\">{}</a>",
                gettext!("Open Collective"))
            )></p>
        </div>
    }
}
