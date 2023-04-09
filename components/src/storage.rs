#[allow(non_snake_case)]
pub mod LocalStorage {
    pub enum Keys {
        DownloadType,
        OrderMode,
        SearchValue,
        ColorScheme,
        Layout,
    }

    impl Keys {
        pub fn as_str(&self) -> &'static str {
            match self {
                Keys::DownloadType => "download-type",
                Keys::OrderMode => "order-mode",
                Keys::SearchValue => "search-value",
                Keys::ColorScheme => "color-scheme",
                Keys::Layout => "layout",
            }
        }
    }
}
