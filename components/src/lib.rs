#![feature(trait_alias)]
#![feature(const_option_ext)]
#![feature(stmt_expr_attributes)]
#![feature(async_closure)]

// TODO: In v0.3.0 Leptos will allow imports of components without needing to
// import the Props struct, so then refactor imports with `use my_crate::MyComponent;`
// instead of the current `use my_crate::*;`

pub mod controls;
pub mod copy;
pub mod footer;
pub mod grid;
pub mod header;
mod ids;
mod modal;
pub(crate) mod spinners;
pub(crate) mod storage;
pub mod svg_defs;
pub(crate) mod url;

pub(crate) use ids::Ids;
pub(crate) use url as Url;
