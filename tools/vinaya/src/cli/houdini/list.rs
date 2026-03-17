use clap::Parser;

use crate::hou::HoudiniInstance;

#[derive(Parser, Debug)]
pub struct Args {}
pub fn execute(_args: Args) -> mischief::Result<()> {
    let hinstance = HoudiniInstance::list_installed()?;
    for i in hinstance {
        println!("{}", i.version_string(true))
    }
    Ok(())
}
