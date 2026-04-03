use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};

use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;
use std::sync::Arc;

use crate::config::Config;
use crate::payload::WebhookPayload;

pub struct AppState {
    pub config: Config,
}

fn verify_signature(
    payload_body: &str,
    secret_token: &str,
    signature_header: &str,
) -> Result<(), StatusCode> {
    let mut mac = Hmac::<Sha256>::new_from_slice(secret_token.as_bytes())
        .map_err(|e| StatusCode::INTERNAL_SERVER_ERROR)?;

    mac.update(payload_body.as_bytes());

    let expected_signature = format!("sha256={}", hex::encode(mac.finalize().into_bytes()));

    if !constant_time_eq(expected_signature.as_bytes(), signature_header.as_bytes()) {
        return Err(StatusCode::FORBIDDEN);
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
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    body: String,
) -> Result<&'static str, StatusCode> {
    let signature_header = headers
        .get("X-Hub-Signature-256")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("");

    let secret_token = &state.config.webhook_secret;
    verify_signature(&body, secret_token, signature_header)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let webhook_payload: WebhookPayload =
        serde_json::from_str(&body).map_err(|_| StatusCode::BAD_REQUEST)?;

    if webhook_payload.event != "queue" {
        clerk::info!(
            "Skipping webhook for non-queue event: {}",
            webhook_payload.event
        );
        return Ok("ok");
    }

    clerk::info!("Webhook processed successfully");
    Ok("ok")
}
