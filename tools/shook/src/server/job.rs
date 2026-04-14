use async_trait::async_trait;
use axum::http::HeaderMap;
use kioyu::IPayload;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::config::{Config, ConfigRunner};
use crate::vm::{build_sandbox, drain_sandbox_handle, start_runner, stop_and_remove_sandbox};

pub trait IJobSpec {
    fn job_spec(
        headers: &HeaderMap,
        body: &str,
        config: &Config,
    ) -> Result<JobSpec, super::ShookServerError>;
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[validate(context = Config)]
pub struct JobSpec {
    pub owner: String,
    #[validate(custom(function = "validate_repository", use_context))]
    pub repo: String,
    pub job: String,
    pub id: usize,
    pub token: String,
    pub runner_spec: ConfigRunner,
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
impl IPayload for JobSpec {
    type Error = super::ShookServerError;

    async fn execute(&self) -> Result<(), Self::Error> {
        let sandbox_name = format!("{}-{}-{}-{}", self.owner, self.repo, self.job, self.id);
        let sandbox = build_sandbox(
            &sandbox_name,
            self.runner_spec.image.as_ref(),
            self.runner_spec.cpus,
            self.runner_spec.memory,
            &self.runner_spec.volumes,
            &self.runner_spec.ports,
            &self.runner_spec.envs,
            &self.runner_spec.secrets,
        )
        .await?;
        let handle = start_runner(&sandbox, &self.owner, &self.repo, &self.token).await?;
        drain_sandbox_handle(handle).await;
        stop_and_remove_sandbox(&sandbox).await;
        Ok(())
    }
}
