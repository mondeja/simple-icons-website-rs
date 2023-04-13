//! Module to track the ids of components, to ensure that they are unique.
//!
//! It minifies identifiers in release mode.

pub enum Ids {
    SearchInput,
    CopyInput,
    IconDetailsModal,
    ViewSVGPath,
    DownloadSVGPath,
    ControlsSVGPath,
    CrossSVGPath,
}

impl Ids {
    pub fn as_str(&self) -> &'static str {
        #[cfg(debug_assertions)]
        match self {
            Ids::IconDetailsModal => "icon-details-modal",
            Ids::SearchInput => "search-input",
            Ids::CopyInput => "copy-input",
            Ids::ViewSVGPath => "view-path",
            Ids::DownloadSVGPath => "download-path",
            Ids::ControlsSVGPath => "controls-path",
            Ids::CrossSVGPath => "cross-path",
        }
        #[cfg(not(debug_assertions))]
        match self {
            Ids::IconDetailsModal => "i",
            Ids::SearchInput => "s",
            Ids::CopyInput => "t",
            Ids::ViewSVGPath => "v",
            Ids::DownloadSVGPath => "d",
            Ids::ControlsSVGPath => "c",
            Ids::CrossSVGPath => "x",
        }
    }
}
