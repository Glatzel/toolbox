use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};

use crate::config::{Config, Vendor};
use crate::nomad::NomadClient;
use crate::payload::{IRunnerSpec, github};

use axum::response::{IntoResponse, Response};

use std::sync::Arc;

pub struct AppContext {
    pub config: Config,
    pub client: NomadClient,
}

pub async fn webhook(
    State(state): State<Arc<AppContext>>,
    headers: HeaderMap,
    body: String,
) -> Result<Response, Response> {
    clerk::debug!(vendor = ?state.config.devop.vendor, "Received webhook request");

    let runner_spec = match state.config.devop.vendor {
        Vendor::Github => {
            clerk::debug!("Dispatching to GitHub webhook parser");
            github::WebhookPayload::runner_spec(&headers, &body, &state.config)?
        }
        unsupported => {
            clerk::error!(vendor = ?unsupported, "Unsupported vendor");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unsupported vendor".to_string(),
            )
                .into_response());
        }
    };

    clerk::info!(
        os = ?runner_spec.os,
        arch = ?runner_spec.arch,
        "Runner spec resolved, dispatching to Nomad"
    );
    Ok(state.client.dispatch(&runner_spec, &state.config).await)
}
