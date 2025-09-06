#![no_std]

pub use crate::report::{Report, Result, WrapErr};
mod error;
pub(crate) mod report;
pub use error::MischiefError;
mod protocol;
mod render;
pub use mischief_macros::mischief;
pub use protocol::{IDiagnostic, Severity};
mod macros;