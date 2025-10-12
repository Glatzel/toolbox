//! Mischief — ergonomic, no_std-friendly error reporting and diagnostics.
//!
//! This crate provides a lightweight error/reporting abstraction built for
//! std/no_std environments while still making it convenient to produce
//! human-friendly diagnostic output. It focuses on:
//! - Representing errors and their causal chain (`MischiefError` and `Report`).
//! - Converting standard `Error` types into `Report` via `IntoMischief`.
//! - Rich, configurable rendering of diagnostics via the `render` module.

#![no_std]
// #![feature(specialization)]
// #![allow(incomplete_features)]
pub use crate::report::{IntoMischief, Report, Result, WrapErr};
mod error;
pub(crate) mod report;
pub use error::MischiefError;
mod protocol;
pub mod render;
pub use protocol::{IDiagnostic, Severity};
#[cfg(feature = "macros")]
mod macros;
#[cfg(feature = "macros")]
pub use macros::mischief;
