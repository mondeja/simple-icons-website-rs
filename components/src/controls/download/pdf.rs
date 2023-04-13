use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlButtonElement};

#[wasm_bindgen(module = "/src/controls/download/pdf.js")]
extern "C" {
    pub fn download_pdf_(slug: &str, errorMessageSchema: String);
}

pub fn download_pdf(slug: &str) {
    let msg_schema = window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector("[data-error-generating-pdf-msg-schema]")
        .unwrap()
        .unwrap()
        .dyn_into::<HtmlButtonElement>()
        .unwrap()
        .get_attribute("data-error-generating-pdf-msg-schema")
        .unwrap();

    download_pdf_(slug, msg_schema);
}

/// Lazy loading of PDFKit JS library
pub fn maybe_initialize_pdfkit() {
    let document = web_sys::window().unwrap().document().unwrap();
    let head = document.head().unwrap();

    // Load PDFKit JS library if it's not already loaded
    if document
        .query_selector("script#pdfkit-standalone")
        .unwrap()
        .is_none()
    {
        let script = document.create_element("script").unwrap();
        script
            .set_attribute("src", "./pdfkit.standalone.js")
            .unwrap();
        script.set_attribute("id", "pdfkit-standalone").unwrap();
        head.append_child(&script).unwrap();
    }

    // Load blob-stream JS library if it's not already loaded
    if document
        .query_selector("script#blob-stream-standalone")
        .unwrap()
        .is_none()
    {
        let script = document.create_element("script").unwrap();
        script.set_attribute("src", "./blob-stream.js").unwrap();
        script
            .set_attribute("id", "blob-stream-standalone")
            .unwrap();
        head.append_child(&script).unwrap();
    }
}
