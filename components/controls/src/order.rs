use super::button::ControlButtonIcon;
use super::search::{
    fire_on_search_event, get_search_value_from_localstorage, SearchValueSignal,
};
use leptos::prelude::*;
use leptos_fluent::move_tr;
use simple_icons_website_controls_layout_types::{Layout, LayoutSignal};
use simple_icons_website_controls_order_types::{
    sort_icons, OrderMode, OrderModeVariant,
};
use simple_icons_website_grid_types::{
    IconsGrid, IconsGridSignal, IconsIndexSignal,
};
use simple_icons_website_storage::LocalStorage;
use simple_icons_website_types::SimpleIcon;
use std::str::FromStr;

pub fn provide_order_mode_context(initial_search_value: &str) -> OrderMode {
    let initial_order_mode =
        get_order_mode_from_localstorage_and_search_value(initial_search_value);
    provide_context(OrderModeSignal(RwSignal::new(initial_order_mode)));
    initial_order_mode
}

#[derive(Copy, Clone)]
pub struct OrderModeSignal(pub RwSignal<OrderMode>);

fn get_order_mode_from_localstorage() -> Option<OrderMode> {
    LocalStorage::get(LocalStorage::Keys::OrderMode)
        .as_ref()
        .and_then(|value| OrderMode::from_str(value).ok())
}

fn get_order_mode_from_localstorage_and_search_value(
    search_value: &str,
) -> OrderMode {
    let maybe_order_mode = get_order_mode_from_localstorage();
    let mut order_mode = maybe_order_mode.unwrap_or_default();
    if !search_value.is_empty() {
        order_mode.current = OrderModeVariant::SearchMatch;
    }
    order_mode
}

fn set_order_mode_on_localstorage(order_mode: &OrderModeVariant) {
    LocalStorage::set(LocalStorage::Keys::OrderMode, &order_mode.to_string());
}

pub fn set_order_mode(
    order_mode: &OrderModeVariant,
    order_mode_signal: &RwSignal<OrderMode>,
    icons_grid_signal: &RwSignal<IconsGrid>,
    layout_signal: Option<&Layout>,
    update_grid: bool,
    icons: Vec<&'static SimpleIcon>,
) {
    order_mode_signal.update(move |state| {
        state.current = *order_mode;
        if *order_mode != OrderModeVariant::SearchMatch {
            state.favorite = *order_mode;
            set_order_mode_on_localstorage(order_mode);
        }
    });
    if update_grid {
        match order_mode {
            &OrderModeVariant::SearchMatch => {
                // Fire a search event to update the grid
                fire_on_search_event();
            }
            _ => {
                icons_grid_signal.update(|grid| {
                    let search_value = get_search_value_from_localstorage();
                    if search_value.is_some() {
                        // If we are searching, just update the order of the current
                        // icons
                        sort_icons(order_mode, &mut grid.icons);
                        sort_icons(order_mode, &mut grid.loaded_icons);
                    } else {
                        // If not searching, load the new icons in order
                        grid.loaded_icons = Vec::with_capacity(icons.len());
                        grid.icons = icons;
                        sort_icons(order_mode, &mut grid.icons);
                        if let Some(layout_signal) = layout_signal {
                            grid.load_next_icons(layout_signal);
                        }
                    }
                });
            }
        }
    }
}

