//! App controls

use leptos::*;

#[component]
pub fn Controls(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex flex-row">
            <SearchControl/>
        </div>
    }
}

#[component]
pub fn SearchControl(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex flex-col">
            <label for="search" class="font-bold text-gray-700">"Search"</label>
            <input id="search" type="text" class="border border-gray-300 rounded-md px-2 py-1" placeholder="Search by brand..."/>
        </div>
    }
}
