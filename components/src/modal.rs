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
        <div class="flex flex-row w-full px-5 py-3"
            style="border-bottom: 1px solid var(--divider-color);"
        >
            <h2 class="flex-grow">{title}</h2>
            <button
                class=concat!(
                    "w-[1.3rem] h-[1.3rem] self-center",
                    " fill-custom-text-default-color",
                    " hover:opacity-80",
                )
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
        <div class="w-full p-4 overflow-y-auto">
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
        <div class=move || {
            let mut class = concat!(
                "fixed top-0 left-0 w-full h-full",
                " z-50 bg-[rgba(0,0,0,.7)]",
            ).to_string();
            if !is_open() {
                class.push(' ');
                class.push_str("hidden");
            }
            class
        }
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
            <div class=concat!(
                "flex flex-col self-center justify-center m-auto mt-0 md:mt-10 rounded",
                " max-w-full md:max-w-[80%] lg:max-w-[50%] max-h-[80%]",
                " bg-custom-background-color shadow-[0_0_3px_0_var(--shadows-color)]"
            )>
                <ModalHeader title=title on_close=on_close/>
                <ModalBody>
                    {children(cx)}
                </ModalBody>
            </div>
        </ModalShadow>
    }
}
