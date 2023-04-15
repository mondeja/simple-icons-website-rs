use crate::header::{nav::button::*, HeaderState, HeaderStateSignal};
use crate::modal::*;
use crate::Url;
use i18n::{move_gettext, Language, LocaleState, LocaleStateSignal, LANGUAGES};
use leptos::*;

pub fn provide_language_context(cx: Scope) -> LocaleStateSignal {
    let language = LocaleStateSignal(create_rw_signal(
        cx,
        LocaleState::new(initial_language(cx)),
    ));
    provide_context(cx, language);
    language
}

pub fn initial_language(cx: Scope) -> Language {
    initial_language_from_url_localstorage_or_navigator_languages(cx)
}

fn initial_language_from_navigator_languages() -> Option<Language> {
    let languages = web_sys::window().unwrap().navigator().languages().to_vec();
    for raw_language in languages {
        let mut language =
            raw_language.as_string().expect("Language not parseable");
        if language.contains('-') {
            language = language.split_once('-').unwrap().0.to_string();
        }
        if let Some(lang) = Language::from_str(language.as_str()) {
            return Some(lang);
        }
    }
    None
}

fn initial_language_from_url_localstorage_or_navigator_languages(
    cx: Scope,
) -> Language {
    let language: Option<Language> =
        match Url::params::get(cx, &Url::params::Names::Language) {
            Some(value) => match Language::from_str(value.as_str()) {
                Some(lang) => {
                    set_language_in_localstorage(lang);
                    Some(lang)
                }
                None => None,
            },
            None => None,
        };

    match language {
        Some(lang) => lang,
        None => match initial_language_from_localstorage() {
            Some(lang) => lang,
            None => match initial_language_from_navigator_languages() {
                Some(lang) => lang,
                None => Language::default(),
            },
        },
    }
}

fn initial_language_from_localstorage() -> Option<Language> {
    let window = web_sys::window().unwrap();
    let local_storage = window.local_storage().unwrap().unwrap();

    match local_storage.get_item("language") {
        Ok(Some(language)) => match Language::from_str(language.as_str()) {
            Some(lang) => Some(lang),
            None => None,
        },
        _ => None,
    }
}

pub fn set_language_in_localstorage(lang: Language) {
    let window = web_sys::window().unwrap();
    let local_storage = window.local_storage().unwrap().unwrap();

    local_storage.set_item("language", lang.code).unwrap();
}

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
                                    set_language_in_localstorage(lang.clone());
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
