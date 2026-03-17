mod cli;
mod hou;
#[tokio::main]
pub async fn main() { cli::main().await; }
