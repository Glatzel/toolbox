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
        clerk::debug!("Verifying webhook signature");
        verify_signature(body, &config.devop.webhook_secret, headers)?;

        clerk::debug!("Parsing webhook JSON payload");
        let webhook_payload: WebhookPayload = serde_json::from_str(body).map_err(|e| {
            clerk::warn!(error = %e, "Failed to parse webhook JSON payload");
            (StatusCode::BAD_REQUEST, "Invalid JSON payload".to_string()).into_response()
        })?;

        clerk::debug!(
            repository = %webhook_payload.repository.name,
            sender = %webhook_payload.sender.login,
            "Validating webhook payload"
        );
        webhook_payload.validate_with_args(config).map_err(|e| {
            clerk::warn!(error = %e, "Webhook payload validation failed");
            (StatusCode::BAD_REQUEST, e.to_string()).into_response()
        })?;

        clerk::info!(
            repository = %webhook_payload.repository.name,
            sender = %webhook_payload.sender.login,
            "Webhook payload accepted, returning runner spec"
        );
        Ok(webhook_payload.workflow_job.labels)
    }
}

pub(super) fn parse_labels<'de, D>(deserializer: D) -> Result<RunnerSpec, D::Error>
where
    D: Deserializer<'de>,
{
    let labels: Vec<String> = Vec::deserialize(deserializer)?;

    clerk::debug!(label_count = labels.len(), "Parsing runner labels");

    if labels.len() != 6 {
        clerk::warn!(
            label_count = labels.len(),
            "Invalid label count, expected 6 [self-hosted, image, os, arch, cpu_mhz, memory_mb]"
        );
        return Err(serde::de::Error::invalid_length(
            labels.len(),
            &"6 labels expected, [self-hosted, image, os, arch, cpu_mhz, memory_mb]",
        ));
    }

    let image = labels[1].clone();

    let os = labels[2].parse::<Os>().map_err(|_| {
        clerk::warn!(value = %labels[2], "Failed to parse OS label");
        serde::de::Error::unknown_variant(labels[2].as_str(), &Os::VARIANTS)
    })?;

    let arch = labels[3].parse::<Arch>().map_err(|_| {
        clerk::warn!(value = %labels[3], "Failed to parse Arch label");
        serde::de::Error::unknown_variant(labels[3].as_str(), &Arch::VARIANTS)
    })?;

    let cpu_mhz: usize = labels[4].parse().map_err(|_| {
        clerk::warn!(value = %labels[4], "Failed to parse cpu_mhz label");
        serde::de::Error::custom(format!("Invalid value for cpu_mhz: {}", labels[4]))
    })?;

    let memory_mb = labels[5].parse().map_err(|_| {
        clerk::warn!(value = %labels[5], "Failed to parse memory_mb label");
        serde::de::Error::custom(format!("Invalid value for memory_mb: {}", labels[5]))
    })?;

    clerk::debug!(image = %image, ?os, ?arch, cpu_mhz, memory_mb, "Runner labels parsed successfully");

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
        clerk::warn!(repository = %repo.name, "Repository not in allowlist");
        return Err(ValidationError::new("Repository not allowed"));
    }
    clerk::debug!(repository = %repo.name, "Repository validated");
    Ok(())
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
            clerk::warn!(event = ?other, "Unsupported event type");
            Err(ValidationError::new("Unsupported event"))
        }
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

    if signature_header.is_empty() {
        clerk::warn!("Missing X-Hub-Signature-256 header");
    }

    let mut mac = Hmac::<Sha256>::new_from_slice(secret_token.as_bytes()).map_err(|e| {
        clerk::error!(error = %e, "Failed to initialise HMAC with secret token");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Missing x-hub-signature-256 header",
        )
            .into_response()
    })?;

    mac.update(payload_body.as_bytes());
    let expected_signature = format!("sha256={}", hex::encode(mac.finalize().into_bytes()));

    if !constant_time_eq(expected_signature.as_bytes(), signature_header.as_bytes()) {
        clerk::warn!("Webhook signature mismatch — request rejected");
        return Err((StatusCode::FORBIDDEN, "Request signatures didn't match!").into_response());
    }

    clerk::debug!("Webhook signature verified successfully");
    Ok(())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderMap;
    use hmac::{Hmac, Mac};
    use rstest::*;
    use sha2::Sha256;

    // ── helpers ─────────────────────────────────────────────────────────────

    fn make_signature(secret: &str, body: &str) -> String {
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

    // ── verify_signature ────────────────────────────────────────────────────

    #[rstest]
    #[case("hello world")]
    #[case(r#"{"event":"Queued"}"#)]
    #[case("")]
    fn verify_signature_valid(#[case] body: &str) {
        let sig = make_signature(SECRET, body);
        let headers = headers_with_sig(&sig);
        assert!(verify_signature(body, SECRET, &headers).is_ok());
    }

    #[rstest]
    #[case("wrong-secret")]
    #[case("")]
    fn verify_signature_bad_secret(#[case] bad_secret: &str) {
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
        let headers = HeaderMap::new(); // no sig header
        let result = verify_signature("body", SECRET, &headers);
        assert!(result.is_err());
    }

    #[test]
    fn verify_signature_tampered_body() {
        let original = "original body";
        let sig = make_signature(SECRET, original);
        let headers = headers_with_sig(&sig);
        let result = verify_signature("tampered body", SECRET, &headers);
        assert!(result.is_err());
    }

    // ── parse_labels ────────────────────────────────────────────────────────

    // Helper: serialise a label vec into a JSON string then deserialise via
    // parse_labels using a raw JSON deserializer.
    fn deserialise_labels(labels: &[&str]) -> Result<RunnerSpec, String> {
        let json = serde_json::to_string(labels).unwrap();
        let mut de = serde_json::Deserializer::from_str(&json);
        parse_labels(&mut de).map_err(|e| e.to_string())
    }

    #[rstest]
    #[case(
        &["self-hosted", "ubuntu-22.04", "linux", "x86_64", "3200", "7812"],
        "linux_x86_64_snapshot"
    )]
    #[case(
        &["self-hosted", "debian-12", "linux", "arm64", "2400", "4096"],
        "linux_arm64_snapshot"
    )]
    fn parse_labels_valid_snapshot(#[case] labels: &[&str], #[case] snapshot_name: &str) {
        let result = deserialise_labels(labels);
        assert!(result.is_ok(), "expected Ok but got: {:?}", result);
        insta::assert_debug_snapshot!(snapshot_name, result.unwrap());
    }

    #[rstest]
    #[case(&["self-hosted", "ubuntu"], "too few labels")]
    #[case(
        &["self-hosted", "ubuntu", "linux", "x86_64", "3200", "7812", "extra"],
        "too many labels"
    )]
    fn parse_labels_wrong_length(#[case] labels: &[&str], #[case] _desc: &str) {
        let result = deserialise_labels(labels);
        assert!(result.is_err());
        insta::assert_snapshot!(result.unwrap_err());
    }

    #[rstest]
    #[case(&["self-hosted", "img", "beos", "x86_64", "3200", "7812"], "unknown_os")]
    #[case(&["self-hosted", "img", "linux", "pdp11", "3200", "7812"], "unknown_arch")]
    fn parse_labels_invalid_variant(#[case] labels: &[&str], #[case] snapshot_name: &str) {
        let result = deserialise_labels(labels);
        assert!(result.is_err());
        insta::assert_snapshot!(snapshot_name, result.unwrap_err());
    }

    #[rstest]
    #[case(&["self-hosted", "img", "linux", "x86_64", "not-a-number", "7812"], "bad_cpu_mhz")]
    #[case(&["self-hosted", "img", "linux", "x86_64", "3200", "not-a-number"], "bad_memory_mb")]
    fn parse_labels_invalid_numbers(#[case] labels: &[&str], #[case] snapshot_name: &str) {
        let result = deserialise_labels(labels);
        assert!(result.is_err());
        insta::assert_snapshot!(snapshot_name, result.unwrap_err());
    }

    // ── validate_event ──────────────────────────────────────────────────────

    #[rstest]
    #[case(Event::Queued, true)]
    #[case(Event::Completed, false)]
    #[case(Event::InProgress, false)]
    #[case(Event::Waiting, false)]
    fn validate_event_cases(#[case] event: Event, #[case] should_pass: bool) {
        let result = validate_event(&event);
        assert_eq!(result.is_ok(), should_pass, "event={event:?}");
    }

    // ── validate_repository / validate_sender ───────────────────────────────

    fn make_config(repos: &[&str], users: &[&str]) -> Config {
        Config {
            server: crate::config::ServerConfig { port: 8787 },
            devop: crate::config::DevOpConfig {
                vendor: crate::config::Vendor::Github,
                token: "test-token".to_string(),
                webhook_secret: SECRET.to_string(),
                allowed_repositories: repos.iter().map(|s| s.to_string()).collect(),
                allowed_users: users.iter().map(|s| s.to_string()).collect(),
            },
            nomad: crate::config::NomadConfig {
                url: "http://localhost:4646/v1/".to_string(),
                timeout_sec: 3.0,
                retry: 3,
            },
        }
    }

    #[rstest]
    #[case("allowed-repo", true)]
    #[case("unknown-repo", false)]
    fn validate_repository_cases(#[case] repo_name: &str, #[case] should_pass: bool) {
        let config = make_config(&["allowed-repo"], &[]);
        let repo = Repository {
            name: repo_name.to_string(),
        };
        let result = validate_repository(&repo, &config);
        assert_eq!(result.is_ok(), should_pass);
    }

    #[rstest]
    #[case("alice", true)]
    #[case("eve", false)]
    fn validate_sender_cases(#[case] login: &str, #[case] should_pass: bool) {
        let config = make_config(&[], &["alice"]);
        let sender = Sender {
            login: login.to_string(),
        };
        let result = validate_sender(&sender, &config);
        assert_eq!(result.is_ok(), should_pass);
    }
}
