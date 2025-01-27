use crate::controls::search::focus_search_bar;
use crate::copy::copy_inner_text_on_click;
use leptos::{ev::MouseEvent, prelude::*};
use leptos_fluent::tr;
use leptos_use::on_click_outside;
use simple_icons_website_svg_defs::SVGDef;
use simple_icons_website_svg_icon::SVGIcon;
use simple_icons_website_url as Url;
use std::str::FromStr;

#[component]
fn ModalHeader(
    /// Title of the modal
    title: Signal<String>,
    /// Indicates whether the title is copyable
    #[prop(optional)]
    title_is_copyable: bool,
    /// Function executed when the close button is clicked
    /// or the user clicks outside the modal
    on_close: Signal<()>,
) -> impl IntoView {
    view! {
        <div>
            <h2
                class:copyable=move || title_is_copyable
                on:click=move |ev: MouseEvent| {
                    if title_is_copyable {
                        copy_inner_text_on_click(ev);
                    }
                }
            >

                {title}
            </h2>
            <button type="button" title=move || tr!("close") on:click=move |_| on_close()>
                <SVGIcon path=&SVGDef::Cross />
            </button>
        </div>
    }
}

#[component]
pub fn Modal(
    children: ChildrenFn,
    /// Title of the modal
    #[prop(optional, into)]
    title: Signal<String>,
    /// Indicates whether the title is copyable
    #[prop(optional)]
    title_is_copyable: bool,
    /// Indicates whether the modal is open or not
    is_open: Signal<bool>,
    /// Function executed when the close button is clicked
    /// or the user clicks outside the modal
    on_close: Signal<()>,
    /// Set the focus on the search bar when the modal is closed
    #[prop(optional)]
    on_close_focus_search_bar: bool,
) -> impl IntoView {
    let modal_ref = NodeRef::new();
    _ = on_click_outside(modal_ref, move |_| {
        if is_open.get_untracked() {
            on_close();
            if on_close_focus_search_bar {
                focus_search_bar();
            }
        }
    });

    let on_close_header = move || {
        on_close();
        if on_close_focus_search_bar {
            focus_search_bar();
        }
    };

    view! {
        <div class=move || {
            let mut cls = "modal-shadow".to_string();
            if !is_open() {
                cls.push_str(" hidden");
            }
            cls
        }>
            <div node_ref=modal_ref class="modal">
                <ModalHeader
                    title
                    title_is_copyable=title_is_copyable
                    on_close=Signal::derive(on_close_header)
                />

                <div>{children()}</div>
            </div>
        </div>
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum ModalOpen {
    Extensions,
    Languages,
    Icon,
}

impl core::fmt::Display for ModalOpen {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ModalOpen::Extensions => write!(f, "extensions"),
            ModalOpen::Languages => write!(f, "languages"),
            ModalOpen::Icon => write!(f, "icon"),
        }
    }
}

impl FromStr for ModalOpen {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "extensions" => Ok(Self::Extensions),
            "languages" => Ok(Self::Languages),
            "icon" => Ok(Self::Icon),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone)]
pub struct ModalOpenSignal(pub RwSignal<Option<ModalOpen>>);

impl ModalOpenSignal {
    pub fn set_none(&self) {
        self.0.update(|mo| *mo = None);
        Url::params::update(&Url::params::Names::Modal, "");
    }

    pub fn set_extensions(&self) {
        self.0.update(|mo| *mo = Some(ModalOpen::Extensions));
        Url::params::update(
            &Url::params::Names::Modal,
            &ModalOpen::Extensions.to_string(),
        );
    }

    pub fn set_languages(&self) {
        self.0.update(|mo| *mo = Some(ModalOpen::Languages));
        Url::params::update(
            &Url::params::Names::Modal,
            &ModalOpen::Languages.to_string(),
        );
    }

    pub fn set_icon(&self) {
        self.0.update(|mo| *mo = Some(ModalOpen::Icon));
        Url::params::update(
            &Url::params::Names::Modal,
            &ModalOpen::Icon.to_string(),
        );
    }
}

pub fn provide_modal_open_context() {
    provide_context(ModalOpenSignal(RwSignal::new(
        Url::params::get(&Url::params::Names::Modal)
            .and_then(|value| value.parse().ok()),
    )));
}
