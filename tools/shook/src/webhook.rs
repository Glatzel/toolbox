use std::sync::Arc;

use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use kioyu::job::{Job, ResourceRequest};
use validator::ValidateArgs;

use crate::config::{Config, Vendor};
use crate::payload::{IRunnerSpec, RunnerSpec, github};

pub struct AppContext {
    pub config: Config,
    pub kioyu_handle: kioyu::dispatcher::DispatcherHandle<RunnerSpec>,
}

pub async fn webhook(
    State(state): State<Arc<AppContext>>,
    headers: HeaderMap,
    body: String,
) -> Response {
    clerk::debug!(vendor = ?state.config.devop.vendor, "Received webhook request");

    let runner_spec = match state.config.devop.vendor {
        Vendor::Github => {
            clerk::debug!("Dispatching to GitHub webhook parser");
            github::WebhookPayload::runner_spec(&headers, &body, &state.config)
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
    let runner_spec = match runner_spec {
        Ok(spec) => spec,
        Err(response) => return response,
    };
    match runner_spec.validate_with_args(&state.config) {
        Ok(_) => {
            clerk::debug!("Validated runner spec");
            let runner = match state.config.runners.get(&runner_spec.runner) {
                Some(runner) => runner.clone(),
                None => {
                    clerk::error!(runner = ?runner_spec.runner, "Runner not found");
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Runner not found".to_string(),
                    )
                        .into_response();
                }
            };
            let resource_request = ResourceRequest::new(vec![("memory_mb", runner.memory)]);
            match state
                .kioyu_handle
                .submit(Job::new(
                    runner_spec.job.clone(),
                    runner_spec,
                    resource_request,
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
