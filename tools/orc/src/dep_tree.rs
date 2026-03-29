use std::fs::read_link;
use std::path::{Path, PathBuf};
use std::{env, fs};

use arbor::protocol::ILazyTree;
#[cfg(target_os = "linux")]
use goblin::elf::Elf;
#[cfg(target_os = "windows")]
use goblin::pe::PE;
use owo_colors::OwoColorize;
use path_slash::PathBufExt;

use crate::cli::LIMIT;

#[derive(Debug, Clone)]
pub struct DepTree {
    pub name: String,
    pub base: Option<PathBuf>,
    pub depth: usize,
    pub target: Option<PathBuf>,
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
}

impl DepTree {
    #[cfg(target_os = "windows")]
    pub fn find_dll_base(name: &str, base: &Path) -> Option<PathBuf> {
        let candidate = base.join(name);
        if candidate.exists() {
            return Some(base.to_path_buf());
        }

        if let Ok(path_env) = env::var("PATH") {
            for p in env::split_paths(&path_env) {
                let candidate = p.join(name);
                if candidate.exists() {
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
        rpaths: &[&str],
        runpaths: &[&str],
    ) -> Option<PathBuf> {
        let candidate = base.join(name);
        if candidate.exists() {
            return Some(base.to_path_buf());
        }

        if let Ok(ld_path) = env::var("LD_LIBRARY_PATH") {
            for p in env::split_paths(&ld_path) {
                let candidate = p.join(name);
                if candidate.exists() {
                    return Some(p);
                }
            }
        }
        for p in rpaths {
            let path = Path::new(p);
            let candidate = path.join(name);
            if candidate.exists() {
                return Some(path.to_path_buf());
            }
        }
        for p in runpaths {
            let path = Path::new(p);
            let candidate = path.join(name);
            if candidate.exists() {
                return Some(path.to_path_buf());
            }
        }
        None
    }
    pub fn find_link_target(link: &Path) -> Option<PathBuf> {
        match read_link(link) {
            Ok(target) => {
                if target.is_absolute() {
                    Some(target)
                } else {
                    Some(dunce::canonicalize(link.parent()?).unwrap().join(target))
                }
            }
            Err(_) => None,
        }
    }
    pub fn content_all(&self) -> String {
        match (self.depth, &self.base, &self.target) {
            (0, _, None) => self.name.clone(),
            (_, Some(p), None) => {
                format!("{} ({})", &self.name, p.join(&self.name).to_slash_lossy())
                    .green()
                    .to_string()
            }
            (_, None, None) => self.name.red().to_string(),
            (0, _, Some(target)) => format!("{} -> {}", &self.name, target.to_slash_lossy()),
            (_, Some(_p), Some(target)) => {
                format!("{} -> {} ", &self.name, target.to_slash_lossy(),)
                    .green()
                    .to_string()
            }
            (_, None, Some(target)) => format!("{} -> {}", &self.name, target.to_slash_lossy())
                .red()
                .to_string(),
        }
    }
    pub fn content_missing(&self) -> String {
        match (self.depth, &self.base, &self.target) {
            (0, _, None) => self.name.clone(),
            (_, Some(_), None) => self.name.clone(),
            (_, None, None) => self.name.red().to_string(),
            (0, _, Some(target)) => format!("{} -> {}", &self.name, target.to_slash_lossy()),
            (_, Some(_), Some(target)) => format!("{} -> {}", &self.name, target.to_slash_lossy()),
            (_, None, Some(target)) => format!("{} -> {}", &self.name, target.to_slash_lossy())
                .red()
                .to_string(),
        }
    }
    pub fn leaves_all(&self) -> Option<Vec<Self>> {
        if self.depth + 1 > *LIMIT.get().unwrap() && *LIMIT.get().unwrap() > 0 {
            return None;
        }
        match &self.base {
            Some(base) => {
                let path = base.join(&self.name);

                let buf = match fs::read(&path) {
                    Ok(b) => b,
                    Err(e) => {
                        clerk::warn!("Failed to read {}: {}", path.display(), e);
                        return None;
                    }
                };
                #[cfg(target_os = "windows")]
                let binary = match PE::parse(&buf) {
                    Ok(p) => p,
                    Err(_e) => {
                        return None;
                    }
                };
                #[cfg(target_os = "linux")]
                let binary = match Elf::parse(&buf) {
                    Ok(p) => p,
                    Err(_e) => {
                        return None;
                    }
                };

                let mut leaves = Vec::new();

                #[cfg(target_os = "windows")]
                let mut imports = binary.imports;
                #[cfg(target_os = "linux")]
                let mut imports = binary.libraries;
                imports.sort();
                imports.dedup();
                #[cfg(target_os = "linux")]
                let rpaths = binary.rpaths;
                #[cfg(target_os = "linux")]
                let runpaths = binary.runpaths;

                for import in imports {
                    #[cfg(target_os = "windows")]
                    let dll = import.dll;
                    #[cfg(target_os = "linux")]
                    let dll = import;
                    #[cfg(target_os = "linux")]
                    let dll_base = Self::find_dll_base(dll, base, &rpaths, &runpaths);
                    #[cfg(target_os = "windows")]
                    let dll_base = Self::find_dll_base(dll, base);
                    let target =
                        Self::find_link_target(&dll_base.clone().unwrap_or(base.clone()).join(dll));
                    leaves.push(Self::new(dll, dll_base, self.depth + 1, target));
                }
                (!leaves.is_empty()).then_some(leaves)
            }
            None => None,
        }
    }

    pub fn leaves_missing(&self) -> Option<Vec<Self>> {
        if self.depth + 1 > *LIMIT.get().unwrap() && *LIMIT.get().unwrap() > 0 {
            return None;
        }

        match &self.base {
            Some(base) => {
                let path = base.join(&self.name);

                let buf = match fs::read(&path) {
                    Ok(b) => b,
                    Err(_e) => {
                        return None;
                    }
                };
                #[cfg(target_os = "windows")]
                let binary = match PE::parse(&buf) {
                    Ok(p) => p,
                    Err(_e) => {
                        return None;
                    }
                };
                #[cfg(target_os = "linux")]
                let binary = match Elf::parse(&buf) {
                    Ok(p) => p,
                    Err(_e) => {
                        return None;
                    }
                };

                let mut leaves = Vec::new();

                #[cfg(target_os = "windows")]
                let mut imports = binary.imports;
                #[cfg(target_os = "linux")]
                let mut imports = binary.libraries;
                imports.sort();
                imports.dedup();
                #[cfg(target_os = "linux")]
                let rpaths = binary.rpaths;
                #[cfg(target_os = "linux")]
                let runpaths = binary.runpaths;
                for import in imports {
                    #[cfg(target_os = "windows")]
                    let dll_name = import.dll;
                    #[cfg(target_os = "linux")]
                    let dll_name = import;
                    #[cfg(target_os = "windows")]
                    let result = Self::find_dll_base(dll_name, base);
                    #[cfg(target_os = "linux")]
                    let result = Self::find_dll_base(dll_name, base, &rpaths, &runpaths);

                    match result {
                        None => {
                            #[cfg(target_os = "windows")]
                            if dll_name.starts_with("api-ms-win") {
                                continue;
                            }

                            leaves.push(Self::new(dll_name, None, self.depth + 1, None))
                        }
                        Some(dll_base) => {
                            if self.depth + 2 > *LIMIT.get().unwrap() && *LIMIT.get().unwrap() > 0 {
                                continue;
                            }
                            #[cfg(target_os = "windows")]
                            if dll_name.starts_with("api-ms-win") {
                                continue;
                            }
                            let path = dll_base.join(dll_name);
                            let buf = match fs::read(&path) {
                                Ok(b) => b,
                                Err(_) => {
                                    leaves.push(Self::new(dll_name, None, self.depth + 1, None));
                                    continue;
                                }
                            };
                            #[cfg(target_os = "windows")]
                            let binary = match PE::parse(&buf) {
                                Ok(p) => p,
                                Err(_e) => {
                                    return None;
                                }
                            };
                            #[cfg(target_os = "linux")]
                            let binary = match Elf::parse(&buf) {
                                Ok(p) => p,
                                Err(_e) => {
                                    return None;
                                }
                            };
                            #[cfg(target_os = "windows")]
                            let dll_imports = binary.imports;
                            #[cfg(target_os = "linux")]
                            let dll_imports = binary.libraries;
                            #[cfg(target_os = "linux")]
                            let rpaths = binary.rpaths;
                            #[cfg(target_os = "linux")]
                            let runpaths = binary.runpaths;
                            if dll_imports.is_empty() {
                                continue;
                            }
                            if dll_imports.iter().any(|d| {
                                #[cfg(target_os = "windows")]
                                let result = Self::find_dll_base(d.dll, &dll_base).is_none();

                                #[cfg(target_os = "linux")]
                                let result =
                                    Self::find_dll_base(d, &dll_base, &rpaths, &runpaths).is_none();
                                result
                            }) {
                                let target = Self::find_link_target(&dll_base.join(dll_name));
                                leaves.push(Self::new(
                                    dll_name,
                                    Some(dll_base),
                                    self.depth + 1,
                                    target,
                                ));
                            }
                        }
                    };
                }
                (!leaves.is_empty()).then_some(leaves)
            }
            None => None,
        }
    }
}

use crate::cli::{SHOW_OPTION, ShowOption};
impl ILazyTree for DepTree {
    type Leave = DepTree;
    fn content(&self) -> String {
        match SHOW_OPTION.get().unwrap() {
            ShowOption::All => self.content_all(),
            ShowOption::Missing => self.content_missing(),
        }
    }
    fn leaves(&self) -> Option<Vec<Self::Leave>> {
        match SHOW_OPTION.get().unwrap() {
            ShowOption::All => self.leaves_all(),
            ShowOption::Missing => self.leaves_missing(),
        }
    }
}
