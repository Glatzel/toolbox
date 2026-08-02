mod cli;

#[tokio::main]
pub async fn main() -> mischief::Result<()> { cli::main().await }
