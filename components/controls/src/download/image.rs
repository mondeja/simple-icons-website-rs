use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/download/image.js")]
extern "C" {
    pub fn download_png_(slug: &str);
    pub fn download_jpg_(slug: &str);
    pub fn copy_as_base64_jpg_(slug: &str);
    pub fn copy_as_base64_png_(slug: &str);
    pub fn copy_as_image_png_(slug: &str);
    pub fn copy_as_image_jpg_(slug: &str);
}

pub fn download_png(slug: &str) {
    download_png_(slug);
}

pub fn download_jpg(slug: &str) {
    download_jpg_(slug);
}

pub fn copy_as_image_png(slug: &str) {
    copy_as_image_png_(slug);
}

pub fn copy_as_image_jpg(slug: &str) {
    copy_as_image_jpg_(slug);
}

pub fn copy_as_base64_jpg(slug: &str) {
    copy_as_base64_jpg_(slug);
}

pub fn copy_as_base64_png(slug: &str) {
    copy_as_base64_png_(slug);
}
