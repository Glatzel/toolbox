use clap::Parser;
use hou_variable::HoudiniVersionShort;
use hou_where::HoudiniPreference;
use path_slash::PathExt;

use crate::cli::custom_parser::parse_generic;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(value_parser = parse_generic::<HoudiniVersionShort>)]
    version: HoudiniVersionShort,
}

pub fn execute(args: &Args) -> mischief::Result<()> {
    let pref = HoudiniPreference::from_version(&args.version)?;
    println!("{}", pref.directory.to_slash_lossy());
    Ok(())
}
