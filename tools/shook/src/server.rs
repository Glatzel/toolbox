mod error;
mod job;
mod vendor;
use std::sync::Arc;

use axum::Router;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use error::Error;
pub use job::{IJobSpec, JobSpec};
use kioyu::{DispatcherHandle, Job, ResourceRequest};
use validator::ValidateArgs;
use vendor::*;

use crate::config::{Config, Vendor};

pub struct AppContext {
    pub config: Config,
    pub kioyu_handle: DispatcherHandle<JobSpec>,
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

    let job_spec = match state.config.devop.vendor {
        Vendor::Github => {
            clerk::debug!("Dispatching to GitHub webhook parser");
            match github::WebhookPayload::job_spec(&headers, &body, &state.config) {
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
    clerk::debug!("{:?}", job_spec);
    match job_spec.validate_with_args(&state.config) {
        Ok(_) => {
            clerk::debug!("Validated runner spec");
            let resource_request =
                ResourceRequest::new(vec![("memory", job_spec.runner_spec.memory as usize)]);
            match state
                .kioyu_handle
                .submit(Job::new(job_spec.job.clone(), job_spec, resource_request))
                .await
            {
                Ok(_) => (StatusCode::OK, "OK".to_string()).into_response(),
                Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
            }
        }
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}
