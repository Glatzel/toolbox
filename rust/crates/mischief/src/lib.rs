#![no_std]
#![feature(specialization)]
#![allow(incomplete_features)]
pub use crate::report::{IntoMischief, Report, Result, WrapErr};
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
