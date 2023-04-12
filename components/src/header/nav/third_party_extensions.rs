use crate::header::{nav::button::*, HeaderState, HeaderStateSignal};
use crate::modal::*;
use i18n::move_gettext;
use leptos::*;
use macros::get_simple_icons_3rd_party_extensions;
use simple_icons::SimpleIconsExtension;

static THIRD_PARTY_EXTENSIONS: &[SimpleIconsExtension] =
    get_simple_icons_3rd_party_extensions!();

/// Third party extensions table
#[component]
pub fn ThirdPartyExtensionsTable(cx: Scope) -> impl IntoView {
    view! { cx,
        <table class="table-auto border-collapse mx-8 my-1">
            <tbody>
                {THIRD_PARTY_EXTENSIONS
                    .iter()
                    .map(|extension| {
                        view! { cx,
                            <tr>
                                <td>
                                    <a href=extension.url class="hover:opacity-80">
                                        <svg class="h-6 w-6 inline" fill="currentColor" viewBox="0 0 24 24">
                                            <path d=extension.icon_slug></path>
                                        </svg>
                                        <span class="ml-4">{extension.name}</span>
                                    </a>
                                </td>
                                <td>
                                    <a class="hover:opacity-80 ml-6" href=extension.author_url>
                                        {extension.author_name}
                                    </a>
                                </td>
                            </tr>
                        }
                    })
                    .collect::<Vec<_>>()}
            </tbody>
        </table>
    }
}

/// Third party extensions button
#[component]
pub fn ThirdPartyExtensionsButton(cx: Scope) -> impl IntoView {
    let header_state = use_context::<HeaderStateSignal>(cx).unwrap().0;

    view! { cx,
        <HeaderMenuButton
            title=move_gettext!(cx, "Third party extensions")
            additional_classes=move || {
                if header_state().menu_open {
                    "block".to_string()
                } else {
                    "hidden lg:block".to_string()
                }
            }
            on:click=move |_| {
                header_state.update(|state: &mut HeaderState| state.toggle_extensions());
            }
            svg_path="M16.513 23.996a.9.9 0 0 0 .885-.907v-4.972c.303-2.68 1.42-1.884 2.734-1.055 3.178 2.003 5.29-3.266 2.72-4.891-2.015-1.276-2.888.917-4.364.69-.57-.088-.967-.72-1.092-1.68V7.59c0-.5-.398-.907-.885-.907h-4.064c-3.355-.436-.377-2.339-.377-4.11C12.072 1.152 10.816 0 9.267 0 7.721 0 6.301 1.152 6.301 2.573c0 1.67 3.082 3.674-.32 4.11H.884A.898.898 0 0 0 0 7.59v3.583c.26 1.528 1.268 1.882 2.559.874.435-.341 1.17-.738 1.7-.738 1.385 0 2.51 1.285 2.51 2.871s-1.123 3.221-2.51 3.221c-.493 0-.954-.164-1.345-.45 0 .121-2.422-2.232-2.914.648v5.494c0 .5.398.907.885.907 2.728 0 5.453 0 8.18-.002.107-.525-.243-1.125-.571-1.646-2.582-4.1 7.463-4.508 4.88.128-.126.228-.253.45-.35.666-.124.27-.206.599-.188.852z"
        />
    }
}

/// Third party extensions
#[component]
pub fn ThirdPartyExtensions(cx: Scope) -> impl IntoView {
    let header_state = use_context::<HeaderStateSignal>(cx).unwrap().0;

    view! { cx,
        <ThirdPartyExtensionsButton/>
        <Modal
            title=move_gettext!(cx, "Third party extensions")
            is_open=move || header_state().extensions_open
            on_close=move |_| {
                header_state.update(|state: &mut HeaderState| state.extensions_open = false);
            }
        >
            <ThirdPartyExtensionsTable/>
        </Modal>
    }
}
