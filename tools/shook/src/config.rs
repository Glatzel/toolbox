use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Vendor {
    #[serde(rename = "bitbucket", alias = "atlassian")]
    Bitbucket,
    Forgejo,
    Gitea,
    Github,
    Gitlab,
    Woodpecker,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevOpConfig {
    pub vendor: Vendor, // fixed typo: vender → vendor
    pub token: String,
    pub webhook_secret: String,
    pub allowed_repositories: Vec<String>,
    pub allowed_users: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NomadConfig {
    pub url: String,
    pub timeout_sec: f32,
    pub retry: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_server_config")]
    pub server: ServerConfig,
    pub devop: DevOpConfig,
    #[serde(default = "default_nomad_config")]
    pub nomad: NomadConfig,
}

impl Config {
    pub fn load_toml(path: &Path) -> mischief::Result<Self> {
        clerk::debug!(path = %path.display(), "Loading config file");

        let content = std::fs::read_to_string(path).inspect_err(|e| {
            clerk::error!(path = %path.display(), error = %e, "Failed to read config file");
        })?;

        let config: Self = toml::from_str(&content).inspect_err(|e| {
            clerk::error!(path = %path.display(), error = %e, "Failed to parse config TOML");
        })?;

        clerk::info!(
            path = %path.display(),
            port = config.server.port,
            vendor = ?config.devop.vendor,
            nomad_url = %config.nomad.url,
            allowed_repositories = config.devop.allowed_repositories.len(),
            allowed_users = config.devop.allowed_users.len(),
            "Config loaded successfully"
        );

        Ok(config)
    }
}

fn default_nomad_config() -> NomadConfig {
    NomadConfig {
        url: String::from("http://localhost:4646"),
        timeout_sec: 3.0,
        retry: 3,
    }
}

fn default_server_config() -> ServerConfig {
    ServerConfig { port: 8787 }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    // ── helpers ─────────────────────────────────────────────────────────────

    fn write_toml(content: &str) -> NamedTempFile {
        let mut f = NamedTempFile::new().unwrap();
        f.write_all(content.as_bytes()).unwrap();
        f
    }

    const MINIMAL_TOML: &str = r#"
        [devop]
        vendor   = "github"
        token    = "ghp_test"
        webhook_secret     = "s3cr3t"
        allowed_repositories = ["my-org/my-repo"]
        allowed_users        = ["alice"]
    "#;

    const FULL_TOML: &str = r#"
        [server]
        port = 9090

        [devop]
        vendor   = "gitlab"
        token    = "glpat-test"
        webhook_secret     = "another-secret"
        allowed_repositories = ["org/repo-a", "org/repo-b"]
        allowed_users        = ["bob", "carol"]

        [nomad]
        url         = "http://nomad.internal:4646"
        timeout_sec = 5.0
        retry       = 5
    "#;

    // ── snapshot: happy paths ────────────────────────────────────────────────

    #[test]
    fn load_minimal_uses_defaults() {
        let f = write_toml(MINIMAL_TOML);
        let cfg = Config::load_toml(f.path()).expect("should parse");
        insta::assert_debug_snapshot!(cfg);
    }

    #[test]
    fn load_full_config() {
        let f = write_toml(FULL_TOML);
        let cfg = Config::load_toml(f.path()).expect("should parse");
        insta::assert_debug_snapshot!(cfg);
    }

    // ── defaults ────────────────────────────────────────────────────────────

    #[test]
    fn defaults_applied_when_sections_absent() {
        let f = write_toml(MINIMAL_TOML);
        let cfg = Config::load_toml(f.path()).unwrap();

        assert_eq!(cfg.server.port, 8787);
        assert_eq!(cfg.nomad.url, "http://localhost:4646");
        assert!((cfg.nomad.timeout_sec - 3.0).abs() < f32::EPSILON);
        assert_eq!(cfg.nomad.retry, 3);
    }

    #[test]
    fn explicit_values_override_defaults() {
        let f = write_toml(FULL_TOML);
        let cfg = Config::load_toml(f.path()).unwrap();

        assert_eq!(cfg.server.port, 9090);
        assert_eq!(cfg.nomad.url, "http://nomad.internal:4646");
        assert!((cfg.nomad.timeout_sec - 5.0).abs() < f32::EPSILON);
        assert_eq!(cfg.nomad.retry, 5);
    }

    // ── vendor deserialization ───────────────────────────────────────────────

    #[rstest]
    #[case("github", "github")]
    #[case("gitlab", "gitlab")]
    #[case("bitbucket", "bitbucket")]
    #[case("atlassian", "bitbucket")] // alias
    #[case("forgejo", "forgejo")]
    #[case("gitea", "gitea")]
    #[case("woodpecker", "woodpecker")]
    fn vendor_roundtrip(#[case] toml_value: &str, #[case] expected_debug: &str) {
        let toml = format!(
            r#"
            [devop]
            vendor = "{toml_value}"
            token  = "t"
            webhook_secret = "s"
            allowed_repositories = []
            allowed_users = []
            "#
        );
        let f = write_toml(&toml);
        let cfg = Config::load_toml(f.path()).unwrap();
        assert_eq!(
            format!("{:?}", cfg.devop.vendor).to_lowercase(),
            expected_debug
        );
    }

    #[test]
    fn unknown_vendor_returns_error() {
        let toml = r#"
            [devop]
            vendor = "perforce"
            token  = "t"
            webhook_secret = "s"
            allowed_repositories = []
            allowed_users = []
        "#;
        let f = write_toml(toml);
        let result = Config::load_toml(f.path());
        assert!(result.is_err());
    }

    // ── error paths ──────────────────────────────────────────────────────────

    #[test]
    fn missing_file_returns_error() {
        let result = Config::load_toml(Path::new("/nonexistent/path/config.toml"));
        assert!(result.is_err());
    }

    #[test]
    fn malformed_toml_returns_error() {
        let f = write_toml("this is not [ valid toml !!!");
        let result = Config::load_toml(f.path());
        assert!(result.is_err());
    }

    #[test]
    fn missing_required_field_returns_error() {
        // `token` is required and absent
        let f = write_toml(
            r#"
            [devop]
            vendor = "github"
            webhook_secret = "s"
            allowed_repositories = []
            allowed_users = []
        "#,
        );
        let result = Config::load_toml(f.path());
        assert!(result.is_err());
    }

    // ── allowed lists ────────────────────────────────────────────────────────

    #[rstest]
    #[case(r#"["a","b","c"]"#, 3)]
    #[case(r#"[]"#, 0)]
    #[case(r#"["single"]"#, 1)]
    fn allowed_repositories_length(#[case] list: &str, #[case] expected: usize) {
        let toml = format!(
            r#"
            [devop]
            vendor = "github"
            token  = "t"
            webhook_secret = "s"
            allowed_repositories = {list}
            allowed_users = []
            "#
        );
        let f = write_toml(&toml);
        let cfg = Config::load_toml(f.path()).unwrap();
        assert_eq!(cfg.devop.allowed_repositories.len(), expected);
    }
}
