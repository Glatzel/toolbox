//! `mischief` is a lightweight diagnostic-oriented error handling library.
//!
//! The crate provides structured error types, diagnostic metadata, and
//! configurable rendering mechanisms for displaying error chains.
//! It is designed to integrate naturally with Rust’s standard `Error`
//! ecosystem while allowing richer diagnostic information and flexible
//! presentation of errors.
//!
//! # Diagnostic Model
//!
//! At the core of the crate is the [`IDiagnostic`] trait, which represents
//! structured diagnostic information associated with an error. A diagnostic
//! may include:
//!
//! - A human-readable description
//! - A causal source diagnostic
//! - An optional error code
//! - A severity classification
//! - Optional help text
//! - An optional documentation URL
//!
//! This abstraction allows errors to carry structured metadata that can be
//! consumed by renderers, logging systems, or external tooling.
//!
//! # Structured Errors
//!
//! The primary concrete diagnostic type provided by the crate is
//! [`MischiefError`]. It stores the diagnostic metadata defined by
//! [`IDiagnostic`] and supports recursive source chaining to represent
//! causal error relationships.
//!
//! `MischiefError` forms the internal representation used by the higher-level
//! error container [`Report`].
//!
//! # Error Container
//!
//! [`Report`] is the main user-facing error type of the crate. It wraps a
//! [`MischiefError`] and provides ergonomic integration with Rust’s standard
//! error traits and conversion mechanisms.
//!
//! A `Report` can be created directly from a `MischiefError`, or constructed
//! automatically from any type implementing [`core::error::Error`]. During
//! conversion, the entire source chain of the original error is recursively
//! transformed into structured diagnostics.
//!
//! The crate also provides utilities for working with `Report` values,
//! including:
//!
//! - [`IntoMischief`] for converting results into diagnostic-aware results
//! - [`WrapErr`] for attaching contextual diagnostics to existing errors
//! - A convenient [`Result`] alias using `Report` as the default error type
//!
//! # Rendering
//!
//! Diagnostic output can be rendered in multiple formats depending on the
//! enabled features.
//!
//! When the `fancy` feature is enabled, diagnostics are rendered using a
//! themed tree-based renderer built on the `arbor` crate. This renderer
//! supports:
//!
//! - hierarchical error chains
//! - customizable indentation styles
//! - colored output
//! - terminal hyperlinks
//!
//! When the `fancy` feature is disabled, a minimal text renderer is used
//! that prints the diagnostic chain in a simple, dependency-free format.
//!
//! # Severity
//!
//! Diagnostics may optionally include a [`Severity`] classification. This
//! allows errors to be categorized into advisory messages, warnings, or
//! critical failures, enabling renderers and external systems to adjust
//! presentation accordingly.
//!
//! # Design Goals
//!
//! The crate focuses on several core goals:
//!
//! - structured diagnostic information
//! - ergonomic integration with the standard error trait
//! - minimal runtime overhead
//! - flexible rendering strategies
//! - compatibility with both simple and rich terminal output
//!
//! `mischief` is suitable for applications and libraries that require
//! structured diagnostics while retaining a straightforward error handling
//! model.
//!
//! # Examples
//!
//! ```should_panic
//! use mischief::mischief;
//!
//! use crate::mischief::WrapErr;
//! fn foo() -> std::result::Result<i32, &'static str> { Err("fake error") }
//! fn main() -> mischief::Result<()> {
//!     foo()
//!         .map_err(|e| mischief!("{}", e))
//!         .wrap_err("error wrapper")?;
//!     Ok(())
//! }
//! ```
//!
//! ```should_panic
//! use mischief::{WrapErr, mischief};
//!
//! fn foo() -> std::result::Result<i32, &'static str> { Err("fake error") }
//! fn main() -> mischief::Result<()> {
//!     foo()
//!         .map_err(|e| mischief!("{}", e))
//!         .wrap_err("error wrapper")?;
//!     Ok(())
//! }
//! ```
//!
//! ```should_panic
//! use std::fs::File;
//!
//! use mischief::{IntoMischief, WrapErr};
//!
//! fn main() -> mischief::Result<()> {
//!     let _ = File::open("fake")
//!         .into_mischief()
//!         .wrap_err(mischief::mischief!("mischief wrapper", help = "some help"))
//!         .wrap_err("error wrapper")?;
//!     Ok(())
//! }
//! ```
#![no_std]
// #![feature(specialization)]
// #![allow(incomplete_features)]
pub use crate::report::{IntoMischief, Report, Result, WrapErr};
mod error;
pub(crate) mod report;
pub use error::MischiefError;
mod protocol;
pub use protocol::{IDiagnostic, Severity};
#[cfg(feature = "macros")]
mod macros;
#[cfg(feature = "macros")]
pub use macros::mischief;
#[cfg(feature = "fancy")]
pub mod fancy_render;
#[cfg(not(feature = "fancy"))]
pub mod no_fancy_render;
