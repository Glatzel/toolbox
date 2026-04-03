use std::env;
use std::path::PathBuf;
use std::sync::Arc;

use axum::Router;
use axum::routing::post;
use clap::Parser;
use clerk::tracing_subscriber::layer::SubscriberExt;
use clerk::tracing_subscriber::util::SubscriberInitExt;
use clerk::tracing_subscriber::{EnvFilter, Layer};

use crate::config::Config;
use crate::webhook::{AppState, webhook};

#[derive(Debug, Parser)]
#[command(author="Glatzel", version, long_about = None)]
struct Args {
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
    #[arg(long, short)]
    config: PathBuf,
}

pub async fn main() -> mischief::Result<()> {
    let args = Args::parse();
    clerk::tracing_subscriber::registry()
        .with(
            clerk::layer::terminal_layer(true).with_filter(
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
    let shared_state = Arc::new(AppState {
        config: Config::load_toml(&args.config)?,
    });
    let app = Router::new()
        .route("/webhook", post(webhook))
        .with_state(shared_state.clone());
    let listener =
        tokio::net::TcpListener::bind(format!("0.0.0.0:{}", shared_state.config.port)).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
