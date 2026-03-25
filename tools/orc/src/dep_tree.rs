#[cfg(target_os = "windows")]
mod win;
#[cfg(target_os = "linux")]
mod linux;
use std::path::{ PathBuf};
use arbor::protocol::ILazyTree;
use crate::cli::{ SHOW_OPTION, ShowOption};

#[derive(Debug, Clone)]
pub struct DepTree {
    pub name: String,
    pub base: Option<PathBuf>,
    pub depth: usize,
}
impl DepTree {
    pub fn new(name: String, base: Option<PathBuf>, depth: usize) -> Self {
        Self { name, base, depth }
    }}
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
