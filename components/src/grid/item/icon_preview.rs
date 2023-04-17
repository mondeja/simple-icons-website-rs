use crate::copy::copy_setting_copied_transition_in_element;
use i18n::move_gettext;
use leptos::{ev::MouseEvent, *};
use reqwasm::http::Request;
use wasm_bindgen::JsCast;
use web_sys;

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
    // TODO: Handle HTTP failures here

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
                spawn_local(
                    (async move || {
                        let svg = Request::get(&src).send().await.unwrap().text().await.unwrap();
                        copy_setting_copied_transition_in_element(svg, target).await;
                    })(),
                );
            }
        >
            <img src=format!("/icons/{}.svg", slug) alt=move_gettext!(cx, "{} icon", title)/>
        </button>
    }
}
