#![no_std]

pub use crate::report::{IntoMischief, Report, Result, WrapErr};

mod diagnostic;
pub(crate) mod report;
