use serde::{Deserialize, Deserializer, Serialize};

use strum::VariantNames;
use validator::{Validate, ValidationError};

use crate::{
    config::Config,
    payload::{Arch, IRunnerSpec, Os, RunnerSpec},
};

#[derive(Debug, Deserialize, Validate)]
#[validate(context = Config)]
pub struct WebhookPayload {
    #[validate(custom(function = "validate_event"))]
    event: Event,
    #[validate(custom(function = "validate_repository", use_context))]
    repository: Repository,
    #[validate(custom(function = "validate_sender", use_context))]
    sender: Sender,
    #[validate(nested)]
    workflow_job: WorkflowJob,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
enum Event {
    Completed,
    InProgress,
    Queued,
    Waiting,
}
#[derive(Debug, Serialize, Deserialize, Validate)]
struct Repository {
    name: String,
}
#[derive(Debug, Serialize, Deserialize, Validate)]
struct Sender {
    login: String,
}
#[derive(Debug, Deserialize, Validate)]
struct WorkflowJob {
    _workflow_name: String,
    _job_id: u32,
    _name: String,
    #[serde(deserialize_with = "parse_labels")]
    #[validate(nested)]
    labels: RunnerSpec,
}
impl IRunnerSpec for WebhookPayload {
    fn runner_spec(&self) -> &RunnerSpec {
        &self.workflow_job.labels
    }
}
pub(super) fn parse_labels<'de, D>(deserializer: D) -> Result<RunnerSpec, D::Error>
where
    D: Deserializer<'de>,
{
    let labels: Vec<String> = Vec::deserialize(deserializer)?;

    // We expect a fixed number of labels in the array (4 in this case)
    if labels.len() != 6 {
        return Err(serde::de::Error::invalid_length(
            labels.len(),
            &"6 labels expected, [self-hosted, image, os, arch, cpu_mhz, memory_mb]",
        ));
    }

    let image = labels[1].clone();
    let os = labels[2]
        .parse::<Os>()
        .map_err(|_| serde::de::Error::unknown_variant(labels[2].as_str(), &Os::VARIANTS))?;

    let arch = labels[3]
        .parse::<Arch>()
        .map_err(|_| serde::de::Error::unknown_variant(labels[3].as_str(), &Arch::VARIANTS))?;

    let cpu_mhz: usize = labels[4].parse().map_err(|_| {
        serde::de::Error::custom(format!("Invalid value for cpu_mhz: {}", labels[4]))
    })?;

    // Assume memory is fixed for this example, adjust as needed
    let memory_mb = labels[5].parse().map_err(|_| {
        serde::de::Error::custom(format!("Invalid value for memory_mb: {}", labels[5]))
    })?;

    Ok(RunnerSpec {
        _image: image,
        os,
        arch,
        _cpu_mhz: cpu_mhz,
        _memory_mb: memory_mb,
    })
}
fn validate_repository(repo: &Repository, context: &Config) -> Result<(), ValidationError> {
    if !context.devop.allowed_repositories.contains(&repo.name) {
        return Err(ValidationError::new("Repository not allowed"));
    }
    Ok(())
}
fn validate_sender(sender: &Sender, context: &Config) -> Result<(), ValidationError> {
    if !context.devop.allowed_users.contains(&sender.login) {
        return Err(ValidationError::new("Repository not allowed"));
    }
    Ok(())
}

fn validate_event(event: &Event) -> Result<(), ValidationError> {
    match event {
        Event::Queued => Ok(()),
        _ => Err(ValidationError::new("Unsupported event")),
    }
}
