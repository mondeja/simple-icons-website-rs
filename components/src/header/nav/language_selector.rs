use crate::header::{nav::button::*, HeaderState, HeaderStateSignal};
use crate::modal::*;
use i18n::{move_gettext, Language, LocaleState, LocaleStateSignal, LANGUAGES};
use leptos::*;

/// Languages list
#[component]
pub fn LanguagesList(cx: Scope) -> impl IntoView {
    let header_state = use_context::<HeaderStateSignal>(cx).unwrap().0;
    let locale_state = use_context::<LocaleStateSignal>(cx).unwrap().0;

    view! { cx,
        <ul class="language-selector">
            {move || {
                let current_language_code = locale_state().current_language.code;
                LANGUAGES
                    .iter()
                    .map(|lang: &Language| {
                        view! { cx,
                            <li
                                class:hidden=lang.code == current_language_code
                                on:click=move |_| {
                                    header_state.update(|state: &mut HeaderState| state.toggle_languages());
                                    locale_state.update(|state: &mut LocaleState| state.set_current_language(lang.code));
                                }
                            >
                                {lang.name}
                            </li>
                        }
                    })
                    .collect::<Vec<_>>()
            }}
        </ul>
    }
}

/// Language selector button
#[component]
pub fn LanguageSelectorButton(cx: Scope) -> impl IntoView {
    let header_state = use_context::<HeaderStateSignal>(cx).unwrap().0;

    view! { cx,
        <HeaderMenuButton
            title=move_gettext!(cx, "Change language")
            additional_classes=move || {
                if header_state().menu_open {
                    "block".to_string()
                } else {
                    "hidden lg:block".to_string()
                }
            }
            on:click=move |_| {
                header_state.update(|state: &mut HeaderState| state.toggle_languages());
            }
            svg_path="m12.87 15.07-2.54-2.51.03-.03A17.52 17.52 0 0 0 14.07 6H17V4h-7V2H8v2H1v1.99h11.17C11.5 7.92 10.44 9.75 9 11.35 8.07 10.32 7.3 9.19 6.69 8h-2c.73 1.63 1.73 3.17 2.98 4.56l-5.09 5.02L4 19l5-5 3.11 3.11.76-2.04zM18.5 10h-2L12 22h2l1.12-3h4.75L21 22h2l-4.5-12zm-2.62 7 1.62-4.33L19.12 17h-3.24z"
        />
    }
}

/// Language selector
#[component]
pub fn LanguageSelector(cx: Scope) -> impl IntoView {
    let header_state = use_context::<HeaderStateSignal>(cx).unwrap().0;

    view! { cx,
        <LanguageSelectorButton/>
        <Modal
            is_open=move || header_state().languages_open
            title=move_gettext!(cx, "Select a language")
            on_close=move |_| {
                header_state.update(|state: &mut HeaderState| state.languages_open = false);
            }
        >
            <LanguagesList/>
        </Modal>
    }
}
