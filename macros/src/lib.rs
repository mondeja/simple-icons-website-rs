//! Macros for simple-icons-website
//!
//! These macros are used to generate code at compile time.

mod color;

use color::{get_relative_luminance, sort_hexes};
use config::CONFIG;
use proc_macro::TokenStream;
use simple_icons::{
    get_simple_icon_svg_path, get_simple_icons,
    sdk::fetch_deprecated_simple_icons,
};
use std::fs;
use std::path::Path;
use syn::{parse_macro_input, LitStr};

/// Get number of icons available in the simple-icons npm package
#[proc_macro]
pub fn get_number_of_icons(_: TokenStream) -> TokenStream {
    if let Some(max_icons) = CONFIG.max_icons {
        max_icons.to_string().parse().unwrap()
    } else {
        Path::new("node_modules/simple-icons/icons")
            .read_dir()
            .unwrap()
            .count()
            .to_string()
            .parse()
            .unwrap()
    }
}

/// Get a string with the SVG path of a simple icon
#[proc_macro]
pub fn simple_icon_svg_path(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let svg_path = get_simple_icon_svg_path(input.value().as_str());
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
        let name = line.split_once('[').unwrap().1.split_once("](").unwrap().0;
        let url = line.split_once("](").unwrap().1.split_once(')').unwrap().0;
        let author_name = line
            .split_once('|')
            .unwrap()
            .1
            .split_once('[')
            .unwrap()
            .1
            .split_once(']')
            .unwrap()
            .0;
        let author_url = line
            .split_once('|')
            .unwrap()
            .1
            .split_once("](")
            .unwrap()
            .1
            .split_once(')')
            .unwrap()
            .0;
        let icon_slug = line
            .split_once("<img src=\"")
            .unwrap()
            .1
            .split_once('"')
            .unwrap()
            .0
            .split('/')
            .last()
            .unwrap()
            .split_once(".svg")
            .unwrap()
            .0;

        extensions_array_code.push_str(&format!(
            concat!(
                "::types::ThirdPartyExtension{{",
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
            get_simple_icon_svg_path(icon_slug)
        ));
    }
    extensions_array_code.push(']');
    extensions_array_code.parse().unwrap()
}

#[proc_macro]
pub fn icons_array(_: TokenStream) -> TokenStream {
    let max_icons = CONFIG.max_icons;
    let simple_icons = get_simple_icons(max_icons);

    let hexes = simple_icons
        .iter()
        .map(|icon| icon.hex.clone())
        .collect::<Vec<_>>();
    let sorted_hexes = sort_hexes(&hexes);

    let deprecated_icons = fetch_deprecated_simple_icons();

    let mut icons_array_code = "[".to_string();
    for (i, icon) in simple_icons.iter().enumerate() {
        // color order index
        let order_color = sorted_hexes
            .iter()
            .position(|hex| *hex == icon.hex)
            .unwrap();

        let deprecated_icon = deprecated_icons
            .iter()
            .find(|deprecated_icon| *deprecated_icon.slug == icon.slug);

        icons_array_code.push_str(&format!(
            concat!(
                "::types::SimpleIcon{{",
                "slug: \"{}\",",
                "title: \"{}\",",
                "hex: \"{}\",",
                "hex_is_relatively_light: {},",
                "source: \"{}\",",
                "guidelines: {},",
                "license_url: {},",
                "license_type: {},",
                "plain_aliases: {},",
                // `get_simple_icons` function returns icons in alphabetical order
                // because they are extracted from the `simple-icons.json` file
                "order_alpha: {},",
                "order_color: {},",
                "deprecation: {},",
                "}},"
            ),
            icon.slug,
            icon.title,
            icon.hex,
            get_relative_luminance(&icon.hex) >= 0.4,
            icon.source,
            match icon.guidelines {
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
            match &icon.aliases {
                None => "&[]".to_string(),
                Some(aliases) => format!("&[{}]", {
                    let mut ret = vec![];
                    if let Some(aka) = aliases.aka.clone() {
                        ret.extend(aka);
                    };
                    if aliases.dup.is_some() {
                        ret.extend(
                            aliases
                                .dup
                                .clone()
                                .unwrap()
                                .iter()
                                .map(|dup| dup.title.clone()),
                        );
                    }
                    if aliases.loc.is_some() {
                        ret.extend(
                            aliases
                                .loc
                                .clone()
                                .unwrap()
                                .values()
                                .map(|v| v.to_string()),
                        );
                    }
                    ret.iter()
                        .map(|alias| format!("\"{}\"", alias))
                        .collect::<Vec<_>>()
                        .join(", ")
                }),
            },
            i,
            order_color,
            match deprecated_icon {
                Some(icon) => {
                    format!(
                        concat!(
                            "Some(",
                            "&::types::IconDeprecation{{",
                            "removal_at_version: \"{}\",",
                            "milestone_number: {},",
                            "milestone_due_on: \"{}\",",
                            "pull_request_number: {},",
                            "}}",
                            ")",
                        ),
                        icon.removal_at_version,
                        icon.milestone_number,
                        icon.milestone_due_on,
                        icon.pull_request_number,
                    )
                }
                None => "None".to_string(),
            },
        ));
    }
    icons_array_code.push(']');

    icons_array_code.parse().unwrap()
}
