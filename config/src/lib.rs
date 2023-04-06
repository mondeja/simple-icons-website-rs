/// Application configuration
pub struct Config {
    /// Number of icons to load
    pub max_icons: Option<usize>,
}

pub const CONFIG: Config = Config {
    max_icons: Some(100),
};
