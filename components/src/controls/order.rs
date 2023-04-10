use crate::controls::button::*;
use crate::storage::LocalStorage;
use crate::DisplayedIconsSignal;
use i18n::move_gettext;
use leptos::*;
use simple_icons::StaticSimpleIcon;
use std::fmt;

#[derive(Default, Copy, Clone, PartialEq)]
pub enum OrderMode {
    #[default]
    Alphabetic,
    Color,
    SearchMatch,
}

impl OrderMode {
    pub fn sort_icons(&self, icons: &mut Vec<StaticSimpleIcon>) {
        match self {
            OrderMode::Alphabetic => {
                icons.sort_by(|a, b| a.order_alpha.cmp(&b.order_alpha));
            }
            _ => {
                icons.sort_by(|a, b| a.order_color.cmp(&b.order_color));
            }
        }
    }
}

impl From<&str> for OrderMode {
    fn from(order_mode: &str) -> Self {
        match order_mode {
            "alpha" => Self::Alphabetic,
            "color" => Self::Color,
            _ => Self::SearchMatch,
        }
    }
}

impl fmt::Display for OrderMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Alphabetic => write!(f, "alpha"),
            Self::Color => write!(f, "color"),
            Self::SearchMatch => write!(f, "search"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct OrderModeSignal(pub RwSignal<OrderMode>);

pub fn initial_order_mode_from_localstorage() -> OrderMode {
    let window = web_sys::window().unwrap();
    let local_storage = window.local_storage().unwrap().unwrap();

    match local_storage.get_item(LocalStorage::Keys::OrderMode.as_str()) {
        Ok(Some(order_mode)) => OrderMode::from(order_mode.as_str()),
        _ => OrderMode::default(),
    }
}

fn set_order_mode_on_localstorage(order_mode: &OrderMode) {
    let window = web_sys::window().unwrap();
    let local_storage = window.local_storage().unwrap().unwrap();
    local_storage
        .set_item(
            LocalStorage::Keys::OrderMode.as_str(),
            &order_mode.to_string(),
        )
        .unwrap();
}

fn set_order_mode(
    order_mode: OrderMode,
    displayed_icons_signal: &RwSignal<Vec<StaticSimpleIcon>>,
) {
    // Sort icons array
    let mut displayed_icons = displayed_icons_signal();
    order_mode.sort_icons(&mut displayed_icons);

    // Update displayed icons signal
    displayed_icons_signal.update(move |state| {
        *state = displayed_icons;
    });

    // Update order mode
    set_order_mode_on_localstorage(&order_mode);
}

#[component]
pub fn OrderControl(cx: Scope) -> impl IntoView {
    let order_mode = use_context::<OrderModeSignal>(cx).unwrap().0;
    let displayed_icons = use_context::<DisplayedIconsSignal>(cx).unwrap().0;

    view! { cx,
        <div class="control">
            <label>{move_gettext!(cx, "Order")}</label>
            <div class="flex flex-row">
                <ControlButtonSVGPath
                    title=move_gettext!(cx, "Sort alphabetically")
                    svg_path="M10.096 18.857H7.525V.429A.414.414 0 0 0 7.096 0H4.525a.414.414 0 0 0-.429.429v18.429H1.525c-.196 0-.331.089-.402.268-.072.17-.04.326.094.469l4.286 4.286c.098.079.2.119.308.119.116 0 .219-.04.308-.12l4.272-4.272a.506.506 0 0 0 .134-.321.414.414 0 0 0-.429-.43zm10.006 3.617H16.78c-.188 0-.322.009-.402.026l-.188.026V22.5l.148-.147c.133-.16.227-.276.281-.348l4.941-7.099v-1.191h-7.594v3.066h1.607v-1.54h3.107c.16 0 .295-.014.4-.04a.856.856 0 0 0 .102-.007c.039-.004.068-.007.086-.007v.04l-.146.121c-.08.08-.176.2-.281.361L13.9 22.795V24h7.82v-3.12h-1.619v1.594h.001zm1.875-13.608L18.895 0h-2.168l-3.082 8.866h-.936v1.419h3.842V8.866h-1.004l.631-1.929h3.254l.629 1.929h-1.004v1.419h3.857V8.866h-.937zm-5.358-3.402.977-2.92c.037-.107.07-.236.102-.388s.047-.232.047-.241l.039-.268h.055c0 .036.008.125.025.268l.162.629.963 2.92h-2.37z"
                    active=move || {order_mode() == OrderMode::Alphabetic}
                    on:click=move |_| {
                        order_mode.update(move|state| {
                            *state = OrderMode::Alphabetic;
                            set_order_mode(OrderMode::Alphabetic, &displayed_icons);
                        });
                    }
                />
                <ControlButtonSVGPath
                    title=move_gettext!(cx, "Sort by color")
                    svg_path="M3.44 0a.42.42 0 0 0-.31.121.42.42 0 0 0-.12.309v18.427H.44a.4.4 0 0 0-.403.268c-.07.17-.04.326.094.469l4.287 4.287c.098.08.199.119.307.119a.449.449 0 0 0 .308-.12l4.272-4.273a.502.502 0 0 0 .134-.318.418.418 0 0 0-.43-.432H6.44V.43A.415.415 0 0 0 6.01 0H3.44zm14.738 4.178c-2.082.008-4.25.926-5.875 2.69a7.722 7.722 0 0 0 .445 10.91 7.722 7.722 0 0 0 10.912-.444 1.287 1.287 0 0 0-.074-1.818 1.26 1.26 0 0 0-.857-.336 1.333 1.333 0 0 1-.854-.342 1.287 1.287 0 0 1-.074-1.818l1.03-1.118a4.29 4.29 0 0 0-.247-6.06c-1.22-1.125-2.787-1.67-4.406-1.664zm-2.063 1.869a1.287 1.287 0 0 1 .723.332 1.287 1.287 0 0 1 .074 1.818 1.287 1.287 0 0 1-1.818.074 1.287 1.287 0 0 1-.074-1.818 1.287 1.287 0 0 1 1.095-.406zm4.268.433a1.287 1.287 0 0 1 .722.332 1.287 1.287 0 0 1 .075 1.819 1.287 1.287 0 0 1-1.819.074 1.287 1.287 0 0 1-.074-1.818 1.287 1.287 0 0 1 1.096-.407zm-7.176 2.721a1.287 1.287 0 0 1 .723.332 1.287 1.287 0 0 1 .074 1.819 1.287 1.287 0 0 1-1.818.074 1.287 1.287 0 0 1-.075-1.819 1.287 1.287 0 0 1 1.096-.406zm.78 4.219a1.287 1.287 0 0 1 .722.332 1.287 1.287 0 0 1 .074 1.818 1.287 1.287 0 0 1-1.818.075 1.287 1.287 0 0 1-.074-1.819 1.287 1.287 0 0 1 1.095-.406z"
                    active=move || {order_mode() == OrderMode::Color}
                    on:click=move |_| {
                        order_mode.update(move|state| {
                            *state = OrderMode::Color;
                            set_order_mode(OrderMode::Color, &displayed_icons);
                        });
                    }
                />
            </div>
        </div>
    }
}
