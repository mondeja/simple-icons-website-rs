//! Macros for simple-icons-website
//!
//! These macros are used to generate code at compile time.

mod color;

use color::{get_relative_luminance, sort_hexes};
use config::CONFIG;
use proc_macro::TokenStream;
use simple_icons::{
    fetch_deprecated_simple_icons, get_simple_icon_svg_path_by_slug,
    get_simple_icons,
};
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
                "icon_slug: \"{}\",",
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
pub fn simple_icons_array(_: TokenStream) -> TokenStream {
    let max_icons = CONFIG.max_icons;
    let simple_icons = get_simple_icons(max_icons);

    let hexes = simple_icons
        .iter()
        .map(|icon| icon.hex.clone())
        .collect::<Vec<_>>();

    let sorted_hexes = sort_hexes(&hexes);

    let deprecated_icons = fetch_deprecated_simple_icons();

    let mut simple_icons_array_code = "[".to_string();
    for (i, icon) in simple_icons.iter().enumerate() {
        // color order index
        let order_color = sorted_hexes
            .iter()
            .position(|hex| *hex == icon.hex)
            .unwrap();

        let deprecated_icon = deprecated_icons
            .iter()
            .find(|deprecated_icon| deprecated_icon.slug == icon.slug);

        simple_icons_array_code.push_str(&format!(
            concat!(
                "::simple_icons::StaticSimpleIcon{{",
                "slug: \"{}\",",
                "title: \"{}\",",
                "hex: \"{}\",",
                "hex_is_relatively_light: {},",
                "source_url: \"{}\",",
                "guidelines_url: {},",
                "license_url: {},",
                "license_type: {},",
                // `get_simple_icons` function returns icons in alphabetical order
                // because they are extracted from the `simple-icons.json` file
                "order_alpha: {},",
                "order_color: {},",
                "is_deprecated: {},",
                "deprecation_pull_request_url: {},",
                "removal_at_version: {},",
                "}},"
            ),
            icon.slug,
            icon.title,
            icon.hex,
            get_relative_luminance(&icon.hex) >= 0.4,
            icon.source_url,
            match icon.guidelines_url {
                Some(ref url) => format!("Some(\"{}\")", url),
                None => "None".to_string(),
            },
            match icon.license {
                Some(ref license) => match license.url {
                    Some(ref url) => format!("Some(\"{}\")", url),
                    None => "None".to_string(),
                },
                None => "None".to_string(),
            },
            match icon.license {
                Some(ref license) => format!("Some(\"{}\")", license.type_),
                None => "None".to_string(),
            },
            i,
            order_color,
            deprecated_icon.is_some(),
            match deprecated_icon.is_some() {
                true => format!(
                    "Some(\"{}\")",
                    deprecated_icon.unwrap().pull_request_url
                ),
                false => "None".to_string(),
            },
            match deprecated_icon.is_some() {
                true => format!(
                    "Some(\"{}\")",
                    deprecated_icon.unwrap().removal_at_version
                ),
                false => "None".to_string(),
            },
        ));
    }
    simple_icons_array_code.push_str("]");

    simple_icons_array_code.parse().unwrap()
}
