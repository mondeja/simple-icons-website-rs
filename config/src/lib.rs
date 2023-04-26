pub const DEPRECATED_ICONS_FILE_NAME: &str = "simple-icons-deprecated.txt";

/// Application configuration
pub struct Config {
    /// Number of icons to load
    pub max_icons: Option<usize>,
    /// Number of icons per page in the grid
    pub icons_per_page: u32,
}

/// Development config
#[cfg(debug_assertions)]
pub const CONFIG: Config = Config {
    max_icons: Some(500),
    // WARNING: If you put a great number here, the search functionality
    // will be very slow
    icons_per_page: 30,
};

/// Production config
#[cfg(not(debug_assertions))]
pub const CONFIG: Config = Config {
    max_icons: None,
    // WARNING: If you put a great number here, the search functionality
    // will be very slow
    icons_per_page: 30,
};
