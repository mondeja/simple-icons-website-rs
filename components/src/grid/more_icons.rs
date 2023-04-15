use crate::grid::IconsGridSignal;
use i18n::move_gettext;
use leptos::*;

/// Handler which controls if more grid items should be loaded
#[derive(Copy, Clone)]
pub struct GridIconsLoader {
    pub load_more_icons: bool,
}

impl GridIconsLoader {
    pub fn new() -> Self {
        Self {
            load_more_icons: true,
        }
    }
}

/// Signal to control the grid items loader
#[derive(Copy, Clone)]
pub struct GridIconsLoaderSignal(pub RwSignal<GridIconsLoader>);

/// Button lo keep loading icons
///
/// this button is displayed when the user scrolls to the bottom of the grid
#[component]
pub fn LoadMoreIconsButton(cx: Scope) -> impl IntoView {
    let icons_grid_signal = use_context::<IconsGridSignal>(cx).unwrap().0;
    let grid_icons_loader = use_context::<GridIconsLoaderSignal>(cx).unwrap().0;

    view! { cx,
        <div
            class="load-more-icons"
            class:hidden=move || {
                let icons_grid = icons_grid_signal();
                if icons_grid.loaded_icons.len() == icons_grid.icons.len() {
                    return true;
                }
                grid_icons_loader().load_more_icons
            }
        >
            <button on:click=move |_| {
                grid_icons_loader
                    .update(|loader| {
                        loader.load_more_icons = true;
                    });
                icons_grid_signal.update(|grid| grid.load_next_icons());
            }>{move_gettext!(cx, "Load more icons")}</button>
        </div>
    }
}
