//! SVG definitions
//!
//! The definitions of the SVG paths used in the icons.
//! They are defined here to optimize the size of the site.
use leptos::*;
use std::fmt;

pub enum SVGDef {
    /// Path used in view detail buttons
    ViewPath,
    /// Path used in download buttons
    DownloadPath,
}

impl SVGDef {
    pub fn id(&self) -> char {
        match self {
            Self::ViewPath => 'v',
            Self::DownloadPath => 'd',
        }
    }
}

impl fmt::Display for SVGDef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ViewPath => write!(
                f,
                "m23.136 20.694-4.41-4.413a1.93 1.93 0 0 0-1.186-.551 9.632 9.632 0 0 0 2.13-6.044C19.67 4.344 15.325 0 9.983 0 4.642 0 .297 4.344.297 9.686c0 5.34 4.344 9.685 9.685 9.685 2.016 0 3.89-.62 5.44-1.677.01.48.195.957.563 1.325l4.413 4.413c.377.38.874.568 1.369.568s.992-.189 1.369-.568a1.935 1.935 0 0 0 0-2.738zm-13.154-4.55a6.465 6.465 0 0 1-6.458-6.458 6.465 6.465 0 0 1 6.458-6.458 6.465 6.465 0 0 1 6.458 6.458 6.465 6.465 0 0 1-6.458 6.458z"
            ),
            Self::DownloadPath => write!(
                f,
                "M11.2 0a.8.8 0 0 0-.8.8v11.4L7.26 9.44a.803.803 0 0 0-1.13.074l-1.05 1.2a.8.8 0 0 0 .073 1.13l6.33 5.54a.795.795 0 0 0 1.05 0l6.32-5.54a.8.8 0 0 0 .074-1.13l-1.05-1.2a.804.804 0 0 0-1.13-.074l-3.14 2.76V.8a.8.8 0 0 0-.8-.8zm-8 20.8a.8.8 0 0 0-.8.8v1.6a.8.8 0 0 0 .8.8h17.6a.8.8 0 0 0 .8-.8v-1.6a.8.8 0 0 0-.8-.8z"
            ),
        }
    }
}

#[component]
pub fn SVGDefs(cx: Scope) -> impl IntoView {
    view! { cx,
        <svg class="hidden">
            <defs>
                <path
                    id=SVGDef::ViewPath.id()
                    d=SVGDef::ViewPath.to_string()
                />
                <path
                    id=SVGDef::DownloadPath.id()
                    d=SVGDef::DownloadPath.to_string()
                />
            </defs>
        </svg>
    }
}
