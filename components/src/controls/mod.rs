//! App controls

mod button;
pub mod color_scheme;
pub mod download;
pub mod order;
pub mod search;

use button::*;
use color_scheme::*;
use download::*;
use i18n::move_gettext;
use leptos::*;
use order::*;
use search::*;

#[component]
pub fn Controls(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex flex-row space-x-4">
            <SearchControl/>
            <OrderControl/>
            <ColorSchemeControl/>
            <DownloadFileTypeControl/>
            <LayoutControl/>
        </div>
    }
}

#[component]
pub fn LayoutControl(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex flex-col">
            <label>{move_gettext!(cx, "Layout")}</label>
            <div class="flex flex-row">
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
