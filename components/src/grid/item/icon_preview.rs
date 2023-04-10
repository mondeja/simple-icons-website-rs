use crate::copy::copy_setting_copied_transition_in_element;
use i18n::move_gettext;
use leptos::{ev::MouseEvent, *};
use reqwasm::http::Request;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlImageElement};

async fn fetch_svg_value_and_copy_setting_copied_transition_in_element(
    src: String,
    button: HtmlElement,
) {
    // TODO: Handle HTTP failures here
    let svg = Request::get(&src)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    copy_setting_copied_transition_in_element(svg, button).await;
}

/// Icon grid item preview
///
/// The icon preview in the grid.
/// Contains the lazy loaded logo of the brand.
#[component]
pub fn IconGridItemPreview(
    cx: Scope,
    /// Icon slug
    slug: &'static str,
    /// Brand title
    title: &'static str,
) -> impl IntoView {
    view! { cx,
        <button
            title=move_gettext!(cx, "Copy {} SVG", title)
            on:click=move |ev: MouseEvent| {
                let target = event_target::<HtmlElement>(&ev);
                let src = target
                    .children()
                    .item(0)
                    .unwrap()
                    .dyn_into::<HtmlImageElement>()
                    .unwrap()
                    .get_attribute("src")
                    .unwrap();
                let future = fetch_svg_value_and_copy_setting_copied_transition_in_element(src, target);
                spawn_local(future);
            }
        >
            <img
                src=format!("/icons/{}.svg", slug)
                loading="lazy"
                alt=move_gettext!(cx, "{} icon", title)
            />
        </button>
    }
}
