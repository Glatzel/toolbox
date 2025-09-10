extern crate alloc;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use core::fmt::Display;

use crate::IDiagnostic;

/// Represents a structured error with optional metadata such as source, code,
/// severity, help message, and URL.
#[derive(Clone)]
pub struct MischiefError {
    description: String,
    pub(crate) source: Option<Box<MischiefError>>,
    code: Option<String>,
    severity: Option<crate::Severity>,
    help: Option<String>,
    url: Option<String>,
}

impl MischiefError {
    /// Constructs a new `MischiefError` with optional metadata.
    ///
    /// All fields that implement `Display` can be passed as `description`,
    /// `code`, `help`, and `url`. `source` is an optional boxed inner error.
    pub fn new<D>(
        description: D,
        source: Option<Box<MischiefError>>,
        code: Option<D>,
        severity: Option<crate::Severity>,
        help: Option<D>,
        url: Option<D>,
    ) -> Self
    where
        D: Display,
    {
        Self {
            description: description.to_string(),
            source,
            code: code.map(|s| s.to_string()),
            severity,
            help: help.map(|s| s.to_string()),
            url: url.map(|s| s.to_string()),
        }
    }

    /// Returns the description as an optional string slice.
    pub fn description(&self) -> Option<&str> { Some(&self.description) }

    /// Returns the source error, if any.
    pub fn source(&self) -> Option<&MischiefError> { self.source.as_deref() }
}

impl IDiagnostic for MischiefError {
    fn description(&self) -> &str { &self.description }

    fn source(&self) -> Option<&dyn IDiagnostic> { self.source().map(|f| f as &dyn IDiagnostic) }

    fn code(&self) -> Option<&str> { self.code.as_deref() }

    fn severity(&self) -> Option<crate::Severity> { self.severity }

    fn help(&self) -> Option<&str> { self.help.as_deref() }

    fn url(&self) -> Option<&str> { self.url.as_deref() }
}
