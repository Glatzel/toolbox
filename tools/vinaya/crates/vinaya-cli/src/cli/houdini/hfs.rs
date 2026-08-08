use clap::{Parser, Subcommand};
use hou_variable::HoudiniVersion;
use hou_where::HoudiniInstance;
use path_slash::PathBufExt;

use crate::cli::custom_parser::parse_generic;

#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}
#[derive(Debug, Subcommand)]
pub enum Commands {
    FromVersion {
        #[arg(value_parser = parse_generic::<HoudiniVersion>)]
        version: HoudiniVersion,
    },
    Latest,
}
pub fn execute(args: &Args) -> mischief::Result<()> {
    let instance = match &args.command {
        Commands::FromVersion { version } => HoudiniInstance {
            version: version.clone(),
        },

        Commands::Latest => HoudiniInstance::latest_installed_version()?,
    };
    println!("{}", instance.hfs()?.to_slash_lossy());
    Ok(())
}
