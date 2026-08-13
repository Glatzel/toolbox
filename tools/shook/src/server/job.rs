use axum::http::HeaderMap;

use crate::config::Config;
use crate::vm::RunnerPayload;

pub trait IRunnerPayload {
    fn runner_payload(
        headers: &HeaderMap,
        body: &str,
        config: &Config,
    ) -> Result<RunnerPayload, super::ShookServerError>;
}
