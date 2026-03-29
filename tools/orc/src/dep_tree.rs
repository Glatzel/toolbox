#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;
#[cfg(target_os = "windows")]
mod win;
use arbor::protocol::ILazyTree;
#[cfg(target_os = "windows")]
pub use win::*;

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
