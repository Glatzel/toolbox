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
    let runner_spec = match state.config.devop.vender {
        Vendor::Github => github::WebhookPayload::runner_spec(&headers, &body, &state.config)?,
        _ => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unsupported vendor".to_string(),
            )
                .into_response());
        }
    };
    Ok(state.client.dispatch(&runner_spec, &state.config).await)
}
