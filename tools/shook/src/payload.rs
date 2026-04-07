use async_trait::async_trait;
use axum::http::HeaderMap;
use axum::response::Response;
use kioyu::job::IPayload;
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

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[validate(context = Config)]
pub struct RunnerSpec {
    pub owner: String,
    #[validate(custom(function = "validate_repository", use_context))]
    pub repo: String,
    pub job: String,
    pub id: usize,
    pub runner: String,
}

fn validate_repository(repo: &String, context: &Config) -> Result<(), ValidationError> {
    if !context.devop.allowed_repositories.contains(repo) {
        clerk::warn!(repository = %repo, "Repository not in allowlist");
        return Err(ValidationError::new("Repository not allowed"));
    }
    clerk::debug!(repository = %repo, "Repository validated");
    Ok(())
}

#[async_trait]
impl IPayload for RunnerSpec {
    type Error = mischief::Report;

    async fn execute(&self) -> Result<(), Self::Error> {
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        clerk::info!("Executing runner spec", id = self.id);
        Ok(())
    }
}
