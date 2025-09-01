#![no_std]
#![allow(incomplete_features)]
#![feature(specialization)]
pub use crate::report::{IntoMischief, Report, Result, WrapErr};
mod diagnostic;
pub(crate) mod report;
