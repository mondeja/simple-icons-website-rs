//! Macros for simple-icons-website
//!
//! These macros are used to generate code at compile time.

use proc_macro::TokenStream;
use std::fs;
use std::path::Path;
use syn::{parse_macro_input, LitStr};

/// Get number of icons available in the simple-icons npm package
#[proc_macro]
pub fn get_number_of_icons(_: TokenStream) -> TokenStream {
    Path::new("node_modules/simple-icons/icons")
        .read_dir()
        .unwrap()
        .count()
        .to_string()
        .parse()
        .unwrap()
}

fn simple_icon_svg_path_impl(filename: &str) -> String {
    let icon_file_path =
        format!("node_modules/simple-icons/icons/{}", filename);
    let icon_file_content =
        fs::read_to_string(Path::new(&icon_file_path)).unwrap();
    let icon_path = icon_file_content
        .split_once("d=\"")
        .unwrap()
        .1
        .split_once("\"")
        .unwrap()
        .0;
    format!("{:?}", icon_path)
}

/// Get a string with the SVG path of a simple icon
#[proc_macro]
pub fn simple_icon_svg_path(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    simple_icon_svg_path_impl(input.value().as_str())
        .parse()
        .unwrap()
}

/// Get the extensions of Simple Icons from the README file of the npm package
#[proc_macro]
pub fn get_simple_icons_3rd_party_extensions(_: TokenStream) -> TokenStream {
    let readme_file_content =
        fs::read_to_string(Path::new("node_modules/simple-icons/README.md"))
            .unwrap();

    let mut extensions_array_code = "&[".to_string();

    let extensions_table_lines = readme_file_content
        .split_once("## Third-Party Extensions")
        .unwrap()
        .1
        .split_once("| :-- | :-- |")
        .unwrap()
        .1
        .split("\n|");
    for line in extensions_table_lines {
        if line.trim().is_empty() {
            continue;
        }
        let name = line.split_once("[").unwrap().1.split_once("](").unwrap().0;
        let url = line.split_once("](").unwrap().1.split_once(")").unwrap().0;
        let author_name = line
            .split_once("|")
            .unwrap()
            .1
            .split_once("[")
            .unwrap()
            .1
            .split_once("]")
            .unwrap()
            .0;
        let author_url = line
            .split_once("|")
            .unwrap()
            .1
            .split_once("](")
            .unwrap()
            .1
            .split_once(")")
            .unwrap()
            .0;
        let icon_file_name = line
            .split_once("<img src=\"")
            .unwrap()
            .1
            .split_once("\"")
            .unwrap()
            .0
            .split("/")
            .last()
            .unwrap();

        // We can't expose a struct from a procedural macro crate,
        // so the extension struct is provided the `types` crate
        extensions_array_code.push_str(&format!(
            concat!(
                "::types::SimpleIconsExtension{{",
                "name: \"{}\",",
                "url: \"{}\",",
                "author_name: \"{}\",",
                "author_url: \"{}\",",
                "icon_svg_path: {},",
                "}},"
            ),
            name,
            url,
            author_name,
            author_url,
            simple_icon_svg_path_impl(icon_file_name)
        ));
    }
    extensions_array_code.push_str("]");
    extensions_array_code.parse().unwrap()
}
