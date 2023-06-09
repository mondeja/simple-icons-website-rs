use crate::controls::button::ControlButtonSVGPath;
use crate::storage::LocalStorage;
use crate::Url;
use i18n::move_gettext;
use leptos::{window, *};
use std::fmt;

#[derive(Default, Copy, Clone, PartialEq)]
pub enum Layout {
    #[default]
    Comfortable,
    Compact,
}

impl Layout {
    fn from_str(layout: &str) -> Option<Self> {
        match layout {
            "comfortable" => Some(Self::Comfortable),
            "compact" => Some(Self::Compact),
            _ => None,
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

pub fn provide_layout_context(cx: Scope) {
    provide_context(cx, LayoutSignal(create_rw_signal(cx, initial_layout())));
}

fn initial_layout() -> Layout {
    match layout_from_url() {
        Some(layout) => {
            set_layout_on_localstorage(&layout);
            layout
        }
        None => match layout_from_localstorage() {
            Some(layout) => layout,
            None => Layout::default(),
        },
    }
}

fn layout_from_url() -> Option<Layout> {
    match Url::params::get(&Url::params::Names::Layout) {
        Some(layout) => Layout::from_str(layout.as_str()),
        None => None,
    }
}

fn layout_from_localstorage() -> Option<Layout> {
    match window()
        .local_storage()
        .unwrap()
        .unwrap()
        .get_item(LocalStorage::Keys::Layout.as_str())
    {
        Ok(Some(layout)) => Layout::from_str(layout.as_str()),
        _ => None,
    }
}

fn set_layout_on_localstorage(layout: &Layout) {
    window()
        .local_storage()
        .unwrap()
        .unwrap()
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
