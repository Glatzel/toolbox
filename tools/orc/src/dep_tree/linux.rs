use std::path::{Path, PathBuf};
use std::{env, fs};

use goblin::elf::Elf;
use hashbrown::HashSet;
use owo_colors::OwoColorize;
use path_slash::PathBufExt;

use crate::cli::LIMIT;

impl super::DepTree {
    fn find_dll(name: &str, base: &Path) -> Option<PathBuf> {
        clerk::trace!("Searching dep: {}", name);
        let candidate = base.join(name);
        if candidate.exists() {
            clerk::debug!("Found dep in local directory: {}", candidate.display());
            return Some(base.to_path_buf());
        }
        if let Ok(path_env) = env::var("PATH") {
            for p in env::split_paths(&path_env) {
                let candidate = p.join(name);
                if candidate.exists() {
                    clerk::debug!("Found dep in PATH: {}", candidate.display());
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
                let pe = match Elf::parse(&buf) {
                    Ok(p) => p,
                    Err(e) => {
                        clerk::warn!("Failed to parse PE {}: {}", path.display(), e);
                        return None;
                    }
                };
                clerk::debug!("Parsed PE imports for {}", self.name);

                let mut leaves = Vec::new();
                let mut visited = HashSet::new();

                for lib in pe.libraries {
                    if visited.contains(lib) {
                        clerk::trace!("Skipping duplicate import {}", lib);
                        continue;
                    }
                    clerk::trace!("Found import {}", lib);
                    visited.insert(lib.to_string());
                    let dll_base = Self::find_dll(&lib, base);
                    leaves.push(Self::new(lib.to_string(), dll_base, self.depth + 1));
                }
                (!leaves.is_empty()).then_some(leaves)
            }
            None => {
                clerk::warn!("Skipping unresolved dependency {}", self.name);
                None
            }
        }
    }

    pub(super) fn leaves_missing(&self) -> Option<Vec<Self>> { todo!() }
}
