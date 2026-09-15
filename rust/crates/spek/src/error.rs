use thiserror::Error;

use crate::windows::WindowError;

#[derive(Error, Debug)]
pub enum SpekError {
    #[error(transparent)]
    Window(#[from] WindowError),
}
