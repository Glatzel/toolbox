#![no_std]

pub use crate::report::{IntoMischief, Report, WrapErr};

mod diagnostic;
pub(crate) mod report;
