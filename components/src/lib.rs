#![feature(trait_alias)]
#![feature(stmt_expr_attributes)]
#![feature(async_closure)]

pub mod controls;
pub mod copy;
pub(crate) mod fetch;
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
