use crate::svg_defs::SVGDefs;
use i18n::move_gettext;
use leptos::{ev::MouseEvent, *};

pub trait TitleFn = Fn() -> String + 'static;
pub trait IsOpenFn = Fn() -> bool + 'static;
pub trait OnCloseFn = FnMut(MouseEvent) + 'static + Copy;

#[component]
fn ModalHeader<T, C>(
    cx: Scope,
    /// Title of the modal
    title: T,
    /// Function executed when the close button is clicked
    /// or the user clicks outside the modal
    on_close: C,
) -> impl IntoView
where
    T: TitleFn + 'static,
    C: OnCloseFn + 'static,
{
    view! { cx,
        <div>
            <h2>{title}</h2>
            <button
                type="button"
                title=move_gettext!(cx, "Close")
                on:click=on_close
            >
                <svg role="img" viewBox="0 0 24 24">
                    <use_ href=format!("#{}", SVGDefs::CrossPath.id()) />
                </svg>
            </button>
        </div>
    }
}

#[component]
fn ModalBody(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <div>
            {children(cx)}
        </div>
    }
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
    O: IsOpenFn + 'static,
    C: OnCloseFn + 'static,
{
    view! { cx,
        <div
            class="modal-shadow"
            class:hidden=move || !is_open()
            on:click=on_close
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
    /// Indicates whether the modal is open or not
    is_open: O,
    /// Function executed when the close button is clicked
    /// or the user clicks outside the modal
    on_close: C,
) -> impl IntoView
where
    O: IsOpenFn + 'static,
    T: TitleFn + 'static,
    C: OnCloseFn + 'static,
{
    view! { cx,
        <ModalShadow is_open=is_open on_close=on_close>
            <div class="modal">
                <ModalHeader title=title on_close=on_close/>
                <ModalBody>
                    {children(cx)}
                </ModalBody>
            </div>
        </ModalShadow>
    }
}
