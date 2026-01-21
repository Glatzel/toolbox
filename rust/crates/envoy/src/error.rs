use core::str::Utf8Error;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnvoyError {
    #[error("The pointer was null.")]
    NullPtr,
    #[error(transparent)]
    InvalidUtf8(#[from] Utf8Error),
}
