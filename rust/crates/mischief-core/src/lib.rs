//! Core diagnostic types and the dependency-free renderer for `mischief`.

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
mod error;
#[cfg(feature = "alloc")]
pub use error::{MischiefError, MischiefErrorBuilder};

mod protocol;
pub use protocol::{IDiagnosis, Severity};

mod render;
pub use render::render_diagnosis;
