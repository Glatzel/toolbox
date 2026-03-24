use std::path::{Path, PathBuf};
use std::{env, fs};

use arbor::protocol::ILazyTree;
use clap::{Parser, ValueEnum};
use goblin::pe::PE;
use hashbrown::HashSet;
use owo_colors::OwoColorize;
use path_slash::PathExt;
use strum::Display;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
    #[arg(long, short, default_value_t = 1)]
    limit: usize,
    #[arg(long,short, default_value_t = ShowOption::All)]
    show: ShowOption,
    #[arg(help = "Path to the executable to scan")]
    executable: PathBuf,
}

#[derive(Default, Debug, Display, Clone, PartialEq, Eq, ValueEnum)]
enum ShowOption {
    #[default]
    #[strum(serialize = "all")]
    All,
    #[strum(serialize = "missing")]
    Missing,
}
#[derive(Debug, Clone)]
struct ImportsTree {
    name: String,
    base: Option<PathBuf>,
    limit: usize,
    depth: usize,
}
impl ImportsTree {
    pub fn new(name: String, base: &Path, limit: usize, depth: usize) -> Self {
        match (depth, Self::find(&name, &base)) {
            (0, _) => Self {
                name,
                base: Some(base.to_path_buf()),
                limit,
                depth,
            },
            (_, p) => Self {
                name,
                base: p,
                limit,
                depth,
            },
        }
    }
    fn find(name: &str, base: &Path) -> Option<PathBuf> {
        let candidate = base.join(&name);
        if candidate.exists() {
            return Some(candidate);
        }

        if let Ok(path_env) = env::var("PATH") {
            for p in env::split_paths(&path_env) {
                let candidate = p.join(&name);
                if candidate.exists() {
                    return Some(candidate);
                }
            }
        }
        None
    }
}
impl ILazyTree for ImportsTree {
    type Leave = ImportsTree;

    fn content(&self) -> String {
        match (self.depth, &self.base) {
            (0, _) => self.name.clone(),
            (_, Some(p)) => format!("{} -> {}", self.name, p.to_slash_lossy())
                .green()
                .to_string(),
            _ => format!("{}", self.name).red().to_string(),
        }
    }

    fn leaves(&self) -> Option<Vec<Self::Leave>> {
        if self.depth + 1 > self.limit && self.limit > 0 {
            return None;
        }
        match &self.base {
            Some(base) => {
                let buf = match fs::read(base.join(&self.name)) {
                    Ok(b) => b,
                    Err(_) => return None,
                };
                let pe = match PE::parse(&buf) {
                    Ok(p) => p,
                    Err(_) => return None,
                };
                let mut leaves = Vec::new();
                let mut visited = HashSet::new();
                for import in pe.imports {
                    let dll = import.dll.to_string();
                    if visited.contains(&dll) {
                        continue;
                    }
                    visited.insert(dll.clone());
                    leaves.push(Self::new(dll, &base, self.limit, self.depth + 1));
                }
                Some(leaves)
            }
            None => None,
        }
    }
}
fn execute(args: Args) -> mischief::Result<()> {
    let abs_path = dunce::canonicalize(&args.executable)?;

    let tree = ImportsTree::new(
        abs_path.file_name().unwrap().to_string_lossy().to_string(),
        abs_path.parent().unwrap(),
        args.limit,
        0,
    );
    let render = arbor::lazy_renders::LazyRender {
        tree: tree,
        indent: arbor::indents::UnicodeIndent,
        width: match terminal_size::terminal_size() {
            Some((w, _)) => w.0 as usize,
            None => 80,
        },
    };
    println!("{render}");
    Ok(())
}

pub fn main() -> mischief::Result<()> {
    let args = Args::parse(); //config logger
    let log_level = match args.verbose.filter() {
        clap_verbosity_flag::VerbosityFilter::Off => clerk::LogLevel::OFF,
        clap_verbosity_flag::VerbosityFilter::Trace => clerk::LogLevel::TRACE,
        clap_verbosity_flag::VerbosityFilter::Debug => clerk::LogLevel::DEBUG,
        clap_verbosity_flag::VerbosityFilter::Info => clerk::LogLevel::INFO,
        clap_verbosity_flag::VerbosityFilter::Warn => clerk::LogLevel::WARN,
        clap_verbosity_flag::VerbosityFilter::Error => clerk::LogLevel::ERROR,
    };
    clerk::tracing_subscriber::registry()
        .with(clerk::layer::terminal_layer(log_level, true))
        .init();
    execute(args)
}
