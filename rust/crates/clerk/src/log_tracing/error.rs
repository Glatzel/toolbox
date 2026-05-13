use std::path::PathBuf;

use thiserror::Error;
#[derive(Error, Debug)]
pub enum ClerkError {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error("Parent directory not found")]
    ParentDirectoryNotFound(PathBuf),
}
