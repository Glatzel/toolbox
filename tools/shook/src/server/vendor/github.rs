use axum::http::HeaderMap;
use hmac::{Hmac, KeyInit, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use validator::{Validate, ValidateArgs, ValidationError};

use crate::config::{Config, ConfigRunner};
use crate::server::IRunnerPayload;
use crate::server::error::ShookServerError;
use crate::utils::constant_time_eq;
use crate::vm::RunnerPayload;

#[derive(Debug, Deserialize, Validate)]
#[validate(context = Config)]
pub struct WebhookPayload {
    #[validate(custom(function = "validate_event"))]
    action: Event,

    repository: Repository,
    #[validate(custom(function = "validate_sender", use_context))]
    sender: Sender,
    workflow_job: WorkflowJob,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Event {
    Completed,
    InProgress,
    Queued,
    Waiting,
}

#[derive(Debug, Serialize, Deserialize)]
struct Repository {
    name: String,
    owner: Owner,
}

#[derive(Debug, Serialize, Deserialize)]
struct Owner {
    login: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Sender {
    login: String,
}

#[derive(Debug, Deserialize)]
struct WorkflowJob {
    #[serde(rename = "workflow_name")]
    _workflow_name: String,
    name: String,
    id: usize,
    labels: Vec<String>,
}

impl IRunnerPayload for WebhookPayload {
    fn runner_payload(
        headers: &HeaderMap,
        body: &str,
        config: &Config,
    ) -> Result<RunnerPayload, ShookServerError> {
        clerk::debug!("Verifying webhook signature");
        verify_signature(body, &config.devop.webhook_secret, headers)?;

        clerk::debug!("Parsing webhook JSON payload");
        let webhook_payload: WebhookPayload = serde_json::from_str(body).map_err(|e| {
            clerk::warn!(error = %e, "Failed to parse webhook JSON payload");
            ShookServerError::SerdeJson(e)
        })?;

        clerk::debug!(
            repository = %webhook_payload.repository.name,
            sender = %webhook_payload.sender.login,
            "Validating webhook payload"
        );
        webhook_payload.validate_with_args(config).map_err(|e| {
            clerk::warn!(error = %e, "Webhook payload validation failed");
            ShookServerError::Validator(e)
        })?;

        clerk::info!(
            repository = %webhook_payload.repository.name,
            sender = %webhook_payload.sender.login,
            "Webhook payload accepted, returning runner spec"
        );
        let runner_name = match webhook_payload.workflow_job.labels.get(1) {
            Some(name) => name,
            None => {
                return Err(ShookServerError::Parse(format!(
                    "Runner label not found: {:?}",
                    webhook_payload.workflow_job.labels
                )));
            }
        };
        let runner: ConfigRunner = match config.runners.get(runner_name) {
            Some(r) => r.clone(),
            None => {
                return Err(ShookServerError::Parse(format!(
                    "Runner not found: {}",
                    runner_name
                )));
            }
        };
        let job_spec = RunnerPayload::new(
            format!(
                "{}-{}-{}-{}",
                webhook_payload.repository.owner.login,
                webhook_payload.repository.name,
                webhook_payload.workflow_job.name,
                webhook_payload.workflow_job.id
            ),
            runner.image,
            runner.cpus,
            runner.memory,
            runner.volumes,
            runner.ports,
            runner.envs,
            runner.secrets,
            webhook_payload.repository.owner.login,
            webhook_payload.repository.name,
            config.devop.token.clone(),
        );
        Ok(job_spec)
    }
}

fn validate_sender(sender: &Sender, context: &Config) -> Result<(), ValidationError> {
    if !context.devop.allowed_users.contains(&sender.login) {
        clerk::warn!(sender = %sender.login, "Sender not in allowlist");
        return Err(ValidationError::new("Sender not allowed"));
    }
    clerk::debug!(sender = %sender.login, "Sender validated");
    Ok(())
}

fn validate_event(event: &Event) -> Result<(), ValidationError> {
    match event {
        Event::Queued => {
            clerk::debug!("Event validated as Queued");
            Ok(())
        }
        other => {
            clerk::info!(event = ?other, "Unsupported event type");
            Err(ValidationError::new("Unsupported event"))
        }
    }
}

fn verify_signature(
    payload_body: &str,
    secret_token: &str,
    headers: &HeaderMap,
) -> Result<(), ShookServerError> {
    let signature_header = headers
        .get("X-Hub-Signature-256")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("");

    if signature_header.is_empty() {
        clerk::warn!("Missing X-Hub-Signature-256 header");
    }

    let mut mac = Hmac::<Sha256>::new_from_slice(secret_token.as_bytes()).map_err(|e| {
        clerk::error!(error = %e, "Failed to initialise HMAC with secret token");
        ShookServerError::MissingHeader("X-Hub-Signature-256".to_string())
    })?;

    mac.update(payload_body.as_bytes());
    let expected_signature = format!("sha256={}", hex::encode(mac.finalize().into_bytes()));

    if !constant_time_eq(expected_signature.as_bytes(), signature_header.as_bytes()) {
        clerk::error!("Webhook signature mismatch — request rejected");
        return Err(ShookServerError::RequestSignaturesMismatch);
    }

    clerk::debug!("Webhook signature verified successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use axum::http::HeaderMap;
    use hmac::{Hmac, Mac};
    use rstest::*;
    use sha2::Sha256;

    use super::*;

    fn make_signature(secret: &str, body: &str) -> String {
        clerk::init_log_with_level(clerk::LevelFilter::TRACE);
        let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(body.as_bytes());
        format!("sha256={}", hex::encode(mac.finalize().into_bytes()))
    }

    fn headers_with_sig(sig: &str) -> HeaderMap {
        let mut h = HeaderMap::new();
        h.insert("X-Hub-Signature-256", sig.parse().unwrap());
        h
    }

    const SECRET: &str = "test-secret";

    #[rstest]
    #[case("hello world")]
    #[case(r#"{"event":"Queued"}"#)]
    #[case("")]
    fn verify_signature_valid(#[case] body: &str) {
        clerk::init_log_with_level(clerk::LevelFilter::TRACE);
        let sig = make_signature(SECRET, body);
        let headers = headers_with_sig(&sig);
        assert!(verify_signature(body, SECRET, &headers).is_ok());
    }

    #[rstest]
    #[case("wrong-secret")]
    #[case("")]
    fn verify_signature_bad_secret(#[case] bad_secret: &str) {
        clerk::init_log_with_level(clerk::LevelFilter::TRACE);
        let body = "some payload";
        let sig = make_signature(bad_secret, body);
        let headers = headers_with_sig(&sig);
        // Only actually wrong when the secret differs from SECRET
        if bad_secret != SECRET {
            assert!(verify_signature(body, SECRET, &headers).is_err());
        }
    }

    #[test]
    fn verify_signature_missing_header() {
        clerk::init_log_with_level(clerk::LevelFilter::TRACE);
        let headers = HeaderMap::new(); // no sig header
        let result = verify_signature("body", SECRET, &headers);
        assert!(result.is_err());
    }

    #[test]
    fn verify_signature_tampered_body() {
        clerk::init_log_with_level(clerk::LevelFilter::TRACE);
        let original = "original body";
        let sig = make_signature(SECRET, original);
        let headers = headers_with_sig(&sig);
        let result = verify_signature("tampered body", SECRET, &headers);
        assert!(result.is_err());
    }
}
