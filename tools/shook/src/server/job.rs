use async_trait::async_trait;
use axum::http::HeaderMap;
use kioyu::IPayload;
use microsandbox::Sandbox;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::config::{Config, ConfigRunner};

pub trait IJobSpec {
    fn job_spec(headers: &HeaderMap, body: &str, config: &Config) -> Result<JobSpec, super::ShookServerError>;
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
    type Error = super::ShookServerError;

    async fn execute(&self) -> Result<(), Self::Error> {
        let name = format!("{}-{}-{}-{}", self.owner, self.repo, self.job, self.id);
        let mut builder = Sandbox::builder(&name)
            .image(self.runner_spec.image.as_ref())
            .cpus(self.runner_spec.cpus)
            .memory(self.runner_spec.memory);
        for (host, guest) in self.runner_spec.volumes.iter() {
            builder = builder.volume(guest.to_string_lossy().as_ref(), |m| m.bind(host));
        }
        for (key, value) in self.runner_spec.envs.iter() {
            builder = builder.env(key, value);
        }
        for (key, (value, url)) in self.runner_spec.secrets.iter() {
            builder = builder.secret(|s| s.env(key).value(value).allow_host(url));
        }
        clerk::debug!("Sandbox builder configured: {name}");
        let sandbox = builder.create().await?;
        clerk::debug!("Sandbox created: {name}");
        sandbox.wait().await?;
        clerk::debug!("Sandbox finished: {name}");
        Sandbox::remove(sandbox.name()).await?;
        clerk::debug!("Sandbox removed: {name}");
        Ok(())
    }
}
