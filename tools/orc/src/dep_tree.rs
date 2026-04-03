use std::fs::read_link;
use std::path::{Path, PathBuf};
use std::{env, fs};

use arbor::protocol::{ILazyTree, ITreeContent};
#[cfg(target_os = "linux")]
use goblin::elf::Elf;
#[cfg(target_os = "windows")]
use goblin::pe::PE;
use owo_colors::OwoColorize;
use path_slash::PathBufExt;

use crate::cli::{LIMIT, SHOW_OPTION, ShowOption};

#[derive(Debug, Clone)]
pub struct DepTree {
    pub name: String,
    pub base: Option<PathBuf>,
    pub depth: usize,
    pub target: Option<PathBuf>,
}

/// Parsed information extracted from a binary on disk.
struct BinaryInfo {
    /// Deduplicated, sorted list of imported library names.
    imports: Vec<String>,
    #[cfg(target_os = "linux")]
    rpaths: Vec<String>,
    #[cfg(target_os = "linux")]
    runpaths: Vec<String>,
}

/// Parse an already-read buffer into a `BinaryInfo`, or return `None` on error.
fn parse_binary_buf(buf: &[u8]) -> Option<BinaryInfo> {
    #[cfg(target_os = "windows")]
    let binary = PE::parse(buf).ok()?;
    #[cfg(target_os = "linux")]
    let binary = Elf::parse(buf).ok()?;

    #[cfg(target_os = "windows")]
    let mut imports: Vec<String> = binary.imports.iter().map(|i| i.dll.to_string()).collect();
    #[cfg(target_os = "linux")]
    let mut imports: Vec<String> = binary.libraries.iter().map(|s| s.to_string()).collect();
    imports.sort();
    imports.dedup();

    Some(BinaryInfo {
        imports,
        #[cfg(target_os = "linux")]
        rpaths: binary.rpaths.iter().map(|s| s.to_string()).collect(),
        #[cfg(target_os = "linux")]
        runpaths: binary.runpaths.iter().map(|s| s.to_string()).collect(),
    })
}

impl DepTree {
    pub fn new(name: &str, base: Option<PathBuf>, depth: usize, target: Option<PathBuf>) -> Self {
        Self {
            name: name.to_string(),
            base,
            depth,
            target,
        }
    }

    /// Returns `true` when `self.depth + extra` would exceed the configured
    /// limit.
    fn exceeds_limit(&self, extra: usize) -> bool {
        let limit = *LIMIT.get().unwrap();
        limit > 0 && self.depth + extra > limit
    }

    /// Locate `dll` and its symlink target relative to `base`, using `info`'s
    /// rpath/runpath data on Linux.
    fn resolve_import(
        &self,
        dll: &str,
        base: &Path,
        #[cfg(target_os = "linux")] info: &BinaryInfo,
    ) -> (Option<PathBuf>, Option<PathBuf>) {
        #[cfg(target_os = "windows")]
        let dll_base = Self::find_dll_base(dll, base);
        #[cfg(target_os = "linux")]
        let dll_base = Self::find_dll_base(dll, base, &info.rpaths, &info.runpaths);

        let target = Self::find_link_target(
            &dll_base
                .clone()
                .unwrap_or_else(|| base.to_path_buf())
                .join(dll),
        );
        (dll_base, target)
    }
}

impl DepTree {
    #[cfg(target_os = "windows")]
    pub fn find_dll_base(name: &str, base: &Path) -> Option<PathBuf> {
        if base.join(name).exists() {
            return Some(base.to_path_buf());
        }
        if let Ok(path_env) = env::var("PATH") {
            for p in env::split_paths(&path_env) {
                if p.join(name).exists() {
                    return Some(p);
                }
            }
        }
        None
    }

    #[cfg(target_os = "linux")]
    pub fn find_dll_base(
        name: &str,
        base: &Path,
        rpaths: &[String],
        runpaths: &[String],
    ) -> Option<PathBuf> {
        if base.join(name).exists() {
            return Some(base.to_path_buf());
        }

        if let Ok(ld_path) = env::var("LD_LIBRARY_PATH") {
            for p in env::split_paths(&ld_path) {
                if p.join(name).exists() {
                    return Some(p);
                }
            }
        }

        // runpaths take priority over rpaths when present
        let rpath_list = if !runpaths.is_empty() {
            runpaths
        } else {
            rpaths
        };
        for p in rpath_list {
            let path = Path::new(p);
            if path.join(name).exists() {
                return Some(path.to_path_buf());
            }
        }

        #[cfg(not(feature = "test-mode"))]
        {
            const SYSTEM_PATHS: &[&str] = &[
                "/lib",
                "/lib64",
                "/usr/lib",
                "/usr/lib64",
                "/usr/local/lib",
                "/usr/local/lib64",
                #[cfg(target_arch = "x86_64")]
                "/lib/x86_64-linux-gnu",
                #[cfg(target_arch = "x86_64")]
                "/usr/lib/x86_64-linux-gnu",
                #[cfg(target_arch = "aarch64")]
                "/lib/aarch64-linux-gnu",
                #[cfg(target_arch = "aarch64")]
                "/usr/lib/aarch64-linux-gnu",
            ];
            for p in SYSTEM_PATHS {
                let path = Path::new(p);
                if path.join(name).exists() {
                    return Some(path.to_path_buf());
                }
            }
        }

        None
    }

