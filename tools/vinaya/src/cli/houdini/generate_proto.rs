use std::path::{Path, PathBuf};

use clap::Parser;

use crate::cli::{ArgMajor, ArgMinor, ArgPatch, HOUDINI_OPTIONS};
use crate::hou::HoudiniInstance;
#[derive(Parser, Debug)]
pub struct Args {
    #[command(flatten)]
    major: ArgMajor,
    #[command(flatten)]
    minor: ArgMinor,
    #[command(flatten)]
    patch: ArgPatch,
    #[arg(help_heading=HOUDINI_OPTIONS,long, default_value_t = 3)]
    python_version_major: u8,
    #[arg(help_heading=HOUDINI_OPTIONS,long)]
    python_version_minor: u8,
    #[arg(help_heading=HOUDINI_OPTIONS,short,long,num_args(0..))]
    infiles: Vec<PathBuf>,
    #[arg(help_heading=HOUDINI_OPTIONS,short, long)]
    outfile: PathBuf,
}

pub fn execute(args: Args) -> mischief::Result<()> {
    let instance = HoudiniInstance {
        major: args.major.value(),
        minor: args.minor.value(),
        patch: args.patch.value(),
    };

    instance.generate_proto(
        args.python_version_major,
        args.python_version_minor,
        &args
            .infiles
            .iter()
            .map(|f| f.as_path())
            .collect::<Vec<&Path>>(),
        &args.outfile,
    )
}
