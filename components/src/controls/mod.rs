//! App controls

use leptos::*;

/// Control buttons
#[component]
pub fn ControlButton(
    cx: Scope,
    /// Button title
    title: &'static str,
    /// Button icon SVG path
    svg_path: &'static str,
) -> impl IntoView {
    view! { cx,
        <button class="w-10 h-10 p-1.5" type="button" title=title>
            <svg
                role="img"
                viewBox="0 0 24 24"
                xmlns="http://www.w3.org/2000/svg"
            >
                <path d=svg_path/>
            </svg>
        </button>
    }
}

#[component]
pub fn Controls(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex flex-row space-x-4">
            <SearchControl/>
            <OrderControl/>
            <ColorSchemeControl/>
            <DownloadFileTypeControl/>
            <LayoutControl/>
        </div>
    }
}

#[component]
pub fn SearchControl(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex flex-col">
            <label for="search" class="font-bold">"Search"</label>
            <input
                id="search"
                type="search"
                class="border px-2 py-1 h-10"
                placeholder="Search by brand..."
            />
        </div>
    }
}

#[component]
pub fn OrderControl(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex flex-col">
            <label class="font-bold">"Order"</label>
            <div class="flex flex-row">
                <ControlButton
                    title="Sort alphabetically"
                    svg_path="M10.096 18.857H7.525V.429A.414.414 0 0 0 7.096 0H4.525a.414.414 0 0 0-.429.429v18.429H1.525c-.196 0-.331.089-.402.268-.072.17-.04.326.094.469l4.286 4.286c.098.079.2.119.308.119.116 0 .219-.04.308-.12l4.272-4.272a.506.506 0 0 0 .134-.321.414.414 0 0 0-.429-.43zm10.006 3.617H16.78c-.188 0-.322.009-.402.026l-.188.026V22.5l.148-.147c.133-.16.227-.276.281-.348l4.941-7.099v-1.191h-7.594v3.066h1.607v-1.54h3.107c.16 0 .295-.014.4-.04a.856.856 0 0 0 .102-.007c.039-.004.068-.007.086-.007v.04l-.146.121c-.08.08-.176.2-.281.361L13.9 22.795V24h7.82v-3.12h-1.619v1.594h.001zm1.875-13.608L18.895 0h-2.168l-3.082 8.866h-.936v1.419h3.842V8.866h-1.004l.631-1.929h3.254l.629 1.929h-1.004v1.419h3.857V8.866h-.937zm-5.358-3.402.977-2.92c.037-.107.07-.236.102-.388s.047-.232.047-.241l.039-.268h.055c0 .036.008.125.025.268l.162.629.963 2.92h-2.37z"
                />
                <ControlButton
                    title="Sort by color"
                    svg_path="M9.219 18.857H6.648V.429A.414.414 0 0 0 6.219 0H3.648a.418.418 0 0 0-.308.121.418.418 0 0 0-.121.308v18.428H.648a.4.4 0 0 0-.402.268c-.071.17-.04.326.094.469l4.286 4.287c.098.08.2.119.308.119a.447.447 0 0 0 .308-.119l4.272-4.273a.506.506 0 0 0 .134-.32.417.417 0 0 0-.429-.431zm3-15.428h3.429A.412.412 0 0 0 16.076 3V.429A.416.416 0 0 0 15.648 0h-3.429a.414.414 0 0 0-.429.429V3a.414.414 0 0 0 .429.429zm0 6.857h6a.42.42 0 0 0 .309-.12.42.42 0 0 0 .121-.308V7.286a.418.418 0 0 0-.121-.308.417.417 0 0 0-.309-.121h-6a.414.414 0 0 0-.308.121.417.417 0 0 0-.12.308v2.572c0 .125.04.228.12.308a.42.42 0 0 0 .308.12zm0 6.857h8.572c.125 0 .229-.039.309-.119s.119-.184.119-.309v-2.572c0-.125-.039-.227-.119-.307s-.184-.121-.309-.121h-8.572a.418.418 0 0 0-.308.121.415.415 0 0 0-.12.307v2.572c0 .125.04.229.12.309.081.08.183.119.308.119zm11.451 3.55a.408.408 0 0 0-.307-.121H12.219a.416.416 0 0 0-.429.428v2.572c0 .125.04.227.121.309a.42.42 0 0 0 .308.119h11.144a.414.414 0 0 0 .307-.119.424.424 0 0 0 .121-.309V21a.416.416 0 0 0-.121-.307z"
                />
            </div>
        </div>
    }
}

