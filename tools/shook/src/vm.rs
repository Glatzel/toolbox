use std::fmt::{Debug, Formatter};
use std::path::PathBuf;

use async_trait::async_trait;
use hashbrown::HashMap;
use kioyu::{CancellationToken, IPayload};
use microsandbox::{ExecEvent, ExecHandle, MicrosandboxError, Sandbox};
use tokio::sync::OnceCell;
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
    sandbox: OnceCell<Sandbox>,
}

impl RunnerPayload {
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
            sandbox: OnceCell::new(),
        }
    }
    fn validate_repository(repo: &String, context: &Config) -> Result<(), ValidationError> {
        if !context.devop.allowed_repositories.contains(repo) {
            clerk::warn!(repository = %repo, "Repository not in allowlist");
            return Err(ValidationError::new("Repository not allowed"));
        }
        clerk::debug!(repository = %repo, "Repository validated");
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
            .field("token", &self.token)
            .finish()
    }
}
#[async_trait]
impl IPayload for RunnerPayload {
    type Error = mischief::Report;
    async fn execute(&self, cancel: CancellationToken) -> mischief::Result<()> {
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
        self.sandbox.set(sandbox).ok();
        drain_sandbox_handle(handle, &cancel).await;
        Ok(())
    }
    async fn post_process(&self) {
        if let Some(sandbox) = self.sandbox.get() {
            stop_and_remove_sandbox(sandbox).await;
        } else {
            clerk::warn!(
                "runner '{}' has no sandbox to clean up (build may have failed)",
                self.sandbox_name
            );
        }
    }
}
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

    for (host, guest) in volumes.iter() {
        builder = builder.volume(guest.to_string_lossy().as_ref(), |m| m.bind(host));
    }
    for (host, guest) in ports.iter() {
        builder = builder.port(*host, *guest);
    }
    for (key, value) in envs.iter() {
        builder = builder.env(key, value);
    }
    for (key, (value, url)) in secrets.iter() {
        builder = builder.secret(|s| s.env(key).value(value).allow_host(url));
    }

    clerk::debug!("Sandbox builder configured: {sandbox_name}");
    let sandbox = builder.create().await?;
    clerk::debug!("Sandbox created: {sandbox_name}");

    Ok(sandbox)
}
async fn start_runner(
    sandbox: &Sandbox,
    owner: &str,
    repo: &str,
    token: &str,
) -> Result<ExecHandle, MicrosandboxError> {
    let handle = sandbox
        .exec_stream("bash", ["./start-runner.sh", owner, repo, token])
        .await?;
    Ok(handle)
}
async fn drain_sandbox_handle(mut handle: ExecHandle, cancel: &CancellationToken) {
    loop {
        tokio::select! {
            _ = cancel.cancelled() => { clerk::debug!("Sandbox cancelled, breaking"); break; },
            event = handle.recv() => {
                let event = match event {
                    Some(event) => event,
                    None => break,
                };
                match event {
                    ExecEvent::Stdout(data) => clerk::debug!("{}", String::from_utf8_lossy(&data)),
                    ExecEvent::Stderr(data) => clerk::debug!("{}", String::from_utf8_lossy(&data)),
                    ExecEvent::Exited { code } => {
                        clerk::debug!("Sandbox exited with code: {code}");
                        break;
                    }
                    _ => {}
                }
            }
        }
    }
}
async fn stop_and_remove_sandbox(sandbox: &Sandbox) {
    let name = sandbox.name();
    if Sandbox::list()
        .await
        .unwrap()
        .iter()
        .all(|s| s.name() != name)
    {
        clerk::debug!("Sandbox {name} is not exists, skipping stop and remove");
        return;
    }

    if let Err(e) = sandbox.stop_and_wait().await {
        clerk::error!("Failed to stop sandbox {name}: {e}");
    }
    if let Err(e) = Sandbox::remove(name).await {
        clerk::error!("Failed to remove sandbox {name}: {e}");
    }
}
