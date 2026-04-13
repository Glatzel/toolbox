use microsandbox::Sandbox;

use crate::cli::CommonArgs;
use crate::config::Config;

pub(super) async fn execute(args: CommonArgs) -> mischief::Result<()> {
    let config = Config::load_config(&args.config)?;

    for (name, runner) in config.runners.into_iter() {
        for i in 0..runner.count {
            let name = name.clone();
            let image = runner.image.clone();
            let volumes = runner.volumes.clone();
            let envs = runner.envs.clone();
            let secrets = runner.secrets.clone();
            let user = config.devop.allowed_users[0].clone();
            let repo = config.devop.allowed_repositories[0].clone();
            let token = config.devop.token.clone();

            tokio::task::spawn(async move {
                let mut builder = Sandbox::builder(format!("{name}-{i}"))
                    .image(image.as_ref())
                    .cpus(runner.cpus)
                    .memory(runner.memory)
                    .replace()
                    .entrypoint(["./start-runner.sh", &user, &repo, &token]);
                for (host, guest) in volumes.iter() {
                    builder = builder.volume(guest.to_string_lossy().as_ref(), |m| m.bind(host));
                }
                for (key, value) in envs.iter() {
                    builder = builder.env(key, value);
                }
                for (key, (value, url)) in secrets.iter() {
                    builder = builder.secret(|s| s.env(key).value(value).allow_host(url));
                }
                clerk::debug!("Sandbox builder configured: {name}");
                builder.create().await.unwrap();
                clerk::debug!("Sandbox created: {name}-{i}");
            });
        }
    }
    Ok(())
}
