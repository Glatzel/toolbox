use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};

use crate::config::Config;
use crate::nomad::NomadClient;
use crate::payload::IRunnerSpec;
use crate::payload::WebhookPayload;
use axum::response::{IntoResponse, Response};
use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;
use std::sync::Arc;
use validator::ValidateArgs;

pub struct AppContext {
    pub config: Config,
    pub client: NomadClient,
}

fn verify_signature(
    payload_body: &str,
    secret_token: &str,
    signature_header: &str,
) -> Result<(), Response> {
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
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter()
        .zip(b.iter())
        .fold(0u8, |acc, (x, y)| acc | (x ^ y))
        == 0
}
pub async fn webhook(
    State(state): State<Arc<AppContext>>,
    headers: HeaderMap,
    body: String,
) -> Result<Response, Response> {
    let signature_header = headers
        .get("X-Hub-Signature-256")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("");

    let secret_token = &state.config.devop.webhook_secret;
    verify_signature(&body, secret_token, signature_header)?;

    let webhook_payload: WebhookPayload = serde_json::from_str(&body).map_err(|_| {
        (StatusCode::BAD_REQUEST, "Invalid JSON payload".to_string()).into_response()
    })?;

    webhook_payload
        .validate_with_args(&state.config)
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()).into_response())?;
    clerk::debug!("Webhook: {:?}", webhook_payload);
    clerk::debug!("Runner Specification: {:?}", webhook_payload.runner_spec());
    Ok(state
        .client
        .dispatch(webhook_payload.runner_spec(), &state.config)
        .await)
}
