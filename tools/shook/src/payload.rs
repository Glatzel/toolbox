use axum::{http::HeaderMap, response::Response};
use serde::{Deserialize, Serialize};

use crate::config::Config;

pub mod github;

pub trait IRunnerSpec {
    fn runner_spec(
        headers: &HeaderMap,
        body: &str,
        config: &Config,
    ) -> Result<RunnerSpec, Response>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunnerSpec {
    pub owner: String,
    pub repo: String,
    pub image: String,
    pub platform: Platform,
    pub cpu_mhz: usize,
    pub memory_mb: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Platform {
    #[serde(rename = "win-64")]
    Win64,
    #[serde(rename = "linux-64")]
    Linux64,
    #[serde(rename = "linux-aarch64")]
    LinuxAarch64,
    #[serde(rename = "osx-arm64")]
    OsxArm64,
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Platform::Win64 => "win-64",
            Platform::Linux64 => "linux-64",
            Platform::LinuxAarch64 => "linux-aarch64",
            Platform::OsxArm64 => "osx-arm64",
        };
        write!(f, "{s}")
    }
}
