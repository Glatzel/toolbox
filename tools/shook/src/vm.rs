use std::path::PathBuf;

use hashbrown::HashMap;
use microsandbox::{ExecEvent, ExecHandle, MicrosandboxError, Sandbox};

pub async fn build_sandbox(
    name: &str,
    image: &str,
    cpus: u8,
    memory: u32,
    volumes: &HashMap<PathBuf, PathBuf>,
    ports: &HashMap<u16, u16>,
    envs: &HashMap<String, String>,
    secrets: &HashMap<String, (String, String)>,
) -> Result<Sandbox, MicrosandboxError> {
    let mut builder = Sandbox::builder(name)
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

    clerk::debug!("Sandbox builder configured: {name}");
    let sandbox = builder.create().await?;
    clerk::debug!("Sandbox created: {name}");

    Ok(sandbox)
}
pub async fn start_runner(
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
pub async fn drain_sandbox_handle(mut handle: ExecHandle) {
    loop {
        let event = match handle.recv().await {
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
pub async fn stop_and_remove_sandbox(sandbox: &Sandbox) {
    let name = sandbox.name();
    if let Err(e) = sandbox.stop_and_wait().await {
        clerk::error!("Failed to stop sandbox {name}: {e}");
    }
    if let Err(e) = Sandbox::remove(name).await {
        clerk::error!("Failed to remove sandbox {name}: {e}");
    }
}
