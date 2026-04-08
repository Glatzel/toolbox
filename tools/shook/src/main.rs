mod cli;
mod config;
mod server;
mod utils;
mod log;
#[tokio::main]
async fn main() -> mischief::Result<()> { cli::main().await }
