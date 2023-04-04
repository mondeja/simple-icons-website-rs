use leptos::*;

/// Carbon ads add grid item
///
/// This is the main source of income for the Simple Icons project
#[component]
pub fn CarbonAdsAdGridItem(cx: Scope) -> impl IntoView {
    view! { cx,
        <script
            async
            src="//cdn.carbonads.com/carbon.js?serve=CKYIPK7M&placement=simpleiconsorg"
            type="text/javascript"
        />
    }
}
