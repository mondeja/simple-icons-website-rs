//! URL utilities working with Leptos
//!
//! Currently, there is not a way to reactively maintain the state
//! of the URL of the page, so we need to hand craft some convenient
//! utilities

/// Single source of thruth for the URL params state
pub mod params {
    use leptos::{window, Scope};
    use leptos_router::{use_location, ParamsMap};
    use wasm_bindgen;

    /// Enum to ensure that the params names are unique
    pub enum Names {
        Search,
        Language,
        DownloadType,
    }

    impl Names {
        pub fn as_str(&self) -> &'static str {
            match self {
                Self::Search => "q",
                Self::Language => "lang",
                Self::DownloadType => "download-type",
            }
        }
    }

    /// Update a parameter value in the URL query using window history
    #[inline(always)]
    pub fn update(cx: Scope, k: &Names, v: &str) {
        let location = use_location(cx);
        let mut params = (location.query)();
        // Remove empty values from the URL!
        if v.is_empty() {
            params.remove(k.as_str());
        } else {
            params.insert(k.as_str().to_string(), v.to_string());
        }

        let query = to_query_string(&params);
        window()
            .history()
            .unwrap()
            .replace_state_with_url(
                &wasm_bindgen::JsValue::NULL,
                "",
                Some(&match query == "?" {
                    true => (location.pathname)(),
                    false => query,
                }),
            )
            .ok();
    }

    /// Get a URL param value using the Leptos router
    #[inline(always)]
    pub fn get(cx: Scope, k: &Names) -> Option<String> {
        match (use_location(cx).query)().get(k.as_str()) {
            Some(value) => match value.is_empty() {
                true => None,
                false => Some(value.to_string()),
            },
            None => None,
        }
    }

    /// Get a URL param value directly from URL of the browser
    ///
    /// This method has the advantadge that it doesn't needs a Leptos,
    /// so it can be used before the initialization of it. Useful
    /// to get information that must be setted before the initialization
    /// of the body of the page.
    #[inline(always)]
    pub fn get_vanilla(k: &Names) -> Option<String> {
        let query = window().location().search().unwrap();
        if !query.starts_with("?") {
            return None;
        }
        for key_value in query.split("?").last().unwrap().split("&").into_iter()
        {
            if key_value.contains("=") {
                let mut split = key_value.split("=");
                if split.next().unwrap() == k.as_str() {
                    let ret = split.next().unwrap();
                    return if ret.is_empty() {
                        None
                    } else {
                        Some(ret.to_string())
                    };
                }
            } else if key_value == k.as_str() {
                return None;
            }
        }
        None
    }

    // `to_query_string` has currently bad support by Leptos,
    // see https://github.com/leptos-rs/leptos/pull/854
    // TODO: remove when the merged PR is released
    #[inline(always)]
    pub fn to_query_string(params: &ParamsMap) -> String {
        if params.0.is_empty() {
            return String::from("?");
        }
        let mut query = params.to_query_string();
        query.pop();
        query
    }
}
