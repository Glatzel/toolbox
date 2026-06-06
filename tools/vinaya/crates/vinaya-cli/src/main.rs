mod cli;

#[tokio::main]
pub async fn main() { cli::main().await; }
