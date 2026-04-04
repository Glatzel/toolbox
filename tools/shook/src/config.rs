use std::path::Path;

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Vendor {
    #[serde(rename = "bitbucket", alias = "atlassian")]
    Bitbucket,
    Forgejo,
    Gitea,
    Github,
    Gitlab,
    Woodpecker,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevOpConfig {
    pub vender: Vendor,
    pub token: String,
    pub webhook_secret: String,
    pub allowed_repositories: Vec<String>,
    pub allowed_users: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NomadConfig {
    pub url: String,
    pub timeout_sec: f32,
    pub retry: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_server_config")]
    pub server: ServerConfig,

    pub devop: DevOpConfig,

    #[serde(default = "default_nomad_config")]
    pub nomad: NomadConfig,
}
impl Config {
    pub fn load_toml(path: &Path) -> mischief::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }
}
fn default_nomad_config() -> NomadConfig {
    NomadConfig {
        url: String::from("http://localhost:4646/v1/"),
        timeout_sec: 3.0,
        retry: 3,
    }
}

fn default_server_config() -> ServerConfig {
    ServerConfig { port: 8787 }
}
