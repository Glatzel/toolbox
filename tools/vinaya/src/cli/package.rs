use std::fmt::Debug;

use clap::{Parser, Subcommand};
use comfy_table::*;
use owo_colors::OwoColorize;
use path_slash::PathExt;

use super::{ArgMajor, ArgMinor, HOUDINI_OPTIONS};
use crate::hou::HoudiniPackageManager;
#[derive(Parser, Debug)]
pub struct Args {
    #[command(flatten)]
    major: ArgMajor,
    #[command(flatten)]
    minor: ArgMinor,

    #[command(subcommand)]
    pub command: Commands,
}
#[derive(Debug, Subcommand)]
pub enum Commands {
    Dir {},
    Disable {
        #[arg(help_heading=HOUDINI_OPTIONS,short, long)]
        names: Vec<String>,
    },
    Enable {
        #[arg(help_heading=HOUDINI_OPTIONS,short, long)]
        names: Vec<String>,
    },

    List {},
}
pub fn execute(args: Args) -> mischief::Result<()> {
    let mut manager = HoudiniPackageManager::from_version(args.major.value(), args.minor.value())?;
    manager.check_is_existed()?;
    match args.command {
        Commands::Dir {} => println!("{}", manager.package_dir.to_slash_lossy()),
        Commands::Disable { names } => manager.switch_packages(&names, false)?,
        Commands::Enable { names } => manager.switch_packages(&names, true)?,
        Commands::List {} => print_packages(&manager),
    };
    Ok(())
}
fn print_packages(manager: &HoudiniPackageManager) {
    println!(
        "{}",
        format!("Houdini Packages {}.{}", manager.major, manager.minor)
            .color(owo_colors::DynColors::Rgb(255, 102, 0))
            .bold()
    );
    //print table
    let mut table = Table::new();
    table.set_header(vec![
        Cell::new("Name").add_attribute(Attribute::Bold),
        Cell::new("Enable").add_attribute(Attribute::Bold),
    ]);
    for p in manager.packages.iter() {
        let enable_cell = if p.enable {
            Cell::new(p.enable).fg(Color::Green)
        } else {
            Cell::new(p.enable).fg(Color::Red)
        };
        table.add_row(vec![Cell::new(&p.name), enable_cell]);
    }
    println!("{}", table)
}
