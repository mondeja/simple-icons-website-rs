use rand::{prelude::SliceRandom, thread_rng};
use simple_icons_website_types::SimpleIcon;
use std::str::FromStr;

#[derive(Default, Copy, Clone, PartialEq)]
pub enum OrderModeVariant {
    Alphabetic,
    Color,
    SearchMatch,
    #[default]
    Random,
}

impl From<&str> for OrderModeVariant {
    fn from(order_mode: &str) -> Self {
        match order_mode {
            "alpha" => Self::Alphabetic,
            "color" => Self::Color,
            "random" => Self::Random,
            _ => Self::SearchMatch,
        }
    }
}

impl core::fmt::Display for OrderModeVariant {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Alphabetic => write!(f, "alpha"),
            Self::Color => write!(f, "color"),
            Self::SearchMatch => write!(f, "search"),
            Self::Random => write!(f, "random"),
        }
    }
}

#[derive(Copy, Clone, Default)]
pub struct OrderMode {
    /// The order mode preferred by the user
    pub favorite: OrderModeVariant,
    /// The order mode currently in use
    pub current: OrderModeVariant,
}

impl FromStr for OrderMode {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            favorite: OrderModeVariant::from(value),
            current: OrderModeVariant::from(value),
        })
    }
}

pub fn sort_icons(order_mode: &OrderModeVariant, icons: &mut Vec<&SimpleIcon>) {
    match order_mode {
        OrderModeVariant::Alphabetic => {
            icons.sort_by(|a, b| a.order_alpha.cmp(&b.order_alpha));
        }
        OrderModeVariant::Color => {
            icons.sort_by(|a, b| a.order_color.cmp(&b.order_color));
        }
        OrderModeVariant::Random => {
            icons.shuffle(&mut thread_rng());
        }
        _ => {
            // Search match order is handled by the search control
        }
    }
}
