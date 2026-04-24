mod error;
mod job;
mod vendor;
use std::sync::Arc;

use axum::Router;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use error::ShookServerError;
pub use job::IRunnerPayload;
use kioyu::{DispatcherHandle, Job, ResourceRequest};
use validator::ValidateArgs;
use vendor::*;

use crate::config::{Config, Vendor};
use crate::vm::RunnerPayload;

pub struct AppContext {
    pub config: Config,
    pub kioyu_handle: DispatcherHandle<RunnerPayload>,
}
fn app(shared_state: Arc<AppContext>) -> Router {
    Router::new()
        .route("/webhook", post(webhook))
        .with_state(shared_state)
}
pub async fn start_server(shared_state: Arc<AppContext>) -> mischief::Result<()> {
    let app = app(shared_state.clone());
    let addr = format!("0.0.0.0:{}", shared_state.config.server.port);
    clerk::info!(address = %addr, "Binding listener");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .inspect_err(|e| {
            clerk::error!(address = %addr, error = %e, "Failed to bind TCP listener");
        })?;
    clerk::info!(address = %addr, "Server started, waiting for connections");
    axum::serve(listener, app).await.inspect_err(|e| {
        clerk::error!(error = %e, "Server exited with error");
    })?;
    Ok(())
}
async fn webhook(
    State(state): State<Arc<AppContext>>,
    headers: HeaderMap,
    body: String,
) -> Response {
    clerk::debug!(vendor = ?state.config.devop.vendor, "Received webhook request");

    let runner_payload = match state.config.devop.vendor {
        Vendor::Github => {
            clerk::debug!("Dispatching to GitHub webhook parser");
            match github::WebhookPayload::runner_payload(&headers, &body, &state.config) {
                Ok(spec) => spec,
                Err(response) => return response.into_response(),
            }
        }
        unsupported => {
            clerk::error!(vendor = ?unsupported, "Unsupported vendor");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unsupported vendor".to_string(),
            )
                .into_response();
        }
    };
    clerk::debug!("{:?}", runner_payload);
    match runner_payload.validate_with_args(&state.config) {
        Ok(_) => {
            clerk::debug!("Validated runner spec");
            let resource_request =
                ResourceRequest::new(vec![("memory", runner_payload.memory as usize)]);
            match state
                .kioyu_handle
                .submit(Job::new(
                    runner_payload.sandbox_name.clone(),
                    runner_payload,
                    resource_request,
                    state.config.kioyu.max_retries,
                ))
                .await
            {
                Ok(_) => (StatusCode::OK, "OK".to_string()).into_response(),
                Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
            }
        }
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}
