use std::time::Duration;

use axum::response::{IntoResponse, Response};
use reqwest::StatusCode;

use crate::{
    config::{Config, NomadConfig},
    payload::RunnerSpec,
};

pub struct NomadClient {
    client: reqwest_middleware::ClientWithMiddleware,
}
impl NomadClient {
    pub fn new(config: &NomadConfig) -> mischief::Result<Self> {
        let retry_policy = reqwest_retry::policies::ExponentialBackoff::builder()
            .build_with_max_retries(config.retry);
        let reqwest_client = reqwest::Client::builder()
            .timeout(Duration::from_secs_f32(config.timeout_sec))
            .build()
            .unwrap();
        let client = reqwest_middleware::ClientBuilder::new(reqwest_client)
            .with(reqwest_retry::RetryTransientMiddleware::new_with_policy(
                retry_policy,
            ))
            .build();
        let sidefx_web = Self { client };
        Ok(sidefx_web)
    }
    pub async fn dispatch(&self, runner_spec: &RunnerSpec, config: &Config) -> Response {
        let body = serde_json::json!({"Meta":{
            "TOKEN":config.devop.token.to_string(),
            "OWNER":runner_spec.owner.to_string(),
            "REPO":runner_spec.repo.to_string(),
            "ID":runner_spec.id.to_string()
        }});
        clerk::debug!("{}", body.to_string());
        let res = match self
            .client
            .post(format!(
                "{}/v1/job/{}/dispatch",
                config.nomad.url, runner_spec.job
            ))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
        {
            Ok(res) => res,
            Err(e) => {
                clerk::error!("Nomad request failed: {e}");
                return (
                    StatusCode::BAD_GATEWAY,
                    format!("Nomad request failed: {e}"),
                )
                    .into_response();
            }
        };

        let status = StatusCode::from_u16(res.status().as_u16())
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        let body = match res.bytes().await {
            Ok(b) => b,
            Err(e) => {
                clerk::error!("Failed to read response: {e}");
                return (
                    StatusCode::BAD_GATEWAY,
                    format!("Failed to read response: {e}"),
                )
                    .into_response();
            }
        };

        (status, body).into_response()
    }
}
