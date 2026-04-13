mod cli;
mod config;
mod server;
mod utils;
mod vm;
#[tokio::main]
async fn main() -> mischief::Result<()> { cli::main().await }
