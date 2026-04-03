use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub port: u16,
    pub webhook_secret: String,
    pub allowed_repositories: Vec<String>,
    pub allowed_senders: Vec<String>,
}
impl Config {
    pub fn load_toml(path: &Path) -> mischief::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }
}
