use clap::{Parser, Subcommand};
use hou_variable::HoudiniVersion;
use hou_where::HoudiniInstance;
use path_slash::PathBufExt;


use crate::cli::{ArgMajor, ArgMinor, ArgPatch};
#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}
#[derive(Debug, Subcommand)]
pub enum Commands {
    FromVersion {
        #[command(flatten)]
        major: ArgMajor,
        #[command(flatten)]
        minor: ArgMinor,
        #[command(flatten)]
        patch: ArgPatch,
    },
    FromVersionString {
        version_string: String,
    },
    Latest,
}
pub fn execute(args: &Args) -> mischief::Result<()> {
    let instance = match &args.command {
        Commands::FromVersion {
            major,
            minor,
            patch,
        } => HoudiniInstance {
            version: HoudiniVersion::new(major.value(), minor.value(), Some(patch.value())),
        },
        Commands::FromVersionString { version_string } => {
            HoudiniInstance::from_version_string(version_string)?
        }
        Commands::Latest => HoudiniInstance::latest_installed_version()?,
    };
    println!("{}", instance.hfs()?.to_slash_lossy());
    Ok(())
}
