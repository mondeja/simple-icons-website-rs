use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

/// Download a SVG icon by its slug
pub fn download_svg(slug: &str) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let link: web_sys::HtmlElement = document
        .create_element("a")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();
    link.set_attribute("class", "hidden").unwrap();
    link.set_attribute("download", &format!("{}.svg", slug))
        .unwrap();
    link.set_attribute("href", &format!("/icons/{}.svg", slug))
        .unwrap();
    let body = document.body().unwrap();
    body.append_child(&link).unwrap();
    link.click();
    body.remove_child(&link).unwrap();
}
