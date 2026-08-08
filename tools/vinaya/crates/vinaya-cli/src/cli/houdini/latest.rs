use clap::{Parser, Subcommand};
use hou_where::HoudiniInstance;
use path_slash::PathBufExt;
#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}
#[derive(Debug, Subcommand)]
pub enum Commands {
    Cmake,
    Hfs,
    Major,
    Minor,
    Patch,
    Version,
    VersionNoPatch,
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
        Commands::Version => println!("{}", hinstance.version),
        Commands::VersionNoPatch => {
            println!("{}.{}", hinstance.version.major, hinstance.version.minor);
        }
    }
    Ok(())
}
