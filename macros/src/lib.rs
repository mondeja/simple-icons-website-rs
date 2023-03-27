//! Macros for simple-icons-website
//!
//! These macros are used to generate code at compile time.

use proc_macro::TokenStream;
use std::path::Path;

/// Returns the number of icons available in the simple-icons npm package
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
