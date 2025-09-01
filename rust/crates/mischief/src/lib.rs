#![no_std]

mod diagnostic;
pub(crate) mod report;
pub use report::{IntoMischief, Report, WrapErr};
