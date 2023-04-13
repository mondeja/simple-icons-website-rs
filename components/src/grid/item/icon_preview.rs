use crate::copy::copy_setting_copied_transition_in_element;
use i18n::move_gettext;
use leptos::{ev::MouseEvent, html::Img, *};
use reqwasm::http::Request;
use wasm_bindgen::JsCast;
use web_sys;

async fn fetch_svg_value_and_copy_setting_copied_transition_in_element(
    src: String,
    button: web_sys::HtmlElement,
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
    let image_ref = create_node_ref::<Img>(cx);

    view! { cx,
        <button
            title=move_gettext!(cx, "Copy {} SVG", title)
            on:click=move |ev: MouseEvent| {
                let target = event_target::<web_sys::HtmlElement>(&ev);
                let src = target
                    .children()
                    .item(0)
                    .unwrap()
                    .dyn_into::<web_sys::HtmlImageElement>()
                    .unwrap()
                    .get_attribute("src")
                    .unwrap();
                let future = fetch_svg_value_and_copy_setting_copied_transition_in_element(
                    src,
                    target,
                );
                spawn_local(future);
            }
        >
            <img
                _ref=image_ref
                src=format!("/icons/{}.svg", slug)
                alt=move_gettext!(cx, "{} icon", title)
                on:load=move |_| {
                    image_ref.get().unwrap().class_list().add_1("loaded").unwrap();
                }
            />
        </button>
    }
}
