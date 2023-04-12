use crate::controls::button::*;
use crate::storage::LocalStorage;
use i18n::move_gettext;
use leptos::*;
use std::fmt;

#[derive(Default, Copy, Clone, PartialEq)]
pub enum Layout {
    #[default]
    Comfortable,
    Compact,
}

impl From<&str> for Layout {
    fn from(layout: &str) -> Self {
        match layout {
            "comfortable" => Self::Comfortable,
            _ => Self::Compact,
        }
    }
}

impl fmt::Display for Layout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Comfortable => write!(f, "comfortable"),
            Self::Compact => write!(f, "compact"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct LayoutSignal(pub RwSignal<Layout>);

pub fn initial_layout_from_localstorage() -> Layout {
    let window = web_sys::window().unwrap();
    let local_storage = window.local_storage().unwrap().unwrap();

    match local_storage.get_item(LocalStorage::Keys::Layout.as_str()) {
        Ok(Some(layout)) => Layout::from(layout.as_str()),
        _ => Layout::default(),
    }
}

fn set_layout_on_localstorage(layout: &Layout) {
    let window = web_sys::window().unwrap();
    let local_storage = window.local_storage().unwrap().unwrap();
    local_storage
        .set_item(LocalStorage::Keys::Layout.as_str(), &layout.to_string())
        .unwrap();
}

fn set_layout(layout: Layout, layout_signal: &RwSignal<Layout>) {
    layout_signal.update(move |state| *state = layout);
    set_layout_on_localstorage(&layout);
}

#[component]
pub fn LayoutControl(cx: Scope) -> impl IntoView {
    let layout = use_context::<LayoutSignal>(cx).unwrap().0;

    view! { cx,
        <div class="control">
            <label>{move_gettext!(cx, "Layout")}</label>
            <div>
                <ControlButtonSVGPath
                    title=move_gettext!(cx, "Comfortable")
                    svg_path="M19 2a2 2 0 0 1 2 2v2a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h14zm0 4V4H5v2h14zm0 10a2 2 0 0 1 2 2v2a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-2a2 2 0 0 1 2-2h14zm0 4v-2H5v2h14zm0-11a2 2 0 0 1 2 2v2a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-2a2 2 0 0 1 2-2h14zm0 4v-2H5v2h14z"
                    active=move || { layout() == Layout::Comfortable }
                    on:click=move |_| set_layout(Layout::Comfortable, &layout)
                />
                <ControlButtonSVGPath
                    title=move_gettext!(cx, "Compact")
                    svg_path="M2 5.5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v13a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-13zm9 0H4v3h7v-3zm2 0v3h7v-3h-7zm7 5h-7v3h7v-3zm0 5h-7v3h7v-3zm-9 3v-3H4v3h7zm-7-5h7v-3H4v3z"
                    active=move || { layout() == Layout::Compact }
                    on:click=move |_| set_layout(Layout::Compact, &layout)
                />
            </div>
        </div>
    }
}
