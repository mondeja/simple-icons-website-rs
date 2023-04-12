use i18n::{gettext, move_gettext};
use leptos::*;

#[component]
pub fn About(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="footer-about">
            <p inner_html=move_gettext!(
                cx, "A {} project maintained by the {}.", &
                format!("<a href=\"https://github.com/simple-icons/simple-icons/blob/develop/LICENSE.md\">{}</a>",
                gettext!(cx, "CCO")), &
                format!("<a href=\"https://github.com/simple-icons/simple-icons\">{}</a>",
                gettext!(cx, "Simple Icons contributors"))
            )></p>
            <p inner_html=move_gettext!(
                cx, "Use {} for requests, corrections and contributions.", &
                format!("<a href=\"https://github.com/simple-icons/simple-icons\">{}</a>",
                gettext!(cx, "GitHub"))
            )></p>
            <p inner_html=move_gettext!(
                cx, "Kindly supported by your donations at {}.", &
                format!("<a href=\"https://opencollective.com/simple-icons\">{}</a>", gettext!(cx,
                "Open Collective"))
            )></p>
        </div>
    }
}
