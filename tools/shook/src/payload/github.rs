use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use hmac::{Hmac, KeyInit, Mac};
use serde::{Deserialize, Deserializer, Serialize};
use sha2::Sha256;
use strum::VariantNames;
use validator::{Validate, ValidateArgs, ValidationError};

use crate::{
    config::Config,
    payload::{Arch, IRunnerSpec, Os, RunnerSpec},
    utils::constant_time_eq,
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
    fn runner_spec(
        headers: &HeaderMap,
        body: &str,
        config: &Config,
    ) -> Result<RunnerSpec, Response> {
        verify_signature(&body, &config.devop.webhook_secret, headers)?;
        let webhook_payload: WebhookPayload = serde_json::from_str(&body).map_err(|_| {
            (StatusCode::BAD_REQUEST, "Invalid JSON payload".to_string()).into_response()
        })?;

        webhook_payload
            .validate_with_args(&config)
            .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()).into_response())?;
        Ok(webhook_payload.workflow_job.labels)
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
fn verify_signature(
    payload_body: &str,
    secret_token: &str,
    headers: &HeaderMap,
) -> Result<(), Response> {
    let signature_header = headers
        .get("X-Hub-Signature-256")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("");
    let mut mac = Hmac::<Sha256>::new_from_slice(secret_token.as_bytes()).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Missing x-hub-signature-256 header",
        )
            .into_response()
    })?;

    mac.update(payload_body.as_bytes());

    let expected_signature = format!("sha256={}", hex::encode(mac.finalize().into_bytes()));

    if !constant_time_eq(expected_signature.as_bytes(), signature_header.as_bytes()) {
        return Err((StatusCode::FORBIDDEN, "Request signatures didn't match!").into_response());
    }

    Ok(())
}
