//! Macros for simple-icons-website
//!
//! These macros are used to generate code at compile time.

use proc_macro::TokenStream;
use std::fs;
use std::path::Path;
use syn::{parse_macro_input, LitStr};

/// Get number of icons available in the simple-icons npm package
#[proc_macro]
pub fn number_of_icons(_: TokenStream) -> TokenStream {
    Path::new("node_modules/simple-icons/icons")
        .read_dir()
        .unwrap()
        .count()
        .to_string()
        .parse()
        .unwrap()
}

/// Include the SVG path of a simple icon
#[proc_macro]
pub fn simple_icon_svg_path(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);

    // get value of the string literal.
    let file_name = input.value();

    let icon_file_path = format!("node_modules/simple-icons/icons/{}", file_name);
    let icon_file_path = Path::new(&icon_file_path);
    let icon_file_content = fs::read_to_string(icon_file_path).unwrap();
    let icon_path = icon_file_content
        .split_once("d=\"")
        .unwrap()
        .1
        .split_once("\"")
        .unwrap()
        .0;
    format!("{:?}", icon_path).parse().unwrap()
}
