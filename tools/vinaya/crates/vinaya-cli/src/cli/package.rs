use core::fmt::Debug;

use clap::{Parser, Subcommand};
use comfy_table::{Attribute, Cell, Color, Table};
use hou_variable::HoudiniVersionShort;
use hou_where::HoudiniPackageManager;
use owo_colors::OwoColorize;
use path_slash::PathExt;

use super::HOUDINI_OPTIONS;
use crate::cli::custom_parser::parse_generic;
#[derive(Parser, Debug)]
pub struct Args {
    #[arg(value_parser = parse_generic::<HoudiniVersionShort>)]
    version: HoudiniVersionShort,

    #[command(subcommand)]
    pub command: Commands,
}
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Houdini package directory
    Dir,

    /// Disable a package
    Disable {
        #[arg(help_heading=HOUDINI_OPTIONS,short, long)]
        names: Vec<String>,
    },

    /// Enable a package
    Enable {
        #[arg(help_heading=HOUDINI_OPTIONS,short, long)]
        names: Vec<String>,
    },

    /// List all packages
    List,
}
pub fn execute(args: &Args) -> mischief::Result<()> {
    let mut manager = HoudiniPackageManager::from_version(&args.version)?;

    match &args.command {
        Commands::Dir => println!("{}", manager.package_dir.to_slash_lossy()),
        Commands::Disable { names } => {
            manager.check_is_existed()?;
            manager.switch_packages(names, false)?;
        }
        Commands::Enable { names } => {
            manager.check_is_existed()?;
            manager.switch_packages(names, true)?;
        }
        Commands::List => print_packages(&manager),
    }
    Ok(())
}
fn print_packages(manager: &HoudiniPackageManager) {
    println!(
        "{}",
        format!(
            "Houdini Packages {}.{}",
            manager.version.major, manager.version.minor
        )
        .color(owo_colors::DynColors::Rgb(255, 102, 0))
        .bold()
    );
    //print table
    let mut table = Table::new();
    table.set_header(vec![
        Cell::new("Name").add_attribute(Attribute::Bold),
        Cell::new("Enabled").add_attribute(Attribute::Bold),
    ]);
    for p in &manager.packages {
        let enable_cell = if p.enable {
            Cell::new(p.enable).fg(Color::Green)
        } else {
            Cell::new(p.enable).fg(Color::Red)
        };
        table.add_row(vec![Cell::new(&p.name), enable_cell]);
    }
    println!("{table}");
}
