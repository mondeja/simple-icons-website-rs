pub const DEPRECATED_ICONS_FILE_NAME: &str = "simple-icons-deprecated.txt";

/// Application configuration
pub struct Config {
    /// Number of icons to load
    pub max_icons: Option<usize>,
    /// Number of icons per page in the grid
    pub icons_per_page: u32,
    /// Time to wait between search input taps and search execution
    pub search_debounce_ms: u32,
}

pub const CONFIG: Config = Config {
    max_icons: None,
    // WARNING: If you put a great number here, the search functionality will be very slow
    icons_per_page: 30,
    // WARNING: If you put a low level here, the search functionality will be very expensive
    search_debounce_ms: 20,
};
