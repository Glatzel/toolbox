use clap::{Parser, Subcommand};
use hou_where::HoudiniInstance;
use path_slash::PathBufExt;

use crate::cli::HOUDINI_OPTIONS;
#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Houdini cmake prefix path
    Cmake,

    /// Houdini HFS path
    Hfs,

    /// Houdini major version
    Major,

    /// Houdini minor version
    Minor,

    /// Houdini patch version
    Patch,

    /// Houdini version
    Version {
        /// Set to ignore patch version
        #[arg(short,long,help_heading=HOUDINI_OPTIONS)]
        short: bool,
    },
}
pub fn execute(args: &Args) -> mischief::Result<()> {
    let hinstance = HoudiniInstance::latest_installed_version()?;
    match &args.command {
        Commands::Cmake => {
            println!("{}", hinstance.cmake_prefix_path()?.to_slash_lossy());
        }
        Commands::Hfs => println!("{}", hinstance.hfs()?.to_slash_lossy()),
        Commands::Major => println!("{}", hinstance.version.major),
        Commands::Minor => println!("{}", hinstance.version.minor),
        Commands::Patch => println!("{}", hinstance.version.patch),
        Commands::Version { short } => {
            if *short {
                println!("{}.{}", hinstance.version.major, hinstance.version.minor);
            } else {
                println!("{}", hinstance.version);
            }
        }
    }
    Ok(())
}
