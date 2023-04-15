pub mod pdf;
pub mod svg;

use crate::controls::button::*;
use crate::controls::download::pdf::maybe_initialize_pdfkit;
use crate::storage::LocalStorage;
use i18n::move_gettext;
use leptos::{window, *};
pub use pdf::download_pdf;
use std::fmt;
pub use svg::download_svg;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

#[derive(Default, Copy, Clone, PartialEq)]
pub enum DownloadType {
    #[default]
    SVG,
    PDF,
}

impl From<&str> for DownloadType {
    fn from(download_type: &str) -> Self {
        match download_type {
            "svg" => Self::SVG,
            _ => Self::PDF,
        }
    }
}

impl fmt::Display for DownloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SVG => write!(f, "svg"),
            Self::PDF => write!(f, "pdf"),
        }
    }
}

pub fn provide_download_type_context(cx: Scope) {
    provide_context(
        cx,
        DownloadTypeSignal(create_rw_signal(
            cx,
            initial_download_type_from_localstorage(),
        )),
    );
}

#[derive(Copy, Clone)]
pub struct DownloadTypeSignal(pub RwSignal<DownloadType>);

fn initial_download_type_from_localstorage() -> DownloadType {
    let local_storage = window().local_storage().unwrap().unwrap();

    let download_type = match local_storage
        .get_item(LocalStorage::Keys::DownloadType.as_str())
    {
        Ok(Some(download_type)) => DownloadType::from(download_type.as_str()),
        _ => DownloadType::default(),
    };

    // if the download type is PDF we need to lazy load the PDFKit library
    if download_type == DownloadType::PDF {
        maybe_initialize_pdfkit();
    }

    download_type
}

fn set_download_type_on_localstorage(download_type: DownloadType) {
    let local_storage = window().local_storage().unwrap().unwrap();
    local_storage
        .set_item(
            LocalStorage::Keys::DownloadType.as_str(),
            &download_type.to_string(),
        )
        .unwrap();
}

#[component]
pub fn DownloadFileTypeControl(cx: Scope) -> impl IntoView {
    let download_type = use_context::<DownloadTypeSignal>(cx).unwrap().0;

    view! { cx,
        <div class="control">
            <label>{move_gettext!(cx, "Download")}</label>
            <div class="flex flex-row">
                <ControlButtonText
                    text=move_gettext!(cx, "SVG")
                    title=move_gettext!(cx, "Download SVG")
                    active=move || { download_type() == DownloadType::SVG }
                    on:click=move |_| {
                        download_type
                            .update(move |state| {
                                *state = DownloadType::SVG;
                                set_download_type_on_localstorage(*state);
                            });
                    }
                />
                <ControlButtonText
                    text=move_gettext!(cx, "PDF")
                    title=move_gettext!(cx, "Download PDF")
                    active=move || { download_type() == DownloadType::PDF }
                    on:click=move |_| {
                        download_type
                            .update(|state| {
                                *state = DownloadType::PDF;
                                set_download_type_on_localstorage(*state);
                                maybe_initialize_pdfkit();
                            });
                    }
                />
            </div>
        </div>
    }
}

/// Download a SVG icon by its slug
pub fn download(filename: &str, href: &str) {
    let document = window().document().unwrap();
    let link: web_sys::HtmlElement = document
        .create_element("a")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();
    link.set_attribute("class", "hidden").unwrap();
    link.set_attribute("download", filename).unwrap();
    link.set_attribute("href", href).unwrap();
    let body = document.body().unwrap();
    body.append_child(&link).unwrap();
    link.click();
    body.remove_child(&link).unwrap();
}
