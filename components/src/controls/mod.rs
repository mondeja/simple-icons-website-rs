//! App controls

mod button;
pub mod color_scheme;
pub mod download;
pub mod order;
pub mod search;

use button::*;
use color_scheme::*;
use download::*;
use i18n::{gettext, move_gettext};
use leptos::*;
use order::*;
use search::*;

/// State of the controls panel
#[derive(Copy, Clone, Default)]
pub struct ControlsState {
    /// Indicate wheter the buttons group or the search bar is open
    /// (only used on tablet and mobile screens)
    pub buttons_group_open: bool,
}

#[derive(Copy, Clone)]
struct ControlsStateSignal(RwSignal<ControlsState>);

#[component]
pub fn Controls(cx: Scope) -> impl IntoView {
    let controls_state = create_rw_signal(cx, ControlsState::default());
    provide_context(cx, ControlsStateSignal(controls_state));

    view! { cx,
        <menu>
            <div class=move||{
                let mut class = "controls-group flex-grow sm:flex-grow-0 mr-4".to_string();
                if controls_state().buttons_group_open {
                    class.push_str(" hidden lg:flex");
                }
                class
            }>
                <SearchControl/>
            </div>
            <div class=move|| {
                let mut class = "controls-group".to_string();
                if !controls_state().buttons_group_open {
                    class.push_str(" hidden lg:flex");
                } else {
                    class.push_str(" flex flex-grow");
                }
                class
            }>
                <OrderControl/>
                <ColorSchemeControl/>
                <DownloadFileTypeControl/>
                <LayoutControl/>
            </div>
            <div class=move || {
                let mut class = "controls-group relative lg:hidden".to_string();
                if !controls_state().buttons_group_open {
                    class.push_str(" flex-grow");
                }
                class
            }>
                <ControlsBurgerButton/>
            </div>
        </menu>
    }
}

#[component]
pub fn LayoutControl(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="control">
            <label>{move_gettext!(cx, "Layout")}</label>
            <div>
                <ControlButtonSVGPath
                    title=move_gettext!(cx, "Comfortable")
                    svg_path="M19 2a2 2 0 0 1 2 2v2a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h14zm0 4V4H5v2h14zm0 10a2 2 0 0 1 2 2v2a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-2a2 2 0 0 1 2-2h14zm0 4v-2H5v2h14zm0-11a2 2 0 0 1 2 2v2a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-2a2 2 0 0 1 2-2h14zm0 4v-2H5v2h14z"
                    active=||{false}
                />
                <ControlButtonSVGPath
                    title=move_gettext!(cx, "Compact")
                    svg_path="M2 5.5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v13a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-13zm9 0H4v3h7v-3zm2 0v3h7v-3h-7zm7 5h-7v3h7v-3zm0 5h-7v3h7v-3zm-9 3v-3H4v3h7zm-7-5h7v-3H4v3z"
                    active=||{false}
                />
            </div>
        </div>
    }
}

#[component]
pub fn ControlsBurgerButton(cx: Scope) -> impl IntoView {
    let controls_state = use_context::<ControlsStateSignal>(cx).unwrap().0;

    view! { cx,
        <button
            class="absolute bottom-0 right-0 control-button rounded-[3px] lg:hidden"
            title=move||{
                if controls_state().buttons_group_open {
                    gettext!(cx, "Open search bar")
                } else {
                    gettext!(cx, "Open controls")
                }
            }
            on:click=move |_| {
                controls_state.update(|mut state| {
                    state.buttons_group_open = !state.buttons_group_open;
                });
            }
        >
            <svg role="img" viewBox="0 0 24 24">
                <path d=move||{
                    if controls_state().buttons_group_open {
                        "M15.5 14h-.79l-.28-.27A6.471 6.471 0 0 0 16 9.5A6.5 6.5 0 1 0 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5S14 7.01 14 9.5S11.99 14 9.5 14z"
                    } else {
                        "M3.429 0A3.442 3.442 0 0 0 0 3.429a3.439 3.439 0 0 0 3.429 3.428c1.585 0 2.931-1.097 3.317-2.571h16.397A.857.857 0 0 0 24 3.429a.857.857 0 0 0-.857-.858H6.749A3.447 3.447 0 0 0 3.43 0Zm0 1.714c.956 0 1.714.76 1.714 1.715a1.7 1.7 0 0 1-1.714 1.714 1.699 1.699 0 0 1-1.715-1.714c0-.955.756-1.715 1.715-1.715zM12 8.571a3.446 3.446 0 0 0-3.319 2.572H.857A.857.857 0 0 0 0 12a.857.857 0 0 0 .857.857h7.828A3.446 3.446 0 0 0 12 15.43c1.586 0 2.931-1.098 3.317-2.572h7.826A.857.857 0 0 0 24 12a.857.857 0 0 0-.857-.857H15.32A3.447 3.447 0 0 0 12 8.57Zm0 1.715c.957 0 1.714.76 1.714 1.714A1.7 1.7 0 0 1 12 13.714 1.699 1.699 0 0 1 10.286 12c0-.955.756-1.714 1.714-1.714zm8.571 6.857a3.446 3.446 0 0 0-3.318 2.571H.857a.857.857 0 0 0-.857.857.857.857 0 0 0 .857.858h16.399A3.446 3.446 0 0 0 20.571 24 3.44 3.44 0 0 0 24 20.571a3.443 3.443 0 0 0-3.429-3.428zm0 1.714c.957 0 1.715.76 1.715 1.714a1.7 1.7 0 0 1-1.715 1.715 1.699 1.699 0 0 1-1.714-1.715c0-.954.756-1.714 1.714-1.714z"
                    }
                }/>
            </svg>
        </button>
    }
}
