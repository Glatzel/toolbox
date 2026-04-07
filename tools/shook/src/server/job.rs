use async_trait::async_trait;
use axum::http::HeaderMap;
use axum::response::Response;
use kioyu::job::IPayload;
use microsandbox::Sandbox;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::config::{Config, ConfigRunner};

pub trait IJobSpec {
    fn job_spec(headers: &HeaderMap, body: &str, config: &Config) -> Result<JobSpec, Response>;
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[validate(context = Config)]
pub struct JobSpec {
    pub owner: String,
    #[validate(custom(function = "validate_repository", use_context))]
    pub repo: String,
    pub job: String,
    pub id: usize,
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
    type Error = mischief::Report;

    async fn execute(&self) -> Result<(), Self::Error> {
        let sandbox = Sandbox::builder("my-sandbox")
            .image("python")
            .cpus(1)
            .memory(512)
            .create()
            .await?;

        let output = sandbox.shell("print('Hello from a microVM!')").await?;
        println!("{}", output.stdout()?);

        Sandbox::remove(sandbox.name()).await?;
        Ok(())
    }
}
