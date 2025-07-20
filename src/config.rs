use anyhow::Result;
use config::{Config, Environment};
use serde::Deserialize;

/// Application-wide configuration loaded from environment variables.
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub environment: String,
    pub log_level: String,
}

/// Loads configuration using environment variables.
/// Fails early if required fields are missing or invalid.
pub fn load_config() -> Result<AppConfig> {
    Config::builder()
        .add_source(Environment::default())
        .build()?
        .try_deserialize::<AppConfig>()
        .map_err(Into::into)
}
