#![no_std]
pub use crate::report::{IntoMischief, Report, Result, WrapErr};
mod error;
pub(crate) mod report;
pub use error::MischiefError;
mod protocol;
mod render;
pub use protocol::IDiagnostic;
