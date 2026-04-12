mod init;
mod log;
mod serve;
use clap::{Parser, Subcommand};
use log::init_log;
use serve::ServeArgs;

#[derive(Debug, Parser)]
#[command(author = "Glatzel", version, long_about = None)]
pub struct Args {
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
    #[command(subcommand)]
    commands: Commands,
}
#[derive(Debug, Subcommand)]
enum Commands {
    Init,
    Serve(ServeArgs),
}

pub async fn main() -> mischief::Result<()> {
    let args = Args::parse();
    init_log(&args);

    match args.commands {
        Commands::Init => init::execute()?,
        Commands::Serve(args) => {
            serve::execute(args).await?;
        }
    }

    Ok(())
}
