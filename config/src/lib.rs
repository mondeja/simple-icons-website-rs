pub const DEPRECATED_ICONS_FILE_NAME: &str = "simple-icons-deprecated.txt";

/// Application configuration
pub struct Config {
    /// Number of icons to load
    pub max_icons: Option<usize>,
    /// Minimum search score to appear in search results
    pub min_search_score: f32,
    /// Number of icons per page
    pub max_icons_per_page: usize,
}

pub const CONFIG: Config = Config {
    max_icons: Some(250),
    min_search_score: 0.5,
    max_icons_per_page: 20,
};
