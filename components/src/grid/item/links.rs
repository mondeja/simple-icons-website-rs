use i18n::{gettext, move_gettext};
use leptos::*;

#[component]
pub fn IconGridItemLinks(
    cx: Scope,
    /// Brand guidelines URL
    guidelines_url: Option<&'static str>,
    /// License URL
    license_url: Option<&'static str>,
    /// License type
    license_type: Option<&'static str>,
) -> impl IntoView {
    view! { cx,
        <div class="links">
            {
                let mut links = vec![];
                if let Some(guidelines_url) = guidelines_url {
                    links
                        .push(
                            view! { cx,
                                <a
                                    href=guidelines_url
                                    title=move_gettext!(cx, "Brand guidelines")
                                    class="brand-guidelines"
                                >
                                    {move_gettext!(cx, "Brand guidelines")}
                                </a>
                            },
                        );
                }
                if license_type.is_some() || license_url.is_some() {
                    let title = move || match license_type {
                        Some(license_type) => license_type.to_string(),
                        None => gettext!(cx, "License"),
                    };
                    links
                        .push(
                            view! { cx,
                                <a
                                    href=match license_url {
                                        Some(license_url) => license_url.to_string(),
                                        None => format!("https://spdx.org/licenses/{}", license_type.unwrap()),
                                    }
                                    title=title
                                    class="license"
                                >
                                    {title}
                                </a>
                            },
                        );
                }
                links
            }
        </div>
    }
}
