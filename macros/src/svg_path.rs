use std::fs;
use std::path::Path;

pub fn get_simple_icon_svg_path_by_slug(filename: &str) -> String {
    let icon_file_path =
        format!("node_modules/simple-icons/icons/{}.svg", filename);
    let icon_file_content =
        fs::read_to_string(Path::new(&icon_file_path)).unwrap();
    let icon_path = icon_file_content
        .split_once("d=\"")
        .unwrap()
        .1
        .split_once("\"")
        .unwrap()
        .0;
    icon_path.to_string()
}
