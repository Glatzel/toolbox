mod devop;
mod kioyu;
mod runner;
mod server;
use std::path::Path;

pub use devop::*;
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::config::devop::{ConfigDevOp, RawConfigDevOp};
use crate::config::kioyu::{ConfigKioyu, RawConfigKioyu};
use crate::config::runner::{ConfigRunner, RawConfigRunner};
use crate::config::server::{ConfigServer, RawConfigServer};

pub trait IResolve<T> {
    fn resolve(self) -> T;
}
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
struct RawConfig {
    #[validate(nested)]
    pub server: RawConfigServer,
    #[validate(nested)]
    pub devop: RawConfigDevOp,
    #[validate(nested)]
    pub kioyu: RawConfigKioyu,
    #[validate(nested)]
    pub runners: RawConfigRunner,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ConfigServer,
    pub devop: ConfigDevOp,
    pub kioyu: ConfigKioyu,
    pub runners: HashMap<String, ConfigRunner>,
}
impl Config {
    pub fn load_config(path: &Path) -> mischief::Result<Self> {
        clerk::debug!(path = %path.display(), "Loading config file");

        let content = std::fs::read_to_string(path).inspect_err(|e| {
            clerk::error!(path = %path.display(), error = %e, "Failed to read config file");
        })?;

        let raw_config: RawConfig = toml::from_str(&content).inspect_err(|e| {
            clerk::error!(path = %path.display(), error = %e, "Failed to parse config TOML");
        })?;
        clerk::debug!(?raw_config);
        raw_config.validate()?;

        let config = Config {
            server: raw_config.server.resolve(),
            devop: raw_config.devop.resolve(),
            kioyu: raw_config.kioyu.resolve(),
            runners: raw_config.runners.resolve(),
        };

        clerk::info!(
            path = %path.display(),
            ?config,
            "Config loaded successfully"
        );

        Ok(config)
    }
}