#[component]
pub fn ColorSchemeControl(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex flex-col">
            <label class="font-bold">"Mode"</label>
            <div class="flex flex-row">
                <ControlButton
                    title="Light color scheme"
                    svg_path="M12 18c-3.309 0-6-2.691-6-6s2.691-6 6-6 6 2.691 6 6-2.691 6-6 6zm0-10c-2.206 0-4 1.794-4 4s1.794 4 4 4 4-1.794 4-4-1.794-4-4-4zm0-4a1 1 0 0 1-1-1V1a1 1 0 0 1 2 0v2a1 1 0 0 1-1 1zm0 20a1 1 0 0 1-1-1v-2a1 1 0 1 1 2 0v2a1 1 0 0 1-1 1zM5.64 6.64a.997.997 0 0 1-.707-.293l-1.42-1.42a.999.999 0 1 1 1.414-1.414l1.42 1.42A.999.999 0 0 1 5.64 6.64zm14.139 14.139a.997.997 0 0 1-.707-.293l-1.42-1.42a.999.999 0 1 1 1.414-1.414l1.42 1.42a.999.999 0 0 1-.707 1.707zM3 13H1a1 1 0 1 1 0-2h2a1 1 0 0 1 0 2zm20 0h-2a1 1 0 1 1 0-2h2a1 1 0 1 1 0 2zM4.22 20.779a.999.999 0 0 1-.707-1.707l1.42-1.42a.999.999 0 1 1 1.414 1.414l-1.42 1.42a.993.993 0 0 1-.707.293zM18.359 6.64a.999.999 0 0 1-.707-1.707l1.42-1.42a.999.999 0 1 1 1.414 1.414l-1.42 1.42a.997.997 0 0 1-.707.293z"
                />
                <ControlButton
                    title="Dark color scheme"
                    svg_path="M12.048 21.963c-.308 0-.618-.015-.93-.043-2.66-.246-5.064-1.513-6.771-3.567s-2.512-4.651-2.266-7.311a10.004 10.004 0 0 1 9.038-9.038 1 1 0 0 1 .896 1.589 6.008 6.008 0 0 0 1.258 8.392c2.078 1.536 5.055 1.536 7.133 0a1 1 0 0 1 1.591.896 9.951 9.951 0 0 1-9.949 9.082zM9.315 4.438a8.006 8.006 0 0 0-5.244 6.787 7.954 7.954 0 0 0 1.813 5.849 7.95 7.95 0 0 0 5.417 2.854 7.95 7.95 0 0 0 8.266-5.243 8.01 8.01 0 0 1-2.729.476 7.946 7.946 0 0 1-4.755-1.565C9.174 11.443 8.145 7.68 9.315 4.438z"
                />
                <ControlButton
                    title="System color scheme"
                    svg_path="M12 4a1 1 0 0 1-1-1V1a1 1 0 0 1 2 0v2a1 1 0 0 1-1 1zM4.933 6.348a.997.997 0 0 0 1.414 0 .999.999 0 0 0 0-1.414l-1.42-1.42a.999.999 0 1 0-1.414 1.414l1.42 1.42zM1 13h2a1 1 0 1 0 0-2H1a1 1 0 0 0 0 2zm19.486-8.072 3.221-3.221A.999.999 0 1 0 22.293.293l-3.221 3.221-1.42 1.42-2.19 2.19A5.955 5.955 0 0 0 12 6c-3.309 0-6 2.691-6 6 0 1.258.406 2.453 1.124 3.462l-2.105 2.105c-.026.021-.058.03-.083.055s-.033.056-.055.082l-1.368 1.368-.001.002-3.219 3.219a.999.999 0 1 0 1.414 1.414l3.987-3.987a10.03 10.03 0 0 0 6.332 2.262c5.103-.001 9.473-3.902 9.951-9.081a1 1 0 0 0-1.591-.896 5.96 5.96 0 0 1-7.037.06l5.717-5.716 1.42-1.421zm-.945 9.78c-1.21 3.337-4.564 5.587-8.257 5.238a8.019 8.019 0 0 1-4.165-1.651l4.802-4.802c.05.038.093.082.144.12a7.955 7.955 0 0 0 7.476 1.095zm-10.979-.684A3.968 3.968 0 0 1 8 12c0-2.206 1.794-4 4-4a3.98 3.98 0 0 1 2.025.561l-5.463 5.463z"
                />
            </div>
        </div>
    }
}

#[component]
pub fn DownloadFileTypeControl(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex flex-col">
            <label class="font-bold">"Download"</label>
            <div class="flex flex-row">
                <button class="font-bold w-10 h-10 p-1.5" type="button" title="Download SVG">
                    <span>"SVG"</span>
                </button>
                <button class="font-bold w-10 h-10 p-1.5" type="button" title="Download PDF">
                    <span>"PDF"</span>
                </button>
            </div>
        </div>
    }
}

#[component]
pub fn LayoutControl(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex flex-col">
            <label class="font-bold">"Layout"</label>
            <div class="flex flex-row">
                <ControlButton
                    title="Comfortable"
                    svg_path="M19 2a2 2 0 0 1 2 2v2a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h14zm0 4V4H5v2h14zm0 10a2 2 0 0 1 2 2v2a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-2a2 2 0 0 1 2-2h14zm0 4v-2H5v2h14zm0-11a2 2 0 0 1 2 2v2a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-2a2 2 0 0 1 2-2h14zm0 4v-2H5v2h14z"
                />
                <ControlButton
                    title="Compact"
                    svg_path="M2 5.5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v13a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-13zm9 0H4v3h7v-3zm2 0v3h7v-3h-7zm7 5h-7v3h7v-3zm0 5h-7v3h7v-3zm-9 3v-3H4v3h7zm-7-5h7v-3H4v3z"
                />
            </div>
        </div>
    }
}
