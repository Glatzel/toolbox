use std::path::{Path, PathBuf};
use std::{env, fs};

use arbor::protocol::ILazyTree;
use goblin::pe::PE;
use hashbrown::HashSet;
use owo_colors::OwoColorize;
use path_slash::PathBufExt;

use crate::cli::{LIMIT, SHOW_OPTION, ShowOption};

impl super::DepTree {
    pub(super) fn find_dll(name: &str, base: &Path) -> Option<PathBuf> {
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

    pub(super) fn content_all(&self) -> String {
        match (self.depth, &self.base) {
            (0, _) => self.name.clone(),
            (_, Some(p)) => format!("{} -> {}", &self.name, p.join(&self.name).to_slash_lossy())
                .green()
                .to_string(),
            (_, None) if self.name.starts_with("api-ms-win") => format!(
                "{} {} {}",
                &self.name.green().to_string(),
                "->".green(),
                "VirtualImport".yellow().bold()
            ),
            (_, None) => self.name.red().to_string(),
        }
    }
    pub(super) fn content_missing(&self) -> String {
        match (self.depth, &self.base) {
            (0, _) => self.name.clone(),
            (_, Some(_)) => self.name.clone(),
            (_, None) => self.name.red().to_string(),
        }
    }
    pub(super) fn leaves_all(&self) -> Option<Vec<Self>> {
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
                    let dll_base = Self::find_dll(&dll, base);
                    leaves.push(Self::new(dll, dll_base, self.depth + 1));
                }
                (!leaves.is_empty()).then_some(leaves)
            }
            None => {
                clerk::warn!("Skipping unresolved dependency {}", self.name);
                None
            }
        }
    }

    pub(super) fn leaves_missing(&self) -> Option<Vec<Self>> {
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
                    match Self::find_dll(&dll, base) {
                        None => {
                            if dll.starts_with("api-ms-win") {
                                continue;
                            }
                            leaves.push(Self::new(dll, None, self.depth + 1))
                        }
                        Some(dll_base) => {
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
                                .any(|d| Self::find_dll(d.dll, &dll_base).is_none())
                            {
                                leaves.push(Self::new(dll, Some(dll_base), self.depth + 1));
                            }
                        }
                    };
                }
                (!leaves.is_empty()).then_some(leaves)
            }
            None => {
                clerk::warn!("Skipping unresolved dependency {}", self.name);
                None
            }
        }
    }
}
