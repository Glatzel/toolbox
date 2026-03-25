use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::{env, fs};

use arbor::indents::UnicodeIndent;
use arbor::protocol::ILazyTree;
use clap::{Parser, ValueEnum};
use clerk::tracing_subscriber::layer::SubscriberExt;
use clerk::tracing_subscriber::util::SubscriberInitExt;
use clerk::tracing_subscriber::{EnvFilter, Layer};
use goblin::pe::PE;
use hashbrown::HashSet;
use owo_colors::OwoColorize;
use path_slash::PathExt;
use strum::Display;
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
enum ShowOption {
    #[default]
    #[strum(serialize = "all")]
    All,
    #[strum(serialize = "missing")]
    Missing,
}
static LIMIT: OnceLock<usize> = OnceLock::new();
static SHOW_OPTION: OnceLock<ShowOption> = OnceLock::new();
#[derive(Debug, Clone)]
struct ImportsTree {
    name: String,
    base: Option<PathBuf>,
    depth: usize,
}
impl ImportsTree {
    pub fn new(name: String, base: Option<PathBuf>, depth: usize) -> Self {
        Self { name, base, depth }
    }
    fn find(name: &str, base: &Path) -> Option<PathBuf> {
        clerk::trace!("Searching DLL: {}", name);

        let candidate = base.join(name);
        if candidate.exists() {
            clerk::debug!("Found DLL in local directory: {}", candidate.display());
            return Some(base.to_path_buf());
        }

        if let Ok(path_env) = env::var("PATH") {
            for p in env::split_paths(&path_env) {
                let candidate = p.join(name);
                if candidate.exists() {
                    clerk::debug!("Found DLL in PATH: {}", candidate.display());
                    return Some(p);
                }
            }
        }

        clerk::info!("DLL not found: {}", name);

        None
    }
}
impl ILazyTree for ImportsTree {
    type Leave = ImportsTree;

    fn content(&self) -> String {
        match (self.depth, &self.base, SHOW_OPTION.get().unwrap()) {
            (0, _, _) => self.name.clone(),
            (_, Some(p), ShowOption::All) => {
                format!("{} -> {}", &self.name, p.join(&self.name).to_slash_lossy())
                    .green()
                    .to_string()
            }
            (_, Some(_), ShowOption::Missing) => self.name.to_string(),
            (_, None, ShowOption::All) => {
                if self.name.starts_with("api-ms-win") {
                    format!(
                        "{} {} {}",
                        &self.name.green().to_string(),
                        "->".green(),
                        "VirtualImport".yellow().bold()
                    )
                } else {
                    self.name.to_string().red().to_string()
                }
            }
            (_, None, ShowOption::Missing) => self.name.to_string().red().to_string(),
        }
    }

    fn leaves(&self) -> Option<Vec<Self::Leave>> {
        if self.depth + 1 > *LIMIT.get().unwrap() && *LIMIT.get().unwrap() > 0 {
            clerk::trace!("Depth limit reached at {}", self.name);
            return None;
        }

        match &self.base {
            Some(base) => {
                let path = base.join(&self.name);

                clerk::trace!("Reading PE file: {}", path.display());

                let buf = match fs::read(&path) {
                    Ok(b) => b,
                    Err(e) => {
                        clerk::warn!("Failed to read {}: {}", path.display(), e);
                        return None;
                    }
                };

                let pe = match PE::parse(&buf) {
                    Ok(p) => p,
                    Err(e) => {
                        clerk::warn!("Failed to parse PE {}: {}", path.display(), e);
                        return None;
                    }
                };

                clerk::debug!("Parsed PE imports for {}", self.name);

                let mut leaves = Vec::new();
                let mut visited = HashSet::new();

                for import in pe.imports {
                    let dll = import.dll.to_string();

                    if visited.contains(&dll) {
                        clerk::trace!("Skipping duplicate import {}", dll);
                        continue;
                    }
                    clerk::trace!("Found import {}", dll);
                    visited.insert(dll.clone());
                    match (Self::find(&dll, base), SHOW_OPTION.get().unwrap()) {
                        (leaf_base, ShowOption::All) => {
                            leaves.push(Self::new(dll, leaf_base, self.depth + 1))
                        }
                        (None, ShowOption::Missing) => {
                            if dll.starts_with("api-ms-win") {
                                continue;
                            }
                            leaves.push(Self::new(dll, None, self.depth + 1))
                        }
                        (Some(dll_base), ShowOption::Missing) => {
                            if self.depth + 2 > *LIMIT.get().unwrap() && *LIMIT.get().unwrap() > 0 {
                                continue;
                            }
                            if dll.starts_with("api-ms-win") {
                                continue;
                            }
                            let path = dll_base.join(&dll);
                            let buf = match fs::read(&path) {
                                Ok(b) => b,
                                Err(_) => {
                                    leaves.push(Self::new(dll, None, self.depth + 1));
                                    continue;
                                }
                            };

                            let pe = match PE::parse(&buf) {
                                Ok(p) => p,
                                Err(_) => {
                                    leaves.push(Self::new(dll, None, self.depth + 1));
                                    continue;
                                }
                            };
                            let dll_imports = pe.imports;
                            if dll_imports.is_empty() {
                                continue;
                            }
                            if dll_imports
                                .iter()
                                .any(|d| Self::find(d.dll, &dll_base).is_none())
                            {
                                leaves.push(Self::new(dll, Some(dll_base), self.depth + 1));
                            }
                        }
                    };
                }

                if leaves.is_empty() {
                    None
                } else {
                    Some(leaves)
                }
            }
            None => {
                clerk::warn!("Skipping unresolved dependency {}", self.name);
                None
            }
        }
    }
}
fn execute(args: Args) -> mischief::Result<()> {
    let abs_path = dunce::canonicalize(&args.input)?;
    clerk::info!("Scanning executable: {}", abs_path.display());
    LIMIT.set(args.limit).unwrap();
    SHOW_OPTION.set(args.show_option).unwrap();
    let tree = ImportsTree::new(
        abs_path.file_name().unwrap().to_string_lossy().to_string(),
        Some(abs_path.parent().unwrap().to_path_buf()),
        0,
    );
    clerk::debug!("Dependency tree root created");
    let render = arbor::lazy_renders::LazyRender {
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
            clerk::layer::terminal_layer(true).with_filter(
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
