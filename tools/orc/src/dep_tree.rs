#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod win;
use crate::cli::{SHOW_OPTION, ShowOption};
use arbor::protocol::ILazyTree;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct DepTree {
    pub name: String,
    pub base: Option<PathBuf>,
    pub depth: usize,
}
impl DepTree {
    pub fn new(name: String, base: Option<PathBuf>, depth: usize) -> Self {
        Self { name, base, depth }
    }
}
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
