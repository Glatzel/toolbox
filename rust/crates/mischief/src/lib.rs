//! Structured diagnostics with optional metadata, layered source chains,
//! macros, and terminal rendering.
//!
//! The crate facade re-exports the core diagnostic model, procedural macros,
//! and feature-selected renderer under one API. The minimum supported Rust
//! version is 1.85.
//!
//! # Quick Start
//!
//! ```
//! use mischief::{IDiagnosis, MischiefError, Report, Result, WrapErr};
//!
//! let diagnosis = MischiefError::builder("parse failed")
//!     .code("E_PARSE")
//!     .help("check the input")
//!     .build();
//! let result: Result<()> = Err(Report::new(diagnosis)).wrap_err("load config");
//! let report = result.expect_err("the result should be an error");
//!
//! assert_eq!(report.error().description(), "load config");
//! assert_eq!(
//!     report.error().source().and_then(|source| source.code()),
//!     Some("E_PARSE")
//! );
//! ```
//!
//! # Macros
//!
//! [`mischief!`] constructs a [`Report`] with formatting and metadata.
//! [`bail!`] returns it immediately, while [`ensure!`] returns it only when a
//! condition is false. `bail!` and `ensure!` are available independently of the
//! `macros` feature.
//!
//! ```
//! use mischief::{IDiagnosis, Result, ensure, mischief};
//!
//! let report = mischief!("connect to {}", "db", code = "E_CONNECT", help = "retry");
//! assert_eq!(report.error().code(), Some("E_CONNECT"));
//!
//! fn validate(ok: bool) -> Result<()> {
//!     ensure!(ok, "invalid input", code = "E_INPUT");
//!     Ok(())
//! }
//! assert!(validate(false).is_err());
//! ```
//!
//! # Derived Diagnoses
//!
//! The optional `IDiagnosis` derive implements the trait for named-field
//! structs. A static description is shown here; `description = field` delegates
//! the description to a field implementing `AsRef<str>`.
//!
//! ```
//! use mischief::{IDiagnosis, MischiefError};
//!
//! #[derive(IDiagnosis)]
//! #[diagnosis(description = "invalid input", code = "E_INPUT")]
//! struct InputError {
//!     #[diagnosis(source)]
//!     source: Option<MischiefError>,
//! }
//!
//! let error = InputError { source: None };
//! assert_eq!(error.code(), Some("E_INPUT"));
//! assert!(error.source().is_none());
//! ```
//!
//! Enums may define defaults and override them per variant:
//!
//! ```
//! use mischief::IDiagnosis;
//!
//! #[derive(IDiagnosis)]
//! #[diagnosis(description = "invalid input", code = "E_INPUT")]
//! enum InputError {
//!     Missing,
//!     #[diagnosis(description_from = 0)]
//!     Invalid(String),
//! }
//!
//! assert_eq!(InputError::Missing.code(), Some("E_INPUT"));
//! assert_eq!(InputError::Invalid("name".into()).description(), "name");
//! ```
//!
//! # Foreign Errors
//!
//! Foreign errors retain their concrete root type for downcasting while their
//! source chain is represented by descriptions. Foreign roots must be
//! `Error + Send + Sync + 'static`.
//!
//! ```
//! use core::fmt;
//!
//! use mischief::MischiefError;
//!
//! #[derive(Debug)]
//! struct Timeout;
//!
//! impl fmt::Display for Timeout {
//!     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str("timeout") }
//! }
//!
//! impl core::error::Error for Timeout {}
//!
//! let error = MischiefError::builder("request failed")
//!     .source_error(Timeout)
//!     .build();
//! assert!(error.downcast_ref::<Timeout>().is_some());
//! ```
//!
//! # Features
//!
//! | Feature | Effect |
//! | --- | --- |
//! | `std` | Enables the verbose tree renderer and backtrace support foundations. |
//! | `fancy` | Enables color, hyperlinks, Unicode trees, and text wrapping; implies `std`. |
//! | `macros` | Re-exports `mischief!` and the `IDiagnosis` derive. `bail!` and `ensure!` are always available. |
//! | `backtrace` | Adds debug-build backtraces to verbose reports; implies `std`. |
//! | `textwrap-*` | Selects text-wrapping backend behavior. |
//!
//! Without `std`, [`Report`] uses the simple renderer and prints descriptions
//! only. The facade always uses `alloc`; the lower-level `mischief-core` crate
//! exposes `alloc` as an optional feature.
//!
//! # Crate Layers
//!
//! - `mischief-core` contains the diagnostic trait, metadata types, source
//!   nodes, error conversion, and simple renderer.
//! - `mischief-render` contains the verbose tree renderer and optional styling.
//! - `mischief-macros` contains `mischief!`, `bail!`, `ensure!`, and the
//!   `IDiagnosis` derive.
//! - `mischief` re-exports the public API and selects the renderer by feature.
#![allow(clippy::unwrap_used, reason = "This crate is for error handling.")]
#![cfg_attr(not(feature = "std"), no_std)]
// #![feature(specialization)]
// #![allow(incomplete_features)]
#[allow(
    unused_extern_crates,
    reason = "The self alias keeps generated macro paths valid when expanded in this crate."
)]
extern crate self as mischief;

pub use mischief_core::{
    IAsDiagnosis, IDiagnosis, IErrorSource, IForeignErrorSource, IIntoDiagnosis, MischiefError,
    MischiefErrorBuilder, Severity,
};
pub use mischief_macros::{bail, ensure};

pub use crate::report::{IntoMischief, Report, Result, WrapErr};
pub(crate) mod report;
#[cfg(feature = "macros")]
pub use mischief_macros::{IDiagnosis, mischief};
pub mod render;
pub use render::*;
