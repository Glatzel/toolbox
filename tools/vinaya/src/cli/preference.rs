use clap::Parser;
use path_slash::PathExt;

use super::{ArgMajor, ArgMinor, ArgNoCheck};
use crate::hou::HoudiniPreference;

#[derive(Parser, Debug)]
pub struct Args {
    #[command(flatten)]
    major: ArgMajor,
    #[command(flatten)]
    minor: ArgMinor,
    #[command(flatten)]
    no_check: ArgNoCheck,
}

pub fn execute(args: Args) -> mischief::Result<()> {
    let pref = HoudiniPreference::from_version(args.major.value(), args.minor.value())?;
    if !args.no_check.value() {
        pref.check_is_existed()?;
    }
    println!("{}", pref.directory.to_slash_lossy());
    Ok(())
}
