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
#[derive(Debug, Serialize, Deserialize)]
pub struct RunnerSpec {
    pub owner: String,
    pub repo: String,
    pub image: String,
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
impl RunnerSpec {
    pub fn nomad_body(&self) -> String {
        serde_json::json!({
        "Meta": self
        })
        .to_string()
    }
}
