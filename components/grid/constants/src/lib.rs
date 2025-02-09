use simple_icons_macros::{
    deprecated_icons_array, get_number_of_deprecated_icons,
    get_number_of_icons, icons_array,
};
use simple_icons_website_types::SimpleIcon;

pub static ICONS: [SimpleIcon; get_number_of_icons!()] = icons_array!();
pub static DEPRECATED_ICONS: [SimpleIcon; get_number_of_deprecated_icons!()] =
    deprecated_icons_array!();
