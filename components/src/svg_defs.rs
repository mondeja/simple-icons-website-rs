//! SVG definitions
//!
//! The definitions of the SVG paths used in the icons.
//! They are defined here to optimize the size of the site.
use leptos::*;
use std::fmt;

pub enum SVGDefs {
    /// Path used in view detail buttons
    ViewPath,
    /// Path used in download buttons
    DownloadPath,
    /// Path to display a controls icon
    ControlsPath,
    /// Path to display a cross icon
    CrossPath,
}

impl SVGDefs {
    pub fn id(&self) -> char {
        match self {
            Self::ViewPath => 'v',
            Self::DownloadPath => 'd',
            Self::ControlsPath => 'c',
            Self::CrossPath => 'x',
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ViewPath => "m23.136 20.694-4.41-4.413a1.93 1.93 0 0 0-1.186-.551 9.632 9.632 0 0 0 2.13-6.044C19.67 4.344 15.325 0 9.983 0 4.642 0 .297 4.344.297 9.686c0 5.34 4.344 9.685 9.685 9.685 2.016 0 3.89-.62 5.44-1.677.01.48.195.957.563 1.325l4.413 4.413c.377.38.874.568 1.369.568s.992-.189 1.369-.568a1.935 1.935 0 0 0 0-2.738zm-13.154-4.55a6.465 6.465 0 0 1-6.458-6.458 6.465 6.465 0 0 1 6.458-6.458 6.465 6.465 0 0 1 6.458 6.458 6.465 6.465 0 0 1-6.458 6.458z",
            Self::DownloadPath => "M11.2 0a.8.8 0 0 0-.8.8v11.4L7.26 9.44a.803.803 0 0 0-1.13.074l-1.05 1.2a.8.8 0 0 0 .073 1.13l6.33 5.54a.795.795 0 0 0 1.05 0l6.32-5.54a.8.8 0 0 0 .074-1.13l-1.05-1.2a.804.804 0 0 0-1.13-.074l-3.14 2.76V.8a.8.8 0 0 0-.8-.8zm-8 20.8a.8.8 0 0 0-.8.8v1.6a.8.8 0 0 0 .8.8h17.6a.8.8 0 0 0 .8-.8v-1.6a.8.8 0 0 0-.8-.8z",
            Self::ControlsPath => "M3.429 0A3.442 3.442 0 0 0 0 3.429a3.439 3.439 0 0 0 3.429 3.428c1.585 0 2.931-1.097 3.317-2.571h16.397A.857.857 0 0 0 24 3.429a.857.857 0 0 0-.857-.858H6.749A3.447 3.447 0 0 0 3.43 0Zm0 1.714c.956 0 1.714.76 1.714 1.715a1.7 1.7 0 0 1-1.714 1.714 1.699 1.699 0 0 1-1.715-1.714c0-.955.756-1.715 1.715-1.715zM12 8.571a3.446 3.446 0 0 0-3.319 2.572H.857A.857.857 0 0 0 0 12a.857.857 0 0 0 .857.857h7.828A3.446 3.446 0 0 0 12 15.43c1.586 0 2.931-1.098 3.317-2.572h7.826A.857.857 0 0 0 24 12a.857.857 0 0 0-.857-.857H15.32A3.447 3.447 0 0 0 12 8.57Zm0 1.715c.957 0 1.714.76 1.714 1.714A1.7 1.7 0 0 1 12 13.714 1.699 1.699 0 0 1 10.286 12c0-.955.756-1.714 1.714-1.714zm8.571 6.857a3.446 3.446 0 0 0-3.318 2.571H.857a.857.857 0 0 0-.857.857.857.857 0 0 0 .857.858h16.399A3.446 3.446 0 0 0 20.571 24 3.44 3.44 0 0 0 24 20.571a3.443 3.443 0 0 0-3.429-3.428zm0 1.714c.957 0 1.715.76 1.715 1.714a1.7 1.7 0 0 1-1.715 1.715 1.699 1.699 0 0 1-1.714-1.715c0-.954.756-1.714 1.714-1.714z",
            Self::CrossPath => "M12 10.586l5.657-5.657a1 1 0 1 1 1.414 1.414L13.414 12l5.657 5.657a1 1 0 0 1-1.414 1.414L12 13.414l-5.657 5.657a1 1 0 0 1-1.414-1.414L10.586 12 4.93 6.343a1 1 0 0 1 1.414-1.414L12 10.586z",
        }
    }
}

impl fmt::Display for SVGDefs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(self.as_str())
    }
}

#[component]
pub fn SVGDefsDefinition(cx: Scope) -> impl IntoView {
    view! { cx,
        <svg class="hidden">
            <defs>
                <path id=SVGDefs::ViewPath.id() d=SVGDefs::ViewPath.as_str()></path>
                <path id=SVGDefs::DownloadPath.id() d=SVGDefs::DownloadPath.as_str()></path>
                <path id=SVGDefs::ControlsPath.id() d=SVGDefs::ControlsPath.as_str()></path>
                <path id=SVGDefs::CrossPath.id() d=SVGDefs::CrossPath.as_str()></path>
            </defs>
        </svg>
    }
}
