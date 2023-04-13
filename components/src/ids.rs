//! Module to track the ids of components, to ensure that they are unique.
//!
//! It minifies identifiers in release mode.

pub enum Ids {
    SearchInput,
    IconDetailsModal,
    ViewSVGPath,
    DownloadSVGPath,
    ControlsSVGPath,
    CrossSVGPath,
}

impl Ids {
    pub fn as_str(&self) -> &'static str {
        match self {
            #[cfg(debug_assertions)]
            Ids::IconDetailsModal => "icon-details-modal",
            #[cfg(not(debug_assertions))]
            Ids::IconDetailsModal => "i",

            #[cfg(debug_assertions)]
            Ids::SearchInput => "search-input",
            #[cfg(not(debug_assertions))]
            Ids::SearchInput => "s",

            #[cfg(debug_assertions)]
            Ids::ViewSVGPath => "view-path",
            #[cfg(not(debug_assertions))]
            Ids::ViewSVGPath => "v",

            #[cfg(debug_assertions)]
            Ids::DownloadSVGPath => "download-path",
            #[cfg(not(debug_assertions))]
            Ids::DownloadSVGPath => "d",

            #[cfg(debug_assertions)]
            Ids::ControlsSVGPath => "controls-path",
            #[cfg(not(debug_assertions))]
            Ids::ControlsSVGPath => "c",

            #[cfg(debug_assertions)]
            Ids::CrossSVGPath => "cross-path",
            #[cfg(not(debug_assertions))]
            Ids::CrossSVGPath => "x",
        }
    }
}
