mod cli;
mod config;
mod payload;
mod utils;
mod webhook;
#[tokio::main]
async fn main() -> mischief::Result<()> { cli::main().await }
