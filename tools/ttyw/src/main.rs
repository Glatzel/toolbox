mod cli;
mod server;
#[tokio::main]
async fn main() -> mischief::Result<()> {
    cli::main().await
}
