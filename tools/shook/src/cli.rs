mod common;
mod init;
mod log;
mod run;
mod serve;

use clap::{Parser, Subcommand};
use common::CommonArgs;
use log::init_log;

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
    Run(CommonArgs),
    Serve(CommonArgs),
}

pub async fn main() -> mischief::Result<()> {
    let args = Args::parse();
    init_log(&args);

    match args.commands {
        Commands::Init => init::execute()?,
        Commands::Run(args) => {
            run::execute(args).await?;
        }
        Commands::Serve(args) => {
            serve::execute(args).await?;
        }
    }

    Ok(())
}
