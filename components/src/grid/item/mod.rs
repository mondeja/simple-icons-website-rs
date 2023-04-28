mod deprecated;
pub mod details;
mod footer;
pub(crate) mod icon_preview;
mod links;
mod title;

use deprecated::*;
use footer::*;
use icon_preview::*;
use links::*;
use title::*;
use types::SimpleIcon;

use leptos::*;

/// Icon grid item
///
/// Each icon displayed in the icons grid
#[component]
pub fn IconGridItem(
    cx: Scope,
    /// Icon
    icon: &'static SimpleIcon,
) -> impl IntoView {
    view! { cx,
        <li>
            <IconGridItemPreview slug=icon.slug title=icon.title/>
            <IconGridItemLinks
                guidelines_url=icon.guidelines
                license_url=icon.license_url
                license_type=icon.license_type
            />
            {icon
                .deprecation
                .as_ref()
                .map(|deprecation| {
                    view! { cx,
                        <IconIsDeprecatedNotice
                            title=icon.title
                            pull_request_url=deprecation.get_pull_request_url()
                            removal_at_version=deprecation.removal_at_version
                        />
                    }
                })}
            <IconGridItemTitle title=icon.title slug=icon.slug/>
            <IconGridItemFooter icon=icon/>
        </li>
    }
}
