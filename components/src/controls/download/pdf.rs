use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/controls/download/pdf.js")]
extern "C" {
    pub fn download_pdf(slug: &str);
}

/// Lazy loading of PDFKit JS library
pub fn maybe_initialize_pdfkit() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    // Load PDFKit JS library if it's not already loaded
    if document
        .query_selector("script#pdfkit-standalone")
        .unwrap()
        .is_none()
    {
        let script = document.create_element("script").unwrap();
        script
            .set_attribute("src", "/pdfkit.standalone.js")
            .unwrap();
        script.set_attribute("id", "pdfkit-standalone").unwrap();
        document.body().unwrap().append_child(&script).unwrap();
    }

    // Load blob-stream JS library if it's not already loaded
    if document
        .query_selector("script#blob-stream-standalone")
        .unwrap()
        .is_none()
    {
        let script = document.create_element("script").unwrap();
        script.set_attribute("src", "/.js").unwrap();
        script
            .set_attribute("id", "blob-stream-standalone")
            .unwrap();
        document.body().unwrap().append_child(&script).unwrap();
    }
}