#[component]
pub fn OrderControl() -> impl IntoView {
    let order_mode = expect_context::<OrderModeSignal>().0;
    let icons_grid = expect_context::<IconsGridSignal>().0;
    let search_signal = expect_context::<SearchValueSignal>().0;
    let layout_signal = expect_context::<LayoutSignal>().0;
    let icons = StoredValue::new(expect_context::<IconsIndexSignal>().0);

    Effect::new(move |_| match order_mode.get_untracked().current {
        OrderModeVariant::Random => set_order_mode(
            &OrderModeVariant::Random,
            &order_mode,
            &icons_grid,
            Some(&layout_signal()),
            true,
            icons.read_value().to_vec(),
        ),
        OrderModeVariant::Color => set_order_mode(
            &OrderModeVariant::Color,
            &order_mode,
            &icons_grid,
            Some(&layout_signal()),
            true,
            icons.read_value().to_vec(),
        ),
        _ => {}
    });

    view! {
        <div class="control">
            <label>{move_tr!("order")}</label>
            <div class="flex flex-row">
                <ControlButtonIcon
                    title=move_tr!("sort-alphabetically")
                    icon="M10.096 18.857H7.525V.429A.414.414 0 0 0 7.096 0H4.525a.414.414 0 0 0-.429.429v18.429H1.525c-.196 0-.331.089-.402.268-.072.17-.04.326.094.469l4.286 4.286c.098.079.2.119.308.119.116 0 .219-.04.308-.12l4.272-4.272a.506.506 0 0 0 .134-.321.414.414 0 0 0-.429-.43zm10.006 3.617H16.78c-.188 0-.322.009-.402.026l-.188.026V22.5l.148-.147c.133-.16.227-.276.281-.348l4.941-7.099v-1.191h-7.594v3.066h1.607v-1.54h3.107c.16 0 .295-.014.4-.04a.856.856 0 0 0 .102-.007c.039-.004.068-.007.086-.007v.04l-.146.121c-.08.08-.176.2-.281.361L13.9 22.795V24h7.82v-3.12h-1.619v1.594h.001zm1.875-13.608L18.895 0h-2.168l-3.082 8.866h-.936v1.419h3.842V8.866h-1.004l.631-1.929h3.254l.629 1.929h-1.004v1.419h3.857V8.866h-.937zm-5.358-3.402.977-2.92c.037-.107.07-.236.102-.388s.047-.232.047-.241l.039-.268h.055c0 .036.008.125.025.268l.162.629.963 2.92h-2.37z"
                    active=Signal::derive(move || {
                        order_mode().current == OrderModeVariant::Alphabetic
                    })
                    class:hidden=Signal::derive(move || {
                        order_mode().current == OrderModeVariant::AlphabeticReverse
                    })
                    on:click=move |_| {
                        let new_order_mode = match order_mode().current {
                            OrderModeVariant::Alphabetic => OrderModeVariant::AlphabeticReverse,
                            _ => OrderModeVariant::Alphabetic,
                        };
                        set_order_mode(
                            &new_order_mode,
                            &order_mode,
                            &icons_grid,
                            Some(&layout_signal()),
                            true,
                            icons.read_value().to_vec(),
                        )
                    }
                />
                <ControlButtonIcon
                    title=move_tr!("sort-alphabetically")
                    icon="M1.515 5.143h2.571V23.57a.414.414 0 0 0 .43.429h2.57a.414.414 0 0 0 .43-.43V5.143h2.57c.196 0 .331-.09.402-.268.072-.17.04-.326-.094-.469L6.108.119A.484.484 0 0 0 5.8 0a.449.449 0 0 0-.308.12L1.22 4.392a.506.506 0 0 0-.134.32.414.414 0 0 0 .43.43Zm18.587 17.331H16.78c-.188 0-.322.008-.402.026l-.188.027V22.5l.148-.147c.133-.16.227-.275.281-.348l4.94-7.099v-1.19h-7.593v3.065h1.607v-1.54h3.107c.16 0 .295-.015.4-.04a.856.856 0 0 0 .102-.007c.039-.004.068-.008.086-.008v.04l-.146.121c-.08.08-.176.2-.28.361L13.9 22.794v1.205h7.82v-3.12H20.1v1.594Zm1.875-13.608L18.895 0h-2.168l-3.082 8.866h-.936v1.42h3.842v-1.42h-1.004l.631-1.929h3.254l.63 1.93h-1.005v1.418h3.857V8.866ZM16.62 5.464l.977-2.92c.038-.107.07-.236.102-.388.03-.152.047-.232.047-.24l.039-.269h.055c0 .036.007.125.026.268l.162.63.963 2.92z"
                    active=Signal::derive(move || {
                        order_mode().current == OrderModeVariant::AlphabeticReverse
                    })
                    class:hidden=Signal::derive(move || {
                        order_mode().current != OrderModeVariant::AlphabeticReverse
                    })
                    on:click=move |_| {
                        let new_order_mode = match order_mode().current {
                            OrderModeVariant::AlphabeticReverse => OrderModeVariant::Alphabetic,
                            _ => OrderModeVariant::AlphabeticReverse,
                        };
                        set_order_mode(
                            &new_order_mode,
                            &order_mode,
                            &icons_grid,
                            Some(&layout_signal()),
                            true,
                            icons.read_value().to_vec(),
                        )
                    }
                />

                <ControlButtonIcon
                    title=move_tr!("sort-by-color")
                    icon="M3.44 0a.42.42 0 0 0-.31.121.42.42 0 0 0-.12.309v18.427H.44a.4.4 0 0 0-.403.268c-.07.17-.04.326.094.469l4.287 4.287c.098.08.199.119.307.119a.449.449 0 0 0 .308-.12l4.272-4.273a.502.502 0 0 0 .134-.318.418.418 0 0 0-.43-.432H6.44V.43A.415.415 0 0 0 6.01 0H3.44zm14.738 4.178c-2.082.008-4.25.926-5.875 2.69a7.722 7.722 0 0 0 .445 10.91 7.722 7.722 0 0 0 10.912-.444 1.287 1.287 0 0 0-.074-1.818 1.26 1.26 0 0 0-.857-.336 1.333 1.333 0 0 1-.854-.342 1.287 1.287 0 0 1-.074-1.818l1.03-1.118a4.29 4.29 0 0 0-.247-6.06c-1.22-1.125-2.787-1.67-4.406-1.664zm-2.063 1.869a1.287 1.287 0 0 1 .723.332 1.287 1.287 0 0 1 .074 1.818 1.287 1.287 0 0 1-1.818.074 1.287 1.287 0 0 1-.074-1.818 1.287 1.287 0 0 1 1.095-.406zm4.268.433a1.287 1.287 0 0 1 .722.332 1.287 1.287 0 0 1 .075 1.819 1.287 1.287 0 0 1-1.819.074 1.287 1.287 0 0 1-.074-1.818 1.287 1.287 0 0 1 1.096-.407zm-7.176 2.721a1.287 1.287 0 0 1 .723.332 1.287 1.287 0 0 1 .074 1.819 1.287 1.287 0 0 1-1.818.074 1.287 1.287 0 0 1-.075-1.819 1.287 1.287 0 0 1 1.096-.406zm.78 4.219a1.287 1.287 0 0 1 .722.332 1.287 1.287 0 0 1 .074 1.818 1.287 1.287 0 0 1-1.818.075 1.287 1.287 0 0 1-.074-1.819 1.287 1.287 0 0 1 1.095-.406z"
                    active=Signal::derive(move || order_mode().current == OrderModeVariant::Color)
                    class:hidden=Signal::derive(move || {
                        order_mode().current == OrderModeVariant::ColorReverse
                    })
                    on:click=move |_| {
                        let new_order_mode = match order_mode().current {
                            OrderModeVariant::Color => OrderModeVariant::ColorReverse,
                            _ => OrderModeVariant::Color,
                        };
                        set_order_mode(
                            &new_order_mode,
                            &order_mode,
                            &icons_grid,
                            Some(&layout_signal()),
                            true,
                            icons.read_value().to_vec(),
                        )
                    }
                />

                <ControlButtonIcon
                    title=move_tr!("sort-by-color")
                    icon="M6 24a.42.42 0 0 0 .31-.121.42.42 0 0 0 .12-.31V5.144H9a.4.4 0 0 0 .403-.268c.07-.17.04-.326-.094-.47L5.022.12a.476.476 0 0 0-.307-.12.449.449 0 0 0-.308.12L.135 4.394A.502.502 0 0 0 0 4.71a.418.418 0 0 0 .43.432H3V23.57a.415.415 0 0 0 .43.43zM18.178 4.178c-2.082.008-4.25.926-5.875 2.69a7.722 7.722 0 0 0 .445 10.91 7.722 7.722 0 0 0 10.912-.444 1.287 1.287 0 0 0-.074-1.818 1.26 1.26 0 0 0-.857-.336 1.333 1.333 0 0 1-.854-.342 1.287 1.287 0 0 1-.074-1.818l1.03-1.118a4.29 4.29 0 0 0-.247-6.06c-1.22-1.125-2.787-1.67-4.406-1.664Zm-2.063 1.869a1.287 1.287 0 0 1 .723.332 1.287 1.287 0 0 1 .074 1.818 1.287 1.287 0 0 1-1.818.074 1.287 1.287 0 0 1-.074-1.818 1.287 1.287 0 0 1 1.095-.406Zm4.268.433a1.287 1.287 0 0 1 .722.332 1.287 1.287 0 0 1 .075 1.819 1.287 1.287 0 0 1-1.819.074 1.287 1.287 0 0 1-.074-1.818 1.287 1.287 0 0 1 1.096-.407Zm-7.176 2.721a1.287 1.287 0 0 1 .723.332 1.287 1.287 0 0 1 .074 1.819 1.287 1.287 0 0 1-1.818.074 1.287 1.287 0 0 1-.075-1.819 1.287 1.287 0 0 1 1.096-.406zm.78 4.219a1.287 1.287 0 0 1 .722.332 1.287 1.287 0 0 1 .074 1.818 1.287 1.287 0 0 1-1.818.075 1.287 1.287 0 0 1-.074-1.819 1.287 1.287 0 0 1 1.095-.406z"
                    active=Signal::derive(move || {
                        order_mode().current == OrderModeVariant::ColorReverse
                    })
                    class:hidden=Signal::derive(move || {
                        order_mode().current != OrderModeVariant::ColorReverse
                    })
                    on:click=move |_| {
                        let new_order_mode = match order_mode().current {
                            OrderModeVariant::ColorReverse => OrderModeVariant::Color,
                            _ => OrderModeVariant::ColorReverse,
                        };
                        set_order_mode(
                            &new_order_mode,
                            &order_mode,
                            &icons_grid,
                            Some(&layout_signal()),
                            true,
                            icons.read_value().to_vec(),
                        )
                    }
                />

                <ControlButtonIcon
                    title=move_tr!("sort-randomly")
                    icon="M10.59 9.17 5.41 4 4 5.41l5.17 5.17 1.42-1.41zM14.5 4l2.04 2.04L4 18.59 5.41 20 17.96 7.46 20 9.5V4h-5.5zm.33 9.41-1.41 1.41 3.13 3.13L14.5 20H20v-5.5l-2.04 2.04-3.13-3.13z"
                    active=Signal::derive(move || order_mode().current == OrderModeVariant::Random)
                    on:click=move |_| set_order_mode(
                        &OrderModeVariant::Random,
                        &order_mode,
                        &icons_grid,
                        Some(&layout_signal()),
                        true,
                        icons.read_value().to_vec(),
                    )
                />

                {move || match search_signal().is_empty() {
                    #[allow(clippy::unit_arg, clippy::unused_unit)]
                    true => view!().into_any(),
                    false => {
                        view! {
                            <ControlButtonIcon
                                title=move_tr!("sort-by-search-match")
                                icon="M1.226 7.709q-.521 0-.874-.353T0 6.483q0-.521.353-.874.353-.354.873-.352h3.678q.521 0 .874.353.354.353.352.873 0 .52-.353.874-.353.353-.873.352zm0 6.13q-.521 0-.874-.353T0 12.613q0-.521.353-.874t.873-.352h3.678q.521 0 .874.353.354.353.352.873 0 .521-.353.874t-.873.352zm20.72 5.272-3.862-3.862q-.735.521-1.61.782-.874.262-1.761.26-2.545 0-4.338-1.794-1.794-1.793-1.793-4.336 0-2.544 1.794-4.338t4.337-1.792q2.544 0 4.337 1.793 1.794 1.794 1.793 4.337 0 .889-.261 1.763-.262.874-.781 1.609l3.862 3.862q.337.337.337.858 0 .521-.337.858-.338.338-.859.338-.52 0-.858-.338zm-7.233-5.272q1.532 0 2.605-1.073t1.073-2.605q0-1.533-1.073-2.605-1.073-1.073-2.605-1.073-1.533 0-2.606 1.073-1.073 1.072-1.073 2.605 0 1.532 1.073 2.605t2.606 1.073zM1.226 19.97q-.521 0-.874-.353T0 18.743q0-.52.353-.874.353-.353.873-.352h9.809q.52 0 .874.354.353.353.351.872 0 .522-.353.875t-.873.351z"
                                active=Signal::derive(move || {
                                    order_mode().current == OrderModeVariant::SearchMatch
                                })

                                on:click=move |_| set_order_mode(
                                    &OrderModeVariant::SearchMatch,
                                    &order_mode,
                                    &icons_grid,
                                    Some(&layout_signal()),
                                    true,
                                    icons.read_value().to_vec(),
                                )
                            />
                        }
                            .into_any()
                    }
                }}

            </div>
        </div>
    }
}
