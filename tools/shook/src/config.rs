use std::path::Path;

use serde::{Deserialize, Serialize};
const CONFIG_HEADING: &str = "Config";
#[derive(Debug, Clone, Serialize, Deserialize, clap::Args)]
pub struct Config {
    #[arg(long, group = "config",help_heading=CONFIG_HEADING)]
    pub port: u16,
    #[arg(long, group = "config",help_heading=CONFIG_HEADING)]
    pub webhook_secret: String,
    #[arg(long, group = "config",help_heading=CONFIG_HEADING)]
    pub allowed_repositories: Vec<String>,
    #[arg(long, group = "config",help_heading=CONFIG_HEADING)]
    pub allowed_senders: Vec<String>,
}
impl Config {
    pub fn load_toml(path: &Path) -> mischief::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }
}
