//! Application pages
use components::controls::download::provide_download_type_context;
use components::controls::layout::provide_layout_context;
use components::controls::order::provide_order_mode_context;
use components::controls::search::provide_search_context;
use components::controls::{Controls, ControlsProps};
use components::copy::{CopyInput, CopyInputProps};
use components::grid::{provide_icons_grid_contexts, Grid, GridProps};
use components::svg_defs::{SVGDefsDefinition, SVGDefsDefinitionProps};
use leptos::*;

#[component]
pub fn Index(cx: Scope) -> impl IntoView {
    let initial_search_value = provide_search_context(cx);
    let initial_order_mode =
        provide_order_mode_context(cx, &initial_search_value);
    provide_download_type_context(cx);
    provide_layout_context(cx);
    provide_icons_grid_contexts(cx, &initial_search_value, &initial_order_mode);

    view! { cx,
        <SVGDefsDefinition/>
        <CopyInput/>
        <main>
            <Controls/>
            <Grid/>
        </main>
    }
}
