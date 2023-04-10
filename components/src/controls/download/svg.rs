use crate::controls::download::download;

/// Download a SVG icon by its slug
pub fn download_svg(slug: &str) {
    download(&format!("{}.svg", slug), &format!("/icons/{}.svg", slug));
}
