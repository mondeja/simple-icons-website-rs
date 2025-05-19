use super::button::ControlButtonIcon;
use icondata::{ChMoon, ChSun, TbSunMoon};
use leptos::prelude::*;
use leptos_fluent::move_tr;
use leptos_use::ColorMode;

#[component]
pub fn ColorSchemeControl() -> impl IntoView {
    let color_scheme = use_context::<Signal<ColorMode>>().unwrap();
    let set_color_scheme = use_context::<WriteSignal<ColorMode>>().unwrap();

    view! {
        <div class="control">
            <label>{move_tr!("theme")}</label>
            {move || {
                let current_color_scheme = color_scheme();
                view! {
                    <div>
                        <ControlButtonIcon
                            title=move_tr!("light-color-scheme")
                            icon=ChSun
                            active=current_color_scheme == ColorMode::Light
                            on:click=move |_| set_color_scheme(ColorMode::Light)
                        />
                        <ControlButtonIcon
                            title=move_tr!("dark-color-scheme")
                            icon=ChMoon
                            active=current_color_scheme == ColorMode::Dark
                            on:click=move |_| set_color_scheme(ColorMode::Dark)
                        />
                        <ControlButtonIcon
                            title=move_tr!("system-color-scheme")
                            icon=TbSunMoon
                            active=current_color_scheme == ColorMode::Auto
                            on:click=move |_| set_color_scheme(ColorMode::Auto)
                        />
                    </div>
                }
            }}
        </div>
    }
}
