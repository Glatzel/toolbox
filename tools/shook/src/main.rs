mod cli;
mod config;
mod log;
mod server;
mod utils;
#[tokio::main]
async fn main() -> mischief::Result<()> { cli::main().await }
