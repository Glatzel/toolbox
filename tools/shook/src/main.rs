mod cli;
mod config;
mod server;
mod utils;
#[tokio::main]
async fn main() -> mischief::Result<()> { cli::main().await }