    pub fn find_link_target(link: &Path) -> Option<PathBuf> {
        match read_link(link) {
            Ok(target) if target.is_absolute() => Some(target),
            Ok(target) => Some(dunce::canonicalize(link.parent()?).unwrap().join(target)),
            Err(_) => None,
        }
    }

    pub fn content_all(&self) -> String {
        match (self.depth, &self.base, &self.target) {
            (0, _, None) => self.name.clone(),
            (_, Some(p), None) => {
                format!(
                    "{} [{}]",
                    &self.name,
                    p.join(&self.name).to_slash_lossy().green()
                )
            }
            #[cfg(target_os = "windows")]
            (_, None, None) if self.name.starts_with("api-ms-win") => {
                format!("{} [{}]", &self.name, "VirtualImport".blue())
            }
            (_, None, None) => self.name.red().to_string(),
            (0, _, Some(target)) => {
                format!("{} -> {}", &self.name, target.to_slash_lossy().green())
            }
            (_, Some(_), Some(target)) => {
                format!("{} -> {}", &self.name, target.to_slash_lossy().green())
            }
            (_, None, Some(target)) => {
                format!("{} -> {}", &self.name, target.to_slash_lossy().red())
            }
        }
    }

    pub fn content_missing(&self) -> String {
        match (self.depth, &self.base, &self.target) {
            (0, _, None) => self.name.clone(),
            (_, Some(_), None) => self.name.clone(),
            (_, None, None) => self.name.red().to_string(),
            (0, _, Some(target)) => format!("{} -> {}", &self.name, target.to_slash_lossy()),
            (_, Some(_), Some(target)) => format!("{} -> {}", &self.name, target.to_slash_lossy()),
            (_, None, Some(target)) => {
                format!("{} -> {}", &self.name, target.to_slash_lossy().red())
            }
        }
    }

    pub fn leaves_all(&self) -> Vec<DepTree> {
        if self.exceeds_limit(1) {
            return Vec::new();
        }
        let Some(base) = &self.base else {
            return Vec::new();
        };
        let path = base.join(&self.name);

        let buf = match fs::read(&path) {
            Ok(b) => b,
            Err(e) => {
                clerk::warn!("Failed to read {}: {}", path.display(), e);
                return Vec::new();
            }
        };
        let Some(info) = parse_binary_buf(&buf) else {
            return Vec::new();
        };

        info.imports
            .iter()
            .map(|dll| {
                let (dll_base, target) = self.resolve_import(
                    dll,
                    base,
                    #[cfg(target_os = "linux")]
                    &info,
                );
                Self::new(dll, dll_base, self.depth + 1, target)
            })
            .collect()
    }

    pub fn leaves_missing(&self) -> Vec<DepTree> {
        if self.exceeds_limit(1) {
            return Vec::new();
        }
        let Some(base) = &self.base else {
            return Vec::new();
        };

        let buf = match fs::read(base.join(&self.name)) {
            Ok(b) => b,
            Err(_) => return Vec::new(),
        };
        let Some(info) = parse_binary_buf(&buf) else {
            return Vec::new();
        };

        let mut leaves = Vec::new();

        for dll in &info.imports {
            #[cfg(target_os = "windows")]
            if dll.starts_with("api-ms-win") {
                continue;
            }

            let (dll_base, target) = self.resolve_import(
                dll,
                base,
                #[cfg(target_os = "linux")]
                &info,
            );

            let Some(dll_base) = dll_base else {
                leaves.push(Self::new(dll, None, self.depth + 1, target));
                continue;
            };

            if self.exceeds_limit(2) {
                continue;
            }

            let dep_buf = match fs::read(dll_base.join(dll)) {
                Ok(b) => b,
                Err(_) => {
                    leaves.push(Self::new(dll, None, self.depth + 1, target));
                    continue;
                }
            };
            let Some(dep_info) = parse_binary_buf(&dep_buf) else {
                continue;
            };

            if dep_info.imports.is_empty() {
                continue;
            }

            let has_missing = dep_info.imports.iter().any(|d| {
                #[cfg(target_os = "windows")]
                {
                    Self::find_dll_base(d, &dll_base).is_none()
                }
                #[cfg(target_os = "linux")]
                {
                    Self::find_dll_base(d, &dll_base, &dep_info.rpaths, &dep_info.runpaths)
                        .is_none()
                }
            });

            if has_missing {
                leaves.push(Self::new(dll, Some(dll_base), self.depth + 1, target));
            }
        }
        leaves
    }
}

impl ITreeContent for DepTree {
    fn content(&self) -> impl AsRef<str> {
        match SHOW_OPTION.get().unwrap() {
            ShowOption::All => self.content_all(),
            ShowOption::Missing => self.content_missing(),
        }
    }
}

impl ILazyTree for DepTree {
    type Leaf = DepTree;
    type Leaves = std::vec::IntoIter<DepTree>;

    fn leaves(&self) -> Self::Leaves {
        match SHOW_OPTION.get().unwrap() {
            ShowOption::All => self.leaves_all().into_iter(),
            ShowOption::Missing => self.leaves_missing().into_iter(),
        }
    }
}
