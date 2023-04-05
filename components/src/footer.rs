//! App footer

use i18n::gettext;
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
                {move || {gettext!(cx, "Made with ❤️ on GitHub").to_string()}}
            </a>
        </footer>
    }
}

#[component]
pub fn ReportProblems(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex flex-col py-8">
            <p>
                {move || gettext!(cx, "Icon missing?").to_string()}
                {" "}
                <a href="https://github.com/simple-icons/simple-icons/issues/new?assignees=&labels=new+icon&template=icon_request.yml">
                    {move || gettext!(cx, "Submit a request").to_string()}
                </a>
            </p>
            <p>
                {move || gettext!(cx, "Icon outdated?").to_string()}
                {" "}
                <a href="https://github.com/simple-icons/simple-icons/issues/new?assignees=&labels=icon+outdated&template=icon_update.yml">
                    {move || gettext!(cx, "Report outdated icon").to_string()}
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
                <p>
                    "A "
                    <a href="https://github.com/simple-icons/simple-icons/blob/develop/LICENSE.md">
                        "CCO"
                    </a>
                    " project maintained by the "
                    <a href="">"Simple Icons contributors"</a>
                    "."
                </p>
                <p>
                    "Use "
                    <a href="https://github.com/simple-icons/simple-icons">
                        "GitHub"
                    </a>
                    " for requests, corrections and contributions."
                </p>
                <p>
                    "Kindly supported by your donations at "
                    <a href="https://opencollective.com/simple-icons">
                        "Open Collective"
                    </a>
                    "."
                </p>
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
                <svg fill="white" class="h-4 mr-3" role="img" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                    <title>"Twitter logo"</title>
                    <path d=TWITTER_ICON_SVG_PATH/>
                </svg>
                <span>"Share this on Twitter!"</span>
        </a>
    }
}
