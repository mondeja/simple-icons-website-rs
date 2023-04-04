pub mod pdf;
pub mod svg;

use crate::controls::download::pdf::maybe_initialize_pdfkit;
use crate::controls::ControlsStateSignal;
use leptos::*;
use std::fmt;

#[derive(Default, Copy, Clone, PartialEq)]
pub enum DownloadType {
    #[default]
    SVG,
    PDF,
}

impl DownloadType {
    fn from_str(s: &str) -> Self {
        match s {
            "svg" => Self::SVG,
            _ => Self::PDF,
        }
    }
}

impl fmt::Display for DownloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SVG => write!(f, "svg"),
            Self::PDF => write!(f, "pdf"),
        }
    }
}

pub fn initial_download_type_from_localstorage() -> DownloadType {
    let window = web_sys::window().unwrap();
    let local_storage = window.local_storage().unwrap().unwrap();

    let download_type = match local_storage.get_item("download-type") {
        Ok(Some(download_type)) => DownloadType::from_str(&download_type),
        _ => DownloadType::default(),
    };

    // if the download type is PDF we need to lazy load the PDFKit library
    if download_type == DownloadType::PDF {
        maybe_initialize_pdfkit();
    }

    download_type
}

fn set_download_type_in_localstorage(download_type: DownloadType) {
    let window = web_sys::window().unwrap();
    let local_storage = window.local_storage().unwrap().unwrap();
    local_storage
        .set_item("download-type", &download_type.to_string())
        .unwrap();
}

#[component]
pub fn DownloadFileTypeControl(cx: Scope) -> impl IntoView {
    let controls_state = use_context::<ControlsStateSignal>(cx).unwrap().0;

    view! { cx,
        <div class="flex flex-col">
            <label>"Download"</label>
            <div class="flex flex-row">
                <button
                    class=move || {
                        let mut class = "font-bold w-10 h-10 p-1.5 ".to_string();
                        if controls_state().download_type == DownloadType::SVG {
                            class.push_str("bg-black text-white");
                        } else {
                            class.push_str("bg-white text-black");
                        }
                        class
                    }
                    type="button"
                    title="Download SVG"
                    on:click=move |_| {
                        controls_state.update(move |mut state| {
                            state.download_type = DownloadType::SVG;
                            set_download_type_in_localstorage(state.download_type);
                        });
                    }
                >
                    <span>"SVG"</span>
                </button>
                <button
                    class=move || {
                        let mut class = "font-bold w-10 h-10 p-1.5 ".to_string();
                        if controls_state().download_type == DownloadType::PDF {
                            class.push_str("bg-black text-white");
                        } else {
                            class.push_str("bg-white text-black");
                        }
                        class
                    }
                    type="button"
                    title="Download PDF"
                    on:click=move |_| {
                        controls_state.update(|mut state| {
                            state.download_type = DownloadType::PDF;
                            set_download_type_in_localstorage(state.download_type);
                            maybe_initialize_pdfkit();
                        });
                    }
                >
                    <span>"PDF"</span>
                </button>
            </div>
        </div>
    }
}
