mod deprecated;
mod third_party_extension;

pub use deprecated::DeprecatedIcon;
pub use third_party_extension::ThirdPartyExtension;

/// Static Simple Icon
#[derive(Clone)]
pub struct SimpleIcon {
    pub slug: &'static str,
    pub title: &'static str,
    pub hex: &'static str,
    pub hex_is_relatively_light: bool,
    pub source: &'static str,
    pub guidelines: Option<&'static str>,
    pub license_url: Option<&'static str>,
    pub license_type: Option<&'static str>,
    pub plain_aliases: &'static [&'static str],
    pub order_alpha: usize,
    pub order_color: usize,
    pub deprecation: Option<&'static DeprecatedIcon>,
}
