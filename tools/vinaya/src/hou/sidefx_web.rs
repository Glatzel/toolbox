use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use mischief::{IntoMischief, mischief};
use serde_json::json;

use crate::hou::{HoudiniBuildVersion, HoudiniPlatform, HoudiniProduct};

pub struct SideFXWeb {
    pub client: reqwest_middleware::ClientWithMiddleware,
    pub token: String,
    pub api_url: String,
}

impl SideFXWeb {
    pub async fn new(
        client_id: &str,
        client_secret: &str,
        token_url: Option<&str>,
        api_url: Option<&str>,
        timeout: Option<f32>,
        retries: Option<u8>,
    ) -> mischief::Result<Self> {
        let token_url = token_url.unwrap_or("https://www.sidefx.com/oauth2/application_token");
        let api_url = api_url.unwrap_or("https://www.sidefx.com/api/");
        let timeout = timeout.unwrap_or(5.0);
        let _retries = retries.unwrap_or(3);

        //build client
        let retry_policy =
            reqwest_retry::policies::ExponentialBackoff::builder().build_with_max_retries(3);
        let reqwest_client = reqwest::Client::builder()
            .timeout(Duration::from_secs_f32(timeout))
            .build()
            .unwrap();
        let client = reqwest_middleware::ClientBuilder::new(reqwest_client)
            .with(reqwest_retry::RetryTransientMiddleware::new_with_policy(
                retry_policy,
            ))
            .build();

        // get token
        let mut authorization = String::from("Basic ");
        authorization.push_str(
            STANDARD
                .encode(format!("{client_id}:{client_secret}"))
                .as_str(),
        );
        let response = client
            .post(token_url)
            .header(
                reqwest::header::HeaderName::from_str("Authorization").unwrap(),
                reqwest::header::HeaderValue::from_str(authorization.as_str()).unwrap(),
            )
            .send()
            .await
            .into_mischief()?
            .error_for_status()
            .map_err(|e| {
                mischief::mischief!(
                    "{}. Fail to get houdini api token.",
                    e,
                    help = "Check your client_id and client_token."
                )
            })?;

        let token = response
            .json::<HashMap<String, serde_json::Value>>()
            .await
            .into_mischief()?["access_token"]
            .to_string()
            .replace("\"", "");

        let sidefx_web = SideFXWeb {
            client,
            token: format!("Bearer {}", token),
            api_url: api_url.to_string(),
        };
        Ok(sidefx_web)
    }

    pub async fn download_get_daily_builds_list(
        &self,
        product: HoudiniProduct,
        major: u16,
        minor: u16,
        platform: HoudiniPlatform,
        only_production: bool,
    ) -> mischief::Result<reqwest::Response> {
        let version = format!("{major}.{minor}").parse::<f32>().into_mischief()?;
        let data = json!(
                [
                    "download.get_daily_builds_list",
                    [product.as_ref()],
                    {"version": version.to_string(),
                     "platform": platform.as_ref(),
                      "only_production": only_production
                    },
                ]
        );
        let response = self
            .client
            .post(self.api_url.as_str())
            .header(
                reqwest::header::CONTENT_TYPE,
                reqwest::header::HeaderValue::from_static("application/x-www-form-urlencoded"),
            )
            .body(format!("json={}", data))
            .header(
                reqwest::header::HeaderName::from_static("Authorization"),
                reqwest::header::HeaderValue::from_str(&self.token).unwrap(),
            )
            .send()
            .await
            .into_mischief()?
            .error_for_status()
            .into_mischief()
            .map_err(|_| mischief!("Fail to get daily_builds_list."))?;
        Ok(response)
    }
    pub async fn download_get_daily_build_download(
        &self,
        product: HoudiniProduct,
        major: u16,
        minor: u16,
        build: HoudiniBuildVersion,
        platform: &HoudiniPlatform,
    ) -> mischief::Result<reqwest::Response> {
        let version = format!("{major}.{minor}").parse::<f32>().unwrap();
        let build = match build {
            HoudiniBuildVersion::Number(num) => num.to_string(),
            HoudiniBuildVersion::Production => "production".to_string(),
        };
        let data = json!([
            "download.get_daily_build_download",
            [product.as_ref(), version, build, platform.as_ref()],
            {}
        ]);
        let response = self
            .client
            .post(self.api_url.as_str())
            .header(
                reqwest::header::CONTENT_TYPE,
                reqwest::header::HeaderValue::from_static("application/x-www-form-urlencoded"),
            )
            .body(format!("json={}", data))
            .header(
                reqwest::header::HeaderName::from_static("Authorization"),
                reqwest::header::HeaderValue::from_str(&self.token).unwrap(),
            )
            .send()
            .await
            .into_mischief()?
            .error_for_status()
            .into_mischief()
            .map_err(|_| mischief!("Fail to get daily_builds_list."))?;
        Ok(response)
    }
}
