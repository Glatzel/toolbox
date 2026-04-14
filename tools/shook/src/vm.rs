use std::fmt::{Debug, Formatter};
use std::path::PathBuf;

use async_trait::async_trait;
use hashbrown::HashMap;
use kioyu::{CancellationToken, IPayload};
use microsandbox::{ExecEvent, ExecHandle, MicrosandboxError, Sandbox};
use validator::{Validate, ValidationError};

use crate::config::Config;

#[derive(Validate)]
#[validate(context = Config)]
pub struct RunnerPayload {
    pub sandbox_name: String,
    pub image: String,
    pub cpus: u8,
    pub memory: u32,
    pub volumes: HashMap<PathBuf, PathBuf>,
    pub ports: HashMap<u16, u16>,
    pub envs: HashMap<String, String>,
    pub secrets: HashMap<String, (String, String)>,
    pub owner: String,
    #[validate(custom(function = "Self::validate_repository", use_context))]
    pub repo: String,
    pub token: String,
}

impl RunnerPayload {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        sandbox_name: String,
        image: String,
        cpus: u8,
        memory: u32,
        volumes: HashMap<PathBuf, PathBuf>,
        ports: HashMap<u16, u16>,
        envs: HashMap<String, String>,
        secrets: HashMap<String, (String, String)>,
        owner: String,
        repo: String,
        token: String,
    ) -> Self {
        Self {
            sandbox_name,
            image,
            cpus,
            memory,
            volumes,
            ports,
            envs,
            secrets,
            owner,
            repo,
            token,
        }
    }

    fn validate_repository(repo: &String, context: &Config) -> Result<(), ValidationError> {
        if !context.devop.allowed_repositories.contains(repo) {
            clerk::warn!(repo = %repo, "repository rejected (not in allowlist)");
            return Err(ValidationError::new("repository_not_allowed"));
        }

        clerk::debug!(repo = %repo, "repository validated");
        Ok(())
    }
}

impl Debug for RunnerPayload {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RunnerPayload")
            .field("sandbox_name", &self.sandbox_name)
            .field("image", &self.image)
            .field("cpus", &self.cpus)
            .field("memory", &self.memory)
            .field("volumes", &self.volumes)
            .field("ports", &self.ports)
            .field("envs", &self.envs)
            .field("secrets", &self.secrets)
            .field("owner", &self.owner)
            .field("repo", &self.repo)
            .finish()
    }
}

#[async_trait]
impl IPayload for RunnerPayload {
    type Error = mischief::Report;

    async fn execute(&self, cancel: CancellationToken) -> mischief::Result<()> {
        clerk::debug!(
            sandbox = %self.sandbox_name,
            image = %self.image,
            repo = %self.repo,
            "starting runner sandbox"
        );

        let sandbox = build_sandbox(
            &self.sandbox_name,
            &self.image,
            self.cpus,
            self.memory,
            &self.volumes,
            &self.ports,
            &self.envs,
            &self.secrets,
        )
        .await?;

        let handle = start_runner(&sandbox, &self.owner, &self.repo, &self.token).await?;

        drain_sandbox_handle(handle, &cancel).await;

        clerk::debug!(sandbox = %self.sandbox_name, "runner execution finished");

        Ok(())
    }

    async fn post_process(&self) -> mischief::Result<()> {
        let name = &self.sandbox_name;

        if Sandbox::list().await?.iter().all(|s| s.name() != name) {
            clerk::debug!(sandbox = %name, "sandbox already removed");
            return Ok(());
        }

        match Sandbox::remove(name).await {
            Ok(()) => clerk::debug!(sandbox = %name, "sandbox removed"),
            Err(e) => clerk::error!(sandbox = %name, error = %e, "sandbox removal failed"),
        }

        Ok(())
    }
}
#[allow(clippy::too_many_arguments)]
pub async fn build_sandbox(
    sandbox_name: &str,
    image: &str,
    cpus: u8,
    memory: u32,
    volumes: &HashMap<PathBuf, PathBuf>,
    ports: &HashMap<u16, u16>,
    envs: &HashMap<String, String>,
    secrets: &HashMap<String, (String, String)>,
) -> Result<Sandbox, MicrosandboxError> {
    let mut builder = Sandbox::builder(sandbox_name)
        .image(image)
        .cpus(cpus)
        .memory(memory)
        .replace()
        .entrypoint(["bash"]);

    for (host, guest) in volumes {
        builder = builder.volume(guest.to_string_lossy().as_ref(), |m| m.bind(host));
    }

    for (host, guest) in ports {
        builder = builder.port(*host, *guest);
    }

    for (key, value) in envs {
        builder = builder.env(key, value);
    }

    for (key, (value, url)) in secrets {
        builder = builder.secret(|s| s.env(key).value(value).allow_host(url));
    }

    clerk::debug!(
        sandbox = %sandbox_name,
        image = %image,
        cpus,
        memory,
        "creating sandbox"
    );

    let sandbox = builder.create_detached().await?;

    clerk::debug!(sandbox = %sandbox_name, "sandbox created");

    Ok(sandbox)
}

async fn start_runner(
    sandbox: &Sandbox,
    owner: &str,
    repo: &str,
    token: &str,
) -> Result<ExecHandle, MicrosandboxError> {
    clerk::debug!(
        sandbox = %sandbox.name(),
        owner = %owner,
        repo = %repo,
        "starting runner process"
    );

    let handle = sandbox
        .exec_stream("bash", ["./start-runner.sh", owner, repo, token])
        .await?;

    Ok(handle)
}

async fn drain_sandbox_handle(mut handle: ExecHandle, cancel: &CancellationToken) {
    loop {
        tokio::select! {
            _ = cancel.cancelled() => {
                clerk::debug!("sandbox cancellation received");
                handle.kill().await.ok();
                break;
            }

            event = handle.recv() => {
                let Some(event) = event else {
                    clerk::debug!("sandbox stream closed");
                    break;
                };

                match event {
                    ExecEvent::Stdout(data) => {
                        clerk::debug!("{}", String::from_utf8_lossy(&data));
                    }

                    ExecEvent::Stderr(data) => {
                        clerk::debug!("{}", String::from_utf8_lossy(&data));
                    }

                    ExecEvent::Exited { code } => {
                        clerk::debug!(exit_code = code, "sandbox process exited");
                        break;
                    }

                    _ => {}
                }
            }
        }
    }
}
