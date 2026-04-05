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
    pub job: String,
    pub id: usize,
}
