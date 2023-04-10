use crate::copy::copy_setting_copied_transition_in_element;
use i18n::move_gettext;
use leptos::{ev::MouseEvent, *};
use web_sys::HtmlElement;

/// Icon grid item title
#[component]
pub fn IconGridItemTitle(
    cx: Scope,
    /// Brand title
    title: &'static str,
    /// Slug
    slug: &'static str,
) -> impl IntoView {
    view! { cx,
        <h2
            title=move_gettext!(cx, "Copy {} slug ({})", title, slug)
            on:click=move|ev: MouseEvent|{
                let target = event_target::<HtmlElement>(&ev);
                spawn_local(
                    copy_setting_copied_transition_in_element(slug.to_string(), target)
                );
            }
        >{title}</h2>
    }
}
