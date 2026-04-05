use axum::{http::HeaderMap, response::Response};
use serde::{Deserialize, Serialize};

use crate::config::Config;

pub mod github;

pub trait IRunnerSpec {
    fn runner_spec(
        headers: &HeaderMap,
        body: &str,
        config: &Config,
    ) -> Result<RunnerSpec, Response>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunnerSpec {
    pub owner: String,
    pub repo: String,
    pub image: String,
    pub platform: Platform,
    pub cpu_mhz: usize,
    pub memory_mb: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Platform {
    #[serde(rename = "win-64")]
    Win64,
    #[serde(rename = "linux-64")]
    Linux64,
    #[serde(rename = "linux-aarch64")]
    LinuxAarch64,
    #[serde(rename = "osx-arm64")]
    OsxArm64,
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Platform::Win64 => "win-64",
            Platform::Linux64 => "linux-64",
            Platform::LinuxAarch64 => "linux-aarch64",
            Platform::OsxArm64 => "osx-arm64",
        };
        write!(f, "{s}")
    }
}

impl RunnerSpec {
    pub fn nomad_body(&self) -> String {
        clerk::debug!(
            owner = %self.owner,
            repo = %self.repo,
            image = %self.image,
            platform = %self.platform,
            cpu_mhz = self.cpu_mhz,
            memory_mb = self.memory_mb,
            "Serialising RunnerSpec to Nomad body"
        );
        let body = serde_json::json!({ "Meta": self }).to_string();
        clerk::debug!(body_len = body.len(), "Nomad body serialised");
        body
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    // ── helpers ──────────────────────────────────────────────────────────────

    fn make_spec(platform: Platform) -> RunnerSpec {
        RunnerSpec {
            owner: "my-org".to_string(),
            repo: "my-repo".to_string(),
            image: "ubuntu-22.04".to_string(),
            platform,
            cpu_mhz: 3200,
            memory_mb: 7812,
        }
    }

    // ── Platform Display ─────────────────────────────────────────────────────

    #[rstest]
    #[case(Platform::Win64,        "win-64")]
    #[case(Platform::Linux64,      "linux-64")]
    #[case(Platform::LinuxAarch64, "linux-aarch64")]
    #[case(Platform::OsxArm64,     "osx-arm64")]
    fn platform_display(#[case] platform: Platform, #[case] expected: &str) {
        assert_eq!(platform.to_string(), expected);
    }

    // ── Platform serde roundtrip ──────────────────────────────────────────────

    #[rstest]
    #[case(Platform::Win64,        r#""win-64""#)]
    #[case(Platform::Linux64,      r#""linux-64""#)]
    #[case(Platform::LinuxAarch64, r#""linux-aarch64""#)]
    #[case(Platform::OsxArm64,     r#""osx-arm64""#)]
    fn platform_serialises_to_kebab(#[case] platform: Platform, #[case] expected_json: &str) {
        assert_eq!(serde_json::to_string(&platform).unwrap(), expected_json);
    }

    #[rstest]
    #[case(r#""win-64""#,        Platform::Win64)]
    #[case(r#""linux-64""#,      Platform::Linux64)]
    #[case(r#""linux-aarch64""#, Platform::LinuxAarch64)]
    #[case(r#""osx-arm64""#,     Platform::OsxArm64)]
    fn platform_deserialises_from_kebab(#[case] json: &str, #[case] expected: Platform) {
        let p: Platform = serde_json::from_str(json).unwrap();
        assert_eq!(p, expected);
    }

    #[test]
    fn platform_unknown_variant_errors() {
        let result = serde_json::from_str::<Platform>(r#""freebsd-64""#);
        assert!(result.is_err());
    }

    // ── nomad_body ────────────────────────────────────────────────────────────

    #[rstest]
    #[case(Platform::Linux64)]
    #[case(Platform::LinuxAarch64)]
    #[case(Platform::Win64)]
    #[case(Platform::OsxArm64)]
    fn nomad_body_snapshot(#[case] platform: Platform) {
        let body = make_spec(platform).nomad_body();
        // Normalise to Value for stable key ordering before snapshotting.
        let value: serde_json::Value = serde_json::from_str(&body).unwrap();
        insta::assert_json_snapshot!(format!("nomad_body_{platform}"), value);
    }

    #[test]
    fn nomad_body_is_valid_json() {
        let body = make_spec(Platform::Linux64).nomad_body();
        assert!(serde_json::from_str::<serde_json::Value>(&body).is_ok());
    }

    #[test]
    fn nomad_body_contains_meta_key() {
        let body = make_spec(Platform::Linux64).nomad_body();
        let value: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert!(value.get("Meta").is_some());
    }

    #[test]
    fn nomad_body_fields_match_spec() {
        let spec = make_spec(Platform::Linux64);
        let body = spec.nomad_body();
        let value: serde_json::Value = serde_json::from_str(&body).unwrap();
        let meta = &value["Meta"];

        assert_eq!(meta["owner"],     spec.owner.as_str());
        assert_eq!(meta["repo"],      spec.repo.as_str());
        assert_eq!(meta["image"],     spec.image.as_str());
        assert_eq!(meta["platform"],  "linux-64");
        assert_eq!(meta["cpu_mhz"],   spec.cpu_mhz);
        assert_eq!(meta["memory_mb"], spec.memory_mb);
    }

    // ── RunnerSpec serde roundtrip ────────────────────────────────────────────

    #[test]
    fn runner_spec_roundtrip() {
        let spec = make_spec(Platform::OsxArm64);
        let json = serde_json::to_string(&spec).unwrap();
        let back: RunnerSpec = serde_json::from_str(&json).unwrap();

        assert_eq!(back.owner,     spec.owner);
        assert_eq!(back.repo,      spec.repo);
        assert_eq!(back.image,     spec.image);
        assert_eq!(back.platform,  spec.platform);
        assert_eq!(back.cpu_mhz,   spec.cpu_mhz);
        assert_eq!(back.memory_mb, spec.memory_mb);
    }
}