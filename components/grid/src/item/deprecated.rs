use leptos::prelude::*;
use leptos_fluent::move_tr;

#[component]
pub fn IconIsDeprecatedNotice(
    /// Icon brand title
    title: Memo<&'static str>,
    /// Link to the pull request that is removing the icon
    pull_request_url: String,
    /// Removal version
    at_version: &'static str,
) -> impl IntoView {
    let title = move_tr!("will-be-removed-at", {
        "icon" => title(),
        "version" => at_version,
    });
    view! {
        <a href=pull_request_url class="deprecated" title=title target="_blank">
            <span></span>
            <p>{move_tr!("deprecated")}</p>
        </a>
    }
}
