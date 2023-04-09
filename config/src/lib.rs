/// Application configuration
pub struct Config {
    /// Number of icons to load
    pub max_icons: Option<usize>,
    /// Minimum search score to appear in search results
    pub min_search_score: f32,
}

pub const CONFIG: Config = Config {
    max_icons: Some(100),
    min_search_score: 0.01,
};
