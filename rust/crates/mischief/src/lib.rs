#![no_std]

pub use crate::report::{Report, Result, WrapErr};
mod error;
pub(crate) mod report;
pub use error::MischiefError;
mod protocol;
mod render;
pub use protocol::{IDiagnostic, Severity};
#[cfg(feature = "macros")]
mod macros;
#[cfg(feature = "macros")]
pub use macros::mischief;
