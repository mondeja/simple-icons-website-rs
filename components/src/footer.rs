//! App footer

use i18n::{gettext, move_gettext};
use leptos::*;
use macros::simple_icon_svg_path;

static TWITTER_ICON_SVG_PATH: &str = simple_icon_svg_path!("twitter");

#[component]
pub fn Footer(cx: Scope) -> impl IntoView {
    view! { cx,
        <footer class="flex flex-col justify-between py-8 text-sm">
            <ReportProblems/>
            <About/>
            <a
                class="w-full text-center mt-6"
                href="https://github.com/simple-icons/simple-icons-website"
            >
                {move_gettext!(cx, "Made with ❤️ on GitHub")}
            </a>
        </footer>
    }
}

#[component]
pub fn ReportProblems(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex flex-col py-8">
            <p>
                {move_gettext!(cx, "Icon missing?")}
                {" "}
                <a href="https://github.com/simple-icons/simple-icons/issues/new?assignees=&labels=new+icon&template=icon_request.yml">
                    {move_gettext!(cx, "Submit a request")}
                </a>
            </p>
            <p>
                {move_gettext!(cx, "Icon outdated?")}
                {" "}
                <a href="https://github.com/simple-icons/simple-icons/issues/new?assignees=&labels=icon+outdated&template=icon_update.yml">
                    {move_gettext!(cx, "Report outdated icon")}
                </a>
            </p>
        </div>
    }
}

#[component]
pub fn About(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex flex-row justify-between">
            <div class="flex flex-col">
                <p inner_html=move_gettext!(
                    cx,
                    "A {} project maintained by the {}.",
                    &format!(
                        "<a href=\"https://github.com/simple-icons/simple-icons/blob/develop/LICENSE.md\">{}</a>",
                        gettext!(cx, "CCO")
                    ),
                    &format!(
                        "<a href=\"https://github.com/simple-icons/simple-icons\">{}</a>",
                        gettext!(cx, "Simple Icons contributors")
                    )
                )/>
                <p inner_html=move_gettext!(
                    cx,
                    "Use {} for requests, corrections and contributions.",
                    &format!(
                        "<a href=\"https://github.com/simple-icons/simple-icons\">{}</a>",
                        gettext!(cx, "GitHub")
                    )
                )/>
                <p inner_html=move_gettext!(
                    cx,
                    "Kindly supported by your donations at {}.",
                    &format!(
                        "<a href=\"https://opencollective.com/simple-icons\">{}</a>",
                        gettext!(cx, "Open Collective")
                    )
                )/>
            </div>
            <TwitterButton/>
        </div>
    }
}

#[component]
pub fn TwitterButton(cx: Scope) -> impl IntoView {
    view! { cx,
        <a
            class="flex flex-row items-center h-0 my-auto align-middle bg-[#1DA1F2] text-white rounded-md px-3 py-5"
            rel="noopener"
            role="button"
            target="_blank"
            href="https://twitter.com/intent/tweet?url=https://simpleicons.org&amp;text=Simple%20Icons%3A%20free%20SVG%20icons%20for%20popular%20brands.">
                <svg fill="white" class="h-4 mr-3" role="img" viewBox="0 0 24 24">
                    <path d=TWITTER_ICON_SVG_PATH/>
                </svg>
                <span>{move_gettext!(cx, "Share this on Twitter!")}</span>
        </a>
    }
}
