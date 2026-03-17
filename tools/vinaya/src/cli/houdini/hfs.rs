use clap::{Parser, Subcommand};
use mischief::IntoMischief;
use path_slash::PathBufExt;
use validator::Validate;

use crate::cli::{ArgMajor, ArgMinor, ArgNoCheck, ArgPatch};
use crate::hou::HoudiniInstance;
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
        #[command(flatten)]
        no_check: ArgNoCheck,
    },
    FromVersionString {
        version_string: String,
        #[arg(long)]
        no_check: bool,
    },
    Latest {},
}
pub fn execute(args: Args) -> mischief::Result<()> {
    match args.command {
        Commands::FromVersion {
            major,
            minor,
            patch,
            no_check,
        } => command_from_version(
            major.value(),
            minor.value(),
            patch.value(),
            !no_check.value(),
        )?,
        Commands::FromVersionString {
            version_string,
            no_check,
        } => command_from_version_string(version_string.as_str(), !no_check)?,
        Commands::Latest {} => command_latest()?,
    };
    Ok(())
}
fn command_from_version(
    major: u16,
    minor: u16,
    patch: u16,
    check_installed: bool,
) -> mischief::Result<()> {
    let instance = HoudiniInstance {
        major,
        minor,
        patch,
    };
    instance.validate().into_mischief()?;
    if check_installed {
        instance.check_is_installed()?
    }
    println!("{}", instance.hfs().to_slash_lossy());
    Ok(())
}
fn command_from_version_string(
    version_string: &str,
    check_installed: bool,
) -> mischief::Result<()> {
    let instance = HoudiniInstance::from_version_string(version_string)?;
    instance.validate().into_mischief()?;
    if check_installed {
        instance.check_is_installed()?
    }
    println!("{}", instance.hfs().to_slash_lossy());
    Ok(())
}
fn command_latest() -> mischief::Result<()> {
    let instance = HoudiniInstance::latest_installed_version()?;

    println!("{}", instance.hfs().to_slash_lossy());
    Ok(())
}
