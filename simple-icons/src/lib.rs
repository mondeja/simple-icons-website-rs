mod svg_path;

use serde::Deserialize;
use serde_json;
use std::fs;
use std::path::Path;
pub use svg_path::get_simple_icon_svg_path_by_slug;
use unicode_normalization::UnicodeNormalization;

/// Third party extensions of Simple Icons
pub struct SimpleIconsExtension {
    pub name: &'static str,
    pub url: &'static str,
    pub author_name: &'static str,
    pub author_url: &'static str,
    pub icon_slug: &'static str,
}

#[derive(Clone, Copy)]
pub struct FullStaticSimpleIcon {
    pub slug: &'static str,
    pub title: &'static str,
    pub hex: &'static str,
    pub source: &'static str,
    pub order_alpha: usize,
    pub order_color: usize,
}

#[derive(Clone)]
pub struct SimpleIcon {
    pub slug: String,
    pub title: String,
    pub hex: String,
    pub source: String,
}

#[derive(Deserialize)]
pub struct SimpleIconData {
    pub slug: Option<String>,
    pub title: String,
    pub hex: String,
    pub source: String,
}

#[derive(Deserialize)]
pub struct SimpleIconsData {
    pub icons: Vec<SimpleIconData>,
}

fn title_to_slug_replace_chars(title: &str) -> String {
    let mut new_title = String::with_capacity(title.len());
    for c in title.chars() {
        match c {
            '+' => new_title.push_str("plus"),
            '.' => new_title.push_str("dot"),
            '&' => new_title.push_str("and"),
            'đ' => new_title.push_str("d"),
            'ħ' => new_title.push_str("h"),
            'ı' => new_title.push_str("i"),
            'ĸ' => new_title.push_str("k"),
            'ŀ' => new_title.push_str("l"),
            'ł' => new_title.push_str("l"),
            'ß' => new_title.push_str("ss"),
            'ŧ' => new_title.push_str("t"),
            'a'..='z' | '0'..='9' => new_title.push(c),
            _ => continue,
        }
    }
    new_title
}

/// Convert a brand title to slug
///
/// This is a reimplementation of the
/// [`titleToSlug` function](https://github.com/simple-icons/simple-icons/blob/e050ace065412ded512c02d619cdfa347bfefb8d/scripts/utils.js#L59)
/// defined in the main simple-icons repository
fn title_to_slug(title: &str) -> String {
    title_to_slug_replace_chars(&title.to_lowercase())
        .nfc()
        .collect::<String>()
}

/// Get simple icons
pub fn get_simple_icons(max_icons: Option<usize>) -> Vec<SimpleIcon> {
    let mut simple_icons: Vec<SimpleIcon> = Vec::new();

    let icons_data_file =
        Path::new("node_modules/simple-icons/_data/simple-icons.json");
    let icons_data_raw = fs::read_to_string(icons_data_file)
        .expect("Could not read simple-icons.json file");
    let icons_data: SimpleIconsData = serde_json::from_str(&icons_data_raw)
        .expect("JSON was not well-formatted");

    for icon in icons_data.icons {
        let slug = match icon.slug {
            Some(slug) => slug,
            None => title_to_slug(&icon.title),
        };

        let icon = SimpleIcon {
            slug,
            title: icon.title,
            hex: icon.hex,
            source: icon.source,
        };
        simple_icons.push(icon);

        if let Some(max_icons) = max_icons {
            if simple_icons.len() == max_icons {
                break;
            }
        }
    }

    simple_icons
}
