//! Macros for simple-icons-website
//!
//! These macros are used to generate code at compile time.

mod svg_path;

use crate::svg_path::get_simple_icon_svg_path_by_slug;
use proc_macro::TokenStream;
use simple_icons::get_simple_icons;
use std::fs;
use std::path::Path;
use syn::{parse_macro_input, LitInt, LitStr};

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

/// Get a string with the SVG path of a simple icon
#[proc_macro]
pub fn simple_icon_svg_path(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let svg_path = get_simple_icon_svg_path_by_slug(input.value().as_str());
    format!("{:?}", svg_path).parse().unwrap()
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
        let icon_slug = line
            .split_once("<img src=\"")
            .unwrap()
            .1
            .split_once("\"")
            .unwrap()
            .0
            .split("/")
            .last()
            .unwrap()
            .split_once(".svg")
            .unwrap()
            .0;

        // We can't expose a struct from a procedural macro crate,
        // so the extension struct is provided the `simple_icons` crate
        extensions_array_code.push_str(&format!(
            concat!(
                "::simple_icons::SimpleIconsExtension{{",
                "name: \"{}\",",
                "url: \"{}\",",
                "author_name: \"{}\",",
                "author_url: \"{}\",",
                "icon_svg_path: \"{}\",",
                "}},"
            ),
            name,
            url,
            author_name,
            author_url,
            get_simple_icon_svg_path_by_slug(icon_slug)
        ));
    }
    extensions_array_code.push_str("]");
    extensions_array_code.parse().unwrap()
}

#[proc_macro]
pub fn simple_icons_array(input: TokenStream) -> TokenStream {
    let max_icons = parse_macro_input!(input as LitInt)
        .base10_digits()
        .parse::<usize>()
        .unwrap();
    let simple_icons = get_simple_icons(Some(max_icons));

    let mut simple_icons_array_code = "[".to_string();
    for icon in simple_icons {
        simple_icons_array_code.push_str(&format!(
            concat!(
                "::simple_icons::StaticSimpleIcon{{",
                "slug: \"{}\",",
                "title: \"{}\",",
                "hex: \"{}\",",
                "source: \"{}\",",
                "}},"
            ),
            icon.slug, icon.title, icon.hex, icon.source,
        ));
    }
    simple_icons_array_code.push_str("]");

    simple_icons_array_code.parse().unwrap()
}
