use leptos::prelude::document;
use simple_icons_macros::js_library_version;
use wasm_bindgen::prelude::*;

static PDFKIT_VERSION: &str = js_library_version!("pdfkit");
static BLOB_STREAM_VERSION: &str = js_library_version!("blob-stream");

#[wasm_bindgen(module = "/src/controls/download/pdf.js")]
extern "C" {
    pub fn download_pdf_(
        slug: &str,
        errorMessageSchema: &str,
        pdfkitVersion: &str,
        blobStreamVersion: &str,
    );
    pub fn add_scripts_(pdfkitVersion: &str, blobStreamVersion: &str);
}

pub fn download_pdf(slug: &str) {
    let msg = document()
        .query_selector("[data-error-generating-pdf-msg]")
        .unwrap()
        .unwrap()
        .unchecked_into::<web_sys::HtmlButtonElement>()
        .get_attribute("data-error-generating-pdf-msg")
        .unwrap();

    download_pdf_(slug, &msg, PDFKIT_VERSION, BLOB_STREAM_VERSION);
}

pub fn add_pdfkit_scripts() {
    add_scripts_(PDFKIT_VERSION, BLOB_STREAM_VERSION);
}
