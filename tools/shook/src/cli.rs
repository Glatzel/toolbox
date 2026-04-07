use std::env;
use std::path::PathBuf;
use std::sync::Arc;

use axum::Router;
use axum::routing::post;
use clap::Parser;
use clerk::tracing_subscriber::EnvFilter;
use clerk::tracing_subscriber::Layer;
use clerk::tracing_subscriber::layer::SubscriberExt;
use clerk::tracing_subscriber::util::SubscriberInitExt;

use crate::config::Config;
use crate::nomad::NomadClient;
use crate::webhook::{AppContext, webhook};

#[derive(Debug, Parser)]
#[command(author = "Glatzel", version, long_about = None)]
struct Args {
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
    config: PathBuf,
}

fn app(shared_state: Arc<AppContext>) -> Router {
    Router::new()
        .route("/webhook", post(webhook))
        .with_state(shared_state)
}

pub async fn main() -> mischief::Result<()> {
    let args = Args::parse();

    clerk::tracing_subscriber::registry()
        .with(
            clerk::terminal_layer(true).with_filter(
                EnvFilter::builder()
                    .with_default_directive(
                        format!("{}={}", env!("CARGO_PKG_NAME"), args.verbose.filter())
                            .parse()
                            .unwrap(),
                    )
                    .from_env_lossy(),
            ),
        )
        .init();

    clerk::debug!(config_path = %args.config.display(), "Loading configuration");
    let config = Config::load_toml(&args.config)?;

    clerk::debug!(nomad_url = %config.nomad.url, "Initialising Nomad client");
    let client = NomadClient::new(&config.nomad)?;

    let port = config.server.port;
    let shared_state = Arc::new(AppContext { config, client });

    clerk::debug!("Building router");
    let app = app(shared_state.clone());

    let addr = format!("0.0.0.0:{port}");
    clerk::info!(address = %addr, "Binding listener");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .inspect_err(|e| {
            clerk::error!(address = %addr, error = %e, "Failed to bind TCP listener");
        })?;

    clerk::info!(address = %addr, "Server started, waiting for connections");
    axum::serve(listener, app).await.inspect_err(|e| {
        clerk::error!(error = %e, "Server exited with error");
    })?;

    Ok(())
}
