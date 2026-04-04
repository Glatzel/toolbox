mod cli;
mod config;
mod nomad;
mod payload;
mod webhook;
#[tokio::main]
async fn main() -> mischief::Result<()> {
    cli::main().await
}
