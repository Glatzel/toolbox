use std::env;

mod init;
mod run;

use axum::Router;
use axum::routing::post;
use clap::{Parser, Subcommand};
use clerk::tracing_subscriber::layer::SubscriberExt;
use clerk::tracing_subscriber::util::SubscriberInitExt;
use clerk::tracing_subscriber::{EnvFilter, Layer};

use crate::cli::run::RunArgs;

#[derive(Debug, Parser)]
#[command(author = "Glatzel", version, long_about = None)]
struct Args {
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
    #[command(subcommand)]
    pub commands: Commands,
}
#[derive(Debug, Subcommand)]
enum Commands {
    Init,
    Run(RunArgs),
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

    match args.commands {
        Commands::Init => init::execute()?,
        Commands::Run(args) => {
            run::execute(args).await?;
        }
    }

    Ok(())
}
