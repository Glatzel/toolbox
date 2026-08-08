mod latest;

use clap::{Parser, Subcommand};
use hou_variable::HoudiniVersion;
use hou_where::HoudiniInstance;
use path_slash::PathBufExt;

use crate::cli::HOUDINI_OPTIONS;
use crate::cli::custom_parser::parse_generic;
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}
#[derive(Debug, Subcommand)]
pub enum Commands {
    Hfs {
        #[arg(long,value_parser = parse_generic::<HoudiniVersion>,help_heading=HOUDINI_OPTIONS)]
        version: Option<HoudiniVersion>,

        #[arg(long,help_heading=HOUDINI_OPTIONS)]
        latest: bool,
    },
    Latest(latest::Args),

    /// List all installed Houdini instances, descending by version.
    List,
}
pub fn execute(args: &Args) -> mischief::Result<()> {
    match &args.command {
        Commands::Hfs { version, latest } => {
            let instance = match (version, latest) {
                (Some(version), false) => HoudiniInstance {
                    version: version.clone(),
                },
                (None, true) => HoudiniInstance::latest_installed_version()?,
                _ => {
                    mischief::bail!("Either version or latest must be specified.")
                }
            };
            println!("{}", instance.hfs()?.to_slash_lossy());
            Ok(())
        }
        Commands::Latest(cmd) => latest::execute(cmd),
        Commands::List => {
            let hinstance = HoudiniInstance::list_installed()?;
            for i in hinstance {
                println!("{}", i.version);
            }
            Ok(())
        }
    }
}
