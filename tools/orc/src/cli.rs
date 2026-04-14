use std::env;
use std::path::PathBuf;
use std::sync::OnceLock;

use arbor::indents::UnicodeIndent;
use clap::{Parser, ValueEnum};
use clerk::tracing_subscriber::layer::SubscriberExt;
use clerk::tracing_subscriber::util::SubscriberInitExt;
use clerk::tracing_subscriber::{EnvFilter, Layer};
use path_slash::PathExt;
use strum::Display;

use crate::dep_tree::DepTree;
#[derive(Debug, Parser)]
#[command(author="Glatzel", version, about="Scan a PE executable or DLL for missing dependencies.", long_about = None)]
struct Args {
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
    #[arg(
        long,
        short,
        default_value_t = 1,
        help = "Limit the level of dependencies to show, 0 is show all level."
    )]
    limit: usize,
    #[arg(long,short, default_value_t = ShowOption::All,help="Show all dependencies or only missing ones")]
    show_option: ShowOption,
    #[arg(help = "Path to the exe or dll to scan")]
    input: PathBuf,
}

#[derive(Default, Debug, Display, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum ShowOption {
    #[default]
    #[strum(serialize = "all")]
    All,
    #[strum(serialize = "missing")]
    Missing,
}
pub static LIMIT: OnceLock<usize> = OnceLock::new();
pub static SHOW_OPTION: OnceLock<ShowOption> = OnceLock::new();

fn execute(args: Args) -> mischief::Result<()> {
    let abs_path = dunce::canonicalize(&args.input)?;
    clerk::info!(
        "Scanning executable: {}",
        abs_path.to_slash_lossy().to_string()
    );
    LIMIT.set(args.limit).unwrap();
    SHOW_OPTION.set(args.show_option).unwrap();
    let tree = DepTree::new(
        args.input.file_name().unwrap().to_str().unwrap(),
        Some(abs_path.parent().unwrap().to_path_buf()),
        0,
        DepTree::find_link_target(&args.input),
    );
    clerk::debug!("Dependency tree root created");
    let render = arbor::renders::LazyRender {
        tree,
        indent: UnicodeIndent,
        width: 0,
    };
    clerk::trace!("Rendering dependency tree");
    println!("{render}");
    Ok(())
}

pub fn main() -> mischief::Result<()> {
    let args = Args::parse();
    clerk::tracing_subscriber::registry()
        .with(
            clerk::terminal_layer(true).with_filter(
                EnvFilter::builder()
                    .with_default_directive(
                        format!("{}={}", env!("CARGO_PKG_NAME"), args.verbose.filter())
                            .parse()
                            .unwrap(),
                    )
                    .from_env_lossy(),
            ),
        )
        .init();
    execute(args)
}
