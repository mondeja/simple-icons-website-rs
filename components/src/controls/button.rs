use leptos::*;

pub trait ActiveFn = Fn() -> bool + 'static;
pub trait TextFn = Fn() -> String + 'static;

/// Abstract control button
#[component]
pub fn ControlButton<A, Ti>(
    cx: Scope,
    /// Button title
    title: Ti,
    /// Button children
    children: Children,
    /// The control is active
    active: A,
) -> impl IntoView
where
    Ti: TextFn,
    A: ActiveFn,
{
    view! { cx,
        <button
            // The class is defined at component scoped CSS
            class=move || if active() {
                "on".to_string()
            } else {
                "off".to_string()
            }
            type="button"
            title=title
        >
            {children(cx)}
        </button>
    }
}

/// Control button made from SVG path
#[component]
pub fn ControlButtonSVGPath<A, Ti>(
    cx: Scope,
    /// Button title
    title: Ti,
    /// Button icon SVG path
    svg_path: &'static str,
    /// The control is active
    active: A,
) -> impl IntoView
where
    Ti: TextFn,
    A: ActiveFn,
{
    view! { cx,
        <ControlButton title=title active=active>
            <svg role="img" viewBox="0 0 24 24">
                <path d=svg_path/>
            </svg>
        </ControlButton>
    }
}

/// Control button made from text
#[component]
pub fn ControlButtonText<A, Ti, Tx>(
    cx: Scope,
    /// Button title
    title: Ti,
    /// Button text
    text: Tx,
    /// The control is active
    active: A,
) -> impl IntoView
where
    Ti: TextFn,
    Tx: TextFn,
    A: ActiveFn,
{
    view! { cx,
        <ControlButton title=title active=active>
            {text}
        </ControlButton>
    }
}
