use crate::controls::download::{download_pdf, download_svg};
use crate::copy::copy_setting_copied_transition_in_element;
use crate::grid::CurrentIconViewSignal;
use crate::modal::*;
use crate::Ids;
use i18n::move_gettext;
use leptos::{document, ev::MouseEvent, *};
use reqwasm::http::Request;
use simple_icons::StaticSimpleIcon;
use wasm_bindgen::JsCast;
use web_sys;

fn get_slug_from_modal_container() -> String {
    document()
        .get_element_by_id(Ids::IconDetailsModal.as_str())
        .unwrap()
        .get_elements_by_tag_name("h3")
        .item(0)
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap()
        .inner_text()
}

pub fn fill_icon_details_modal_with_icon(icon: StaticSimpleIcon) {
    let modal_body = document()
        .get_element_by_id(Ids::IconDetailsModal.as_str())
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();

    // Set the modal title
    let modal_header = modal_body
        .parent_element()
        .unwrap()
        .previous_element_sibling()
        .unwrap()
        .first_element_child()
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();
    modal_header.set_inner_text(icon.title);

    // Set the slug
    let modal_slug = modal_body
        .get_elements_by_tag_name("h3")
        .item(0)
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();
    modal_slug.set_inner_text(icon.slug);

    // Set the copy hex color button
    let modal_hex_color_button = modal_body
        .get_elements_by_tag_name("button")
        .item(0)
        .unwrap()
        .dyn_into::<web_sys::HtmlButtonElement>()
        .unwrap();
    modal_hex_color_button.set_inner_text(&format!("#{}", icon.hex));
    modal_hex_color_button
        .set_attribute(
            "style",
            &format!(
                "background-color:#{};color:var(--{}-contrast-color);",
                icon.hex,
                match icon.hex_is_relatively_light {
                    true => "dark",
                    false => "light",
                }
            ),
        )
        .unwrap();
    modal_hex_color_button
        .class_list()
        .add_1(match icon.hex_is_relatively_light {
            true => "copy-button-black",
            false => "copy-button-white",
        })
        .unwrap();

    // Set preview image container src
    modal_body
        .get_elements_by_tag_name("img")
        .item(0)
        .unwrap()
        .dyn_into::<web_sys::HtmlImageElement>()
        .unwrap()
        .set_attribute("src", &format!("/icons/{}.svg", icon.slug))
        .unwrap();

    // Set the brand guidelines link
    let modal_brand_guidelines_link = modal_body
        .get_elements_by_tag_name("a")
        .item(0)
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();
    if let Some(guidelines_url) = icon.guidelines_url {
        modal_brand_guidelines_link
            .set_attribute("href", guidelines_url)
            .unwrap();
        modal_brand_guidelines_link
            .class_list()
            .remove_1("hidden")
            .unwrap();
    } else {
        modal_brand_guidelines_link
            .class_list()
            .add_1("hidden")
            .unwrap();
    }

    // Set the license
    let modal_license_link = modal_body
        .get_elements_by_tag_name("a")
        .item(1)
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();
    if icon.license_url.is_some() || icon.license_type.is_some() {
        modal_license_link.class_list().remove_1("hidden").unwrap();
    } else {
        modal_license_link.class_list().add_1("hidden").unwrap();
    }
    if let Some(license_url) = icon.license_url {
        modal_license_link
            .set_attribute("href", license_url)
            .unwrap();
    }
    if let Some(license_type) = icon.license_type {
        modal_license_link.set_inner_text(license_type);
        modal_license_link
            .set_attribute(
                "href",
                &format!("https://spdx.org/licenses/{}", license_type),
            )
            .unwrap();
    } else {
        let title = modal_license_link.get_attribute("title").unwrap();
        modal_license_link.set_inner_text(&title);
    }

    let modal_footer = modal_body
        .first_element_child()
        .unwrap()
        .next_element_sibling()
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();

    let download_colored_icon_container = modal_footer
        .get_elements_by_tag_name("a")
        .item(0)
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();

    spawn_local((async move || {
        let svg = Request::get(&format!("/icons/{}.svg", icon.slug))
            .send()
            .await
            // TODO: handle http errors
            .unwrap()
            .text()
            .await
            .unwrap();
        let colored_icon_svg =
            svg.replace("<svg", &format!("<svg fill=\"%23{}\"", icon.hex));
        download_colored_icon_container
            .set_attribute(
                "href",
                &format!("data:image/svg+xml,{}", &colored_icon_svg),
            )
            .unwrap();
        download_colored_icon_container
            .set_attribute("download", &format!("{}-color.svg", icon.slug))
            .unwrap();
    })());
}

/// Details modal icon preview
#[component]
fn IconDetailsModalPreview(cx: Scope) -> impl IntoView {
    view! { cx, <img/> }
}

/// Details modal icon information
#[component]
fn IconDetailsModalInformation(cx: Scope) -> impl IntoView {
    view! { cx,
        <div>
            <h3></h3>
            <button
                on:click=move |ev: MouseEvent| {
                    let target = event_target::<web_sys::HtmlElement>(&ev);
                    let value = target.text_content().unwrap();
                    spawn_local(copy_setting_copied_transition_in_element(value, target));
                }
                title=move_gettext!(cx, "Copy hex color")
            ></button>
            <a target="_blank">{move_gettext!(cx, "Brand guidelines")}</a>
            <a target="_blank" title=move_gettext!(cx, "License")></a>
        </div>
    }
}

#[component]
fn IconDetailsModalFooter(cx: Scope) -> impl IntoView {
    view! { cx,
        <div>
            <button
                on:click=move |_| download_svg(&get_slug_from_modal_container())
                aria-label=move_gettext!(cx, "Download SVG")
            >
                {move_gettext!(cx, "Download SVG")}
            </button>
            <a aria-label=move_gettext!(cx, "Download colored SVG")>
                {move_gettext!(cx, "Download colored SVG")}
            </a>
            <button
                on:click=move |_| download_pdf(&get_slug_from_modal_container())
                aria-label=move_gettext!(cx, "Download PDF")
            >
                {move_gettext!(cx, "Download PDF")}
            </button>
        </div>
    }
}

/// Detail modal view for icons
#[component]
pub fn IconDetailsModal(cx: Scope) -> impl IntoView {
    let current_icon_view = use_context::<CurrentIconViewSignal>(cx).unwrap().0;

    view! { cx,
        <Modal
            title=move || "".to_string()
            is_open=move || current_icon_view().is_some()
            on_close=move |_| {
                current_icon_view.update(|state| *state = None);
            }
        >
            <div class="icon-details-modal" id=Ids::IconDetailsModal.as_str()>
                <div>
                    <IconDetailsModalPreview/>
                    <IconDetailsModalInformation/>
                </div>
                <IconDetailsModalFooter/>
            </div>
        </Modal>
    }
}
