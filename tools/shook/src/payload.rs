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
#[derive(Debug)]
pub struct RunnerSpec {
    pub _image: String,
    pub _platform: Platform,
    pub _cpu_mhz: usize,
    pub _memory_mb: usize,
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
