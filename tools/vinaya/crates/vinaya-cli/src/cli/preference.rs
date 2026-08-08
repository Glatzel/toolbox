use clap::Parser;
use hou_where::HoudiniPreference;
use path_slash::PathExt;

use super::{ArgMajor, ArgMinor};

#[derive(Parser, Debug)]
pub struct Args {
    #[command(flatten)]
    major: ArgMajor,
    #[command(flatten)]
    minor: ArgMinor,
}

pub fn execute(args: &Args) -> mischief::Result<()> {
    let pref = HoudiniPreference::from_version(args.major.value(), args.minor.value())?;
    println!("{}", pref.directory.to_slash_lossy());
    Ok(())
}
