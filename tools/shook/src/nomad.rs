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
    pub async fn dispatch(&self, _runner_spec: &RunnerSpec, config: &Config) -> Response {
        let res = self
            .client
            .post(format!("{}:{}", config.nomad.url, config.nomad.port))
            .send()
            .await
            .unwrap();
        let status = res.status();

        let body = res.bytes().await.unwrap();

        (StatusCode::from_u16(status.as_u16()).unwrap(), body).into_response()
    }
}
