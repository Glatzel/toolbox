use clap::{Parser, Subcommand};
use path_slash::PathBufExt;

use crate::hou::HoudiniInstance;
#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}
#[derive(Debug, Subcommand)]
pub enum Commands {
    Cmake {},
    Hfs {},
    Major {},
    Minor {},
    Patch {},
    Version {
        #[arg(long)]
        no_patch: bool,
    },
}
pub fn execute(args: Args) -> mischief::Result<()> {
    let hinstance = HoudiniInstance::latest_installed_version()?;
    match args.command {
        Commands::Cmake {} => {
            println!("{}", hinstance.cmake_prefix_path().to_slash_lossy())
        }
        Commands::Hfs {} => println!("{}", hinstance.hfs().to_slash_lossy()),
        Commands::Major {} => println!("{}", hinstance.major),
        Commands::Minor {} => println!("{}", hinstance.minor),
        Commands::Patch {} => println!("{}", hinstance.patch),
        Commands::Version { no_patch } => println!("{}", hinstance.to_string(!no_patch)),
    };
    Ok(())
}
