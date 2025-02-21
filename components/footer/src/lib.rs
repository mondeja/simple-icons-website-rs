use icondata::{SiFacebook, SiMastodon, SiX};
use leptos::{
    html::Footer as FooterHtmlElement,
    prelude::{NodeRef, *},
};
use leptos_fluent::{move_tr, tr};
use leptos_icons::Icon;

/// Footer of the website
#[component]
pub fn Footer(
    /// Reference to the footer container, for using in sibling components
    container_ref: NodeRef<FooterHtmlElement>,
) -> impl IntoView {
    view! {
        <footer node_ref=container_ref>
            <ReportProblems />
            <div class="flex flex-col md:flex-row justify-between lg:-mt-[41px]">
                <About />
                <div class="flex flex-col space-y-2">
                    <FacebookButton />
                    <XButton />
                    <MastodonButton />
                </div>
            </div>
            <a
                class=concat!(
                    "w-full text-center mt-6 hover:underline focus:underline",
                    " text-[var(--link-color)] hover:text-[var(--link-color-hover)]",
                    " focus:text-[var(--link-color-hover)]",
                )

                href="https://github.com/simple-icons/simple-icons-website-rs"
            >
                {move_tr!("made-on")}
            </a>
        </footer>
    }
}

#[component]
fn ReportLink(
    /// Link URL
    href: &'static str,
    /// Link content
    children: Children,
) -> impl IntoView {
    view! {
        <a
            class=concat!(
                "text-[#00e] hover:text-[#3434ee]",
                " focus:text-[#3434ee] visited:text-[#551a8b]",
                " dark:text-[#227fff] dark:hover:text-[#3c8eff]",
                " dark:focus:text-[#3c8eff] dark:visited:text-[#a990bd]",
            )

            href=href
        >
            {children()}
        </a>
    }
}

#[component]
fn ReportProblems() -> impl IntoView {
    view! {
        <div class="flex flex-col py-8">
            <p>
                {move_tr!("icon-missing")} {" "}
                <ReportLink href="https://github.com/simple-icons/simple-icons/issues/new?assignees=&labels=new+icon&template=icon_request.yml">
                    {move_tr!("submit-a-request")}
                </ReportLink>
            </p>
            <p>
                {move_tr!("icon-outdated")} {" "}
                <ReportLink href="https://github.com/simple-icons/simple-icons/issues/new?assignees=&labels=icon+outdated&template=icon_update.yml">
                    {move_tr!("report-outdated-icon")}
                </ReportLink>
            </p>
        </div>
    }
}

#[component]
fn SocialButton(
    href: &'static str,
    icon: icondata::Icon,
    text: Signal<String>,
    class: &'static str,
) -> impl IntoView {
    view! {
        <a
            href=href
            class=format!("social-button {class}")
            rel="noopener"
            role="button"
            target="_blank"
        >
            <Icon icon attr:class="text-white h-4 mr-3" />
            <span>{text}</span>
        </a>
    }
}

#[component]
fn XButton() -> impl IntoView {
    view! {
        <SocialButton
            href="https://x.com/intent/tweet?url=https://simpleicons.org&text=Simple%20Icons%3A%20SVG%20icons%20for%20popular%20brands."
            icon=SiX
            text=move_tr!("share-this-on", { "platform" => "X" })
            class="x-button"
        />
    }
}

#[component]
fn FacebookButton() -> impl IntoView {
    view! {
        <SocialButton
            href="https://www.facebook.com/sharer/sharer.php?u=https%3A%2F%2Fsimpleicons.org"
            icon=SiFacebook
            text=move_tr!("share-this-on", { "platform" => "Facebook" })
            class="facebook-button"
        />
    }
}

#[component]
fn MastodonButton() -> impl IntoView {
    view! {
        <SocialButton
            href="https://mastodonshare.com/?text=Simple%20Icons%3A%20SVG%20icons%20for%20popular%20brands.&url=https%3A%2F%2Fsimpleicons.org"
            icon=SiMastodon
            text=move_tr!("share-this-on", { "platform" => "Mastodon" })
            class="mastodon-button"
        />
    }
}

#[component]
fn About() -> impl IntoView {
    let maintained_by_html = move || {
        tr!("maintained-by", {
            "license" => format!(
                "<a href=\"https://github.com/simple-icons/simple-icons/blob/develop/LICENSE.md\">{}</a>",
                tr!("cco")
            ),
            "maintainers" => format!(
                "<a href=\"https://github.com/simple-icons/simple-icons\">{}</a>",
                tr!("simple-icons-contributors")
            ),
        })
    };
    let use_platform_html = move || {
        tr!("use-platform", {
            "platform" => format!(
                "<a href=\"https://github.com/simple-icons/simple-icons\">{}</a>",
                tr!("github"),
            )
        })
    };
    let supported_by_html = move || {
        tr!("supported-by", {
            "platform" => format!(
                "<a href=\"https://opencollective.com/simple-icons\">{}</a>",
                tr!("open-collective"),
            ),
        })
    };
    view! {
        <div class="footer-about">
            <p inner_html=maintained_by_html></p>
            <p inner_html=use_platform_html></p>
            <p inner_html=supported_by_html></p>
        </div>
    }
}
