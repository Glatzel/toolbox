//! Library-facing diagnostic types and derive support.
//!
//! This crate provides the traits, core diagnosis types, and `IDiagnosis`
//! derive needed to define errors without pulling in reporting, rendering, or
//! backtrace symbolication. It is `no_std`-compatible and uses `alloc` for
//! owned descriptions and metadata.
//!
//! # Examples
//!
//! ```
//! use mischief_api::{IDiagnosis, MischiefError};
//!
//! #[derive(IDiagnosis)]
//! #[diagnosis(description = "invalid input")]
//! struct InputError {
//!     #[diagnosis(source)]
//!     source: Option<MischiefError>,
//! }
//!
//! let error = InputError { source: None };
//! assert_eq!(error.description(), "invalid input");
//! ```
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(
    unused_extern_crates,
    reason = "The self alias keeps generated derive paths valid inside this crate."
)]

extern crate self as mischief_api;

pub use mischief_core::{
    DescriptionDisplay, IAsDiagnosis, IDiagnosis, IErrorSource, IForeignErrorSource,
    IIntoDiagnosis, MischiefError, MischiefErrorBuilder, Severity,
};
pub use mischief_macros::IDiagnosis;
