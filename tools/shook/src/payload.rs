use axum::{http::HeaderMap, response::Response};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::config::Config;
pub mod github;
pub trait IRunnerSpec {
    fn runner_spec(
        headers: &HeaderMap,
        body: &str,
        config: &Config,
    ) -> Result<RunnerSpec, Response>;
}
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RunnerSpec {
    pub image: String,
    #[validate(custom(function = "validate_platform"))]
    pub platform: Platform,
    pub cpu_mhz: usize,
    pub memory_mb: usize,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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
impl Platform {
    pub fn os(&self) -> &'static str {
        match self {
            Platform::Win64 => "windows",
            Platform::Linux64 | Platform::LinuxAarch64 => "linux",
            Platform::OsxArm64 => "macos",
        }
    }
    pub fn arch(&self) -> &'static str {
        match self {
            Platform::Win64 | Platform::Linux64 => "x86_64",
            Platform::LinuxAarch64 | Platform::OsxArm64 => "aarch64",
        }
    }
}

fn validate_platform(platform: &Platform) -> Result<(), ValidationError> {
    if std::env::consts::OS != platform.os() {
        return Err(ValidationError::new("Host OS not equal to required OS"));
    }
    if std::env::consts::ARCH != platform.arch() {
        return Err(ValidationError::new("Host Arch not equal to required Arch"));
    }
    Ok(())
}
