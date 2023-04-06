#[allow(non_snake_case)]
pub mod LocalStorage {
    pub enum Keys {
        DownloadType,
        OrderMode,
    }

    impl Keys {
        pub fn as_str(&self) -> &'static str {
            match self {
                Keys::DownloadType => "download-type",
                Keys::OrderMode => "order-mode",
            }
        }
    }
}
