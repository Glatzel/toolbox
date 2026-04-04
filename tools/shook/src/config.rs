use std::path::Path;

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevOpConfig {
    pub token: String,
    pub webhook_secret: String,
    pub allowed_repositories: Vec<String>,
    pub allowed_users: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NomadConfig {
    pub url: String,
    pub port: u16,
    pub timeout_sec: f32,
    pub retry: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_server_config")]
    pub server: ServerConfig,
    pub devop: DevOpConfig,
    pub nomad: NomadConfig,
}
impl Config {
    pub fn load_toml(path: &Path) -> mischief::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }
}
fn default_server_config() -> ServerConfig {
    ServerConfig { port: 8787 }
}
