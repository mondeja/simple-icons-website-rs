use crate::controls::download::{download_pdf, download_svg};
use crate::grid::CurrentIconViewSignal;
use crate::modal::*;
use i18n::move_gettext;
use leptos::*;
use reqwasm::http::Request;
use simple_icons::StaticSimpleIcon;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlButtonElement, HtmlElement, HtmlImageElement};

static ICON_DETAILS_MODAL_ID: &str = "icon-details-modal";

async fn fetch_svg_value_and_set_download_colored_button_href(
    slug: &'static str,
    icon_hex: &'static str,
    download_colored_icon_container: HtmlElement,
) {
    // TODO: handle http errors
    let svg = Request::get(&format!("/icons/{}.svg", slug))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let colored_icon_svg =
        svg.replace("<svg", &format!("<svg fill=\"%23{}\"", icon_hex));
    download_colored_icon_container
        .set_attribute(
            "href",
            &format!("data:image/svg+xml,{}", &colored_icon_svg),
        )
        .unwrap();
    download_colored_icon_container
        .set_attribute("download", &format!("{}-color.svg", slug))
        .unwrap();
}

fn get_slug_from_modal_container() -> String {
    document()
        .get_element_by_id(ICON_DETAILS_MODAL_ID)
        .unwrap()
        .get_elements_by_tag_name("h3")
        .item(0)
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap()
        .inner_text()
}

pub fn fill_icon_details_modal_with_icon(icon: StaticSimpleIcon) {
    let document = window().unwrap().document().unwrap();

    let modal_body = document
        .get_element_by_id(ICON_DETAILS_MODAL_ID)
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();

    // Set the modal title
    let modal_header = modal_body
        .parent_element()
        .unwrap()
        .previous_element_sibling()
        .unwrap()
        .first_element_child()
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();
    modal_header.set_inner_text(icon.title);

    // Set the slug
    let modal_slug = modal_body
        .get_elements_by_tag_name("h3")
        .item(0)
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();
    modal_slug.set_inner_text(icon.slug);

    // Set the copy hex color button
    let modal_hex_color_button = modal_body
        .get_elements_by_tag_name("button")
        .item(0)
        .unwrap()
        .dyn_into::<HtmlButtonElement>()
        .unwrap();
    modal_hex_color_button.set_inner_text(icon.hex);
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

    // Set preview image container src
    modal_body
        .get_elements_by_tag_name("img")
        .item(0)
        .unwrap()
        .dyn_into::<HtmlImageElement>()
        .unwrap()
        .set_attribute("src", &format!("/icons/{}.svg", icon.slug))
        .unwrap();

    // Set the brand guidelines link
    let modal_brand_guidelines_link = modal_body
        .get_elements_by_tag_name("a")
        .item(0)
        .unwrap()
        .dyn_into::<HtmlElement>()
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
        .dyn_into::<HtmlElement>()
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
        .dyn_into::<HtmlElement>()
        .unwrap();

    let download_colored_icon_container = modal_footer
        .get_elements_by_tag_name("a")
        .item(0)
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();

    spawn_local(fetch_svg_value_and_set_download_colored_button_href(
        icon.slug,
        icon.hex,
        download_colored_icon_container,
    ));
}

/// Details modal icon preview
#[component]
fn IconDetailsModalPreview(cx: Scope) -> impl IntoView {
    view! { cx,
        <img class="w-1/2 p-10 dark:invert"/>
    }
}

/// Details modal icon information
#[component]
fn IconDetailsModalInformation(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex flex-col w-1/2 space-y-2">
            // Slug
            <h3></h3>
            // Hex color button
            <button title=move_gettext!(cx, "Copy hex color")></button>
            // Brand guidelines
            <a
                class="pt-7 hover:opacity-70"
                target="_blank"
                title=move_gettext!(cx, "Brand guidelines")
            >
                {move_gettext!(cx, "Brand guidelines")}
            </a>
            // License
            <a
                class="pt-7 hover:opacity-70"
                target="_blank"
                title=move_gettext!(cx, "License")
            ></a>
        </div>
    }
}

#[component]
fn IconDetailsModalFooter(cx: Scope) -> impl IntoView {
    view! { cx,
        <div>
            <button
                on:click=move|_| download_svg(&get_slug_from_modal_container())
                title=move_gettext!(cx, "Download SVG")
            >
                {move_gettext!(cx, "Download SVG")}
            </button>
            <a title=move_gettext!(cx, "Download colored SVG")>
                {move_gettext!(cx, "Download colored SVG")}
            </a>
            <button
                on:click=move|_| download_pdf(&get_slug_from_modal_container())
                title=move_gettext!(cx, "Download PDF")
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
        // The modal is initialized empty. Before open it will be filled with the
        // information of the icon
        <Modal
            title=move||"".to_string()
            is_open=move||current_icon_view().is_some()
            on_close=move|_|{
                current_icon_view.update(|state| *state = None);
            }
        >
            <div class="flex flex-col" id=ICON_DETAILS_MODAL_ID>
                <div class="flex flex-row flex-grow">
                    <IconDetailsModalPreview />
                    <IconDetailsModalInformation />
                </div>
                <IconDetailsModalFooter/>
            </div>
        </Modal>

    }
}
