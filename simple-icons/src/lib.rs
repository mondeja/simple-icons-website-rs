mod data;
mod deprecated;
mod svg_path;

pub use crate::data::{
    get_simple_icons_data, SimpleIconData, SimpleIconDataAliases,
    SimpleIconDataLicense,
};
pub use deprecated::{fetch_deprecated_simple_icons, DeprecatedIcon};
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
pub struct StaticSimpleIcon {
    pub slug: &'static str,
    pub title: &'static str,
    pub hex: &'static str,
    pub hex_is_relatively_light: bool,
    pub source_url: &'static str,
    pub guidelines_url: Option<&'static str>,
    pub license_url: Option<&'static str>,
    pub license_type: Option<&'static str>,
    pub plain_aliases: &'static [&'static str],
    pub order_alpha: usize,
    pub order_color: usize,
    pub is_deprecated: bool,
    pub deprecation_pull_request_url: Option<&'static str>,
    pub removal_at_version: Option<&'static str>,
}

#[derive(Clone)]
pub struct SimpleIcon {
    pub slug: String,
    pub title: String,
    pub hex: String,
    pub source_url: String,
    pub guidelines_url: Option<String>,
    pub license: Option<SimpleIconDataLicense>,
    pub aliases: Option<SimpleIconDataAliases>,
}

fn title_to_slug_replace_chars(title: &str) -> String {
    let mut new_title = String::with_capacity(title.len());
    for c in title.chars() {
        match c {
            'a'..='z' | '0'..='9' => new_title.push(c),
            '+' => new_title.push_str("plus"),
            '.' => new_title.push_str("dot"),
            '&' => new_title.push_str("and"),
            'đ' => new_title.push('d'),
            'ħ' => new_title.push('h'),
            'ı' => new_title.push('i'),
            'ĸ' => new_title.push('k'),
            'ŀ' => new_title.push('l'),
            'ł' => new_title.push('l'),
            'ß' => new_title.push_str("ss"),
            'ŧ' => new_title.push('t'),
            // The next implementation differs from the one in Javascript
            // TODO: should this be reported to the unicode_normalization
            // crate? Investigate
            'á' => new_title.push('a'),
            'é' => new_title.push('e'),
            'í' => new_title.push('i'),
            'ó' => new_title.push('o'),
            'ú' => new_title.push('u'),
            'ä' => new_title.push('a'),
            'ë' => new_title.push('e'),
            'ï' => new_title.push('i'),
            'ö' => new_title.push('o'),
            'ü' => new_title.push('u'),
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
    let simple_icons_data = get_simple_icons_data();
    let mut simple_icons: Vec<SimpleIcon> =
        Vec::with_capacity(simple_icons_data.icons.len());

    for icon_data in simple_icons_data.icons {
        let icon = SimpleIcon {
            slug: match icon_data.slug {
                Some(slug) => slug,
                None => title_to_slug(&icon_data.title),
            },
            title: icon_data.title,
            hex: icon_data.hex,
            source_url: icon_data.source,
            guidelines_url: icon_data.guidelines,
            license: icon_data.license,
            aliases: icon_data.aliases,
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
