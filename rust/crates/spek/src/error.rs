use thiserror::Error;

use crate::pad::PadError;
use crate::windows::WindowError;

#[derive(Error, Debug)]
pub enum SpekError {
    #[error(transparent)]
    Pad(#[from] PadError),
    #[error(transparent)]
    Window(#[from] WindowError),
}
