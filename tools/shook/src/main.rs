mod cli;
mod config;
mod payload;
mod webhook;
#[tokio::main]
async fn main() -> mischief::Result<()> {
    cli::main().await
}
