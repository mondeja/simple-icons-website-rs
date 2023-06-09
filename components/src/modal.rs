use crate::copy::copy_inner_text_on_click;
use crate::svg_defs::SVGDefs;
use i18n::move_gettext;
use leptos::{ev::MouseEvent, *};

pub trait TitleFn = Fn() -> String + 'static;
pub trait IsOpenFn = Fn() -> bool + 'static;
pub trait OnCloseFn = Fn(MouseEvent) + 'static + Copy;

#[component]
fn ModalHeader<T, C>(
    cx: Scope,
    /// Title of the modal
    title: T,
    /// Indicates whether the title is copyable
    #[prop(optional)]
    title_is_copyable: bool,
    /// Function executed when the close button is clicked
    /// or the user clicks outside the modal
    on_close: C,
) -> impl IntoView
where
    T: TitleFn,
    C: OnCloseFn,
{
    view! { cx,
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
            <button type="button" title=move_gettext!(cx, "Close") on:click=on_close>
                <svg role="img" viewBox="0 0 24 24">
                    <use_ href=format!("#{}", SVGDefs::CrossPath.id())></use_>
                </svg>
            </button>
        </div>
    }
}

#[component]
fn ModalBody(cx: Scope, children: Children) -> impl IntoView {
    view! { cx, <div>{children(cx)}</div> }
}

#[component]
fn ModalShadow<O, C>(
    cx: Scope,
    children: Children,
    /// Indicates whether the modal is open or not
    is_open: O,
    /// Function executed when the user clicks in the shadow of the modal
    on_close: C,
) -> impl IntoView
where
    O: IsOpenFn,
    C: OnCloseFn,
{
    let class: &'static str = "modal-shadow";

    view! { cx,
        <div
            class=class
            class:hidden=move || !is_open()
            on:click=move |ev: MouseEvent| {
                let target = event_target::<web_sys::HtmlElement>(&ev);
                if target.class_list().contains(class) {
                    on_close(ev);
                }
            }
        >
            {children(cx)}
        </div>
    }
}

#[component]
pub fn Modal<T, O, C>(
    cx: Scope,
    children: Children,
    /// Title of the modal
    title: T,
    /// Indicates whether the title is copyable
    #[prop(optional)]
    title_is_copyable: bool,
    /// Indicates whether the modal is open or not
    is_open: O,
    /// Function executed when the close button is clicked
    /// or the user clicks outside the modal
    on_close: C,
) -> impl IntoView
where
    O: IsOpenFn,
    T: TitleFn,
    C: OnCloseFn,
{
    view! { cx,
        <ModalShadow is_open=is_open on_close=on_close>
            <div class="modal">
                <ModalHeader title=title title_is_copyable=title_is_copyable on_close=on_close/>
                <ModalBody>{children(cx)}</ModalBody>
            </div>
        </ModalShadow>
    }
}
