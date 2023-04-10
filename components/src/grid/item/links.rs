use i18n::{gettext, move_gettext};
use leptos::*;

static LINK_CLASSES: &'static str = concat!(
    "dark:before:invert hover:underline opacity-70 hover:opacity-100",
    " bg-no-repeat pl-5 bg-[size:0.9rem] bg-left-bottom ",
);

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
    // TODO: Rewrite styles with stylesheet and refactor component

    view! { cx,
        <div class="flex flex-col text-xs px-4">
            {
                let mut links = vec![];
                if let Some(guidelines_url) = guidelines_url {
                    let mut class = LINK_CLASSES.to_string();
                    class.push_str(concat!(
                        "bg-[url(./external-link.svg)]",
                        " dark:bg-[url(./external-link-white.svg)]"
                    ));
                    links.push(view!{
                        cx,
                        <a
                            href=guidelines_url
                            title=move_gettext!(cx, "Brand guidelines")
                            class=class
                        >{move_gettext!(cx, "Brand guidelines")}</a>
                    });
                }

                if license_type.is_some() || license_url.is_some() {
                    let mut class = LINK_CLASSES.to_string();
                    class.push_str(concat!(
                        "bg-[url(./legal.svg)]",
                        " dark:bg-[url(./legal-white.svg)]"
                    ));
                    let title = move || match license_type {
                        Some(license_type) => license_type.to_string(),
                        None => gettext!(cx, "License")
                    };
                    links.push(view!{
                        cx,
                        <a
                            href=match license_url {
                                Some(license_url) => license_url.to_string(),
                                None => format!("https://spdx.org/licenses/{}", license_type.unwrap())
                            }
                            title=title
                            class=class
                        >{title}</a>
                    });
                }
                links
            }
        </div>
    }
}
