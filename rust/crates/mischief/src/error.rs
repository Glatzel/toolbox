extern crate alloc;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use core::fmt::Display;

use crate::IDiagnostic;

/// Concrete diagnostic error type used by the crate.
///
/// `MischiefError` provides a structured representation of an error
/// containing a primary description and optional metadata such as
/// a causal source, error code, severity level, help text, and
/// documentation URL.
///
/// The type is designed to be lightweight and composable. Errors can
/// form chains through the `source` field, allowing higher-level
/// diagnostics to wrap lower-level failures while preserving the
/// original cause.
///
/// Unlike plain error types that only implement `Display`, this
/// structure exposes diagnostic metadata that can be consumed by
/// renderers, logging systems, or reporting frameworks.
#[derive(Clone)]
pub struct MischiefError {
    /// Human-readable description of the diagnostic.
    description: String,

    /// Optional underlying error that caused this diagnostic.
    ///
    /// This enables hierarchical error chains where each layer
    /// adds contextual information to an underlying failure.
    pub(crate) source: Option<Box<MischiefError>>,

    /// Optional stable identifier for the diagnostic.
    ///
    /// Error codes may be used to categorize errors or reference
    /// documentation.
    #[cfg(feature = "fancy")]
    code: Option<String>,

    /// Optional severity level associated with the diagnostic.
    ///
    /// If not specified, consumers may assume a default severity.
    #[cfg(feature = "fancy")]
    severity: Option<crate::Severity>,

    /// Optional guidance describing how the issue may be resolved.
    #[cfg(feature = "fancy")]
    help: Option<String>,

    /// Optional URL pointing to related documentation.
    #[cfg(feature = "fancy")]
    url: Option<String>,
}

impl MischiefError {
    /// Creates a new `MischiefError`.
    ///
    /// The constructor accepts a primary description along with
    /// optional diagnostic metadata. Any value implementing
    /// [`Display`] may be used for textual fields, allowing flexible
    /// construction from string literals or formatted values.
    ///
    /// The `source` parameter can be used to attach an underlying
    /// error, forming a diagnostic chain.
    ///
    /// All optional parameters may be omitted by passing `None`.
    pub fn new<D>(
        description: D,
        source: Option<Box<MischiefError>>,
        #[cfg(feature = "fancy")] code: Option<D>,
        #[cfg(feature = "fancy")] severity: Option<crate::Severity>,
        #[cfg(feature = "fancy")] help: Option<D>,
        #[cfg(feature = "fancy")] url: Option<D>,
    ) -> Self
    where
        D: Display,
    {
        Self {
            description: description.to_string(),
            source,
            #[cfg(feature = "fancy")]
            code: code.map(|s| s.to_string()),
            #[cfg(feature = "fancy")]
            severity,
            #[cfg(feature = "fancy")]
            help: help.map(|s| s.to_string()),
            #[cfg(feature = "fancy")]
            url: url.map(|s| s.to_string()),
        }
    }
}

impl IDiagnostic for MischiefError {
    /// Returns the primary description of the diagnostic.
    fn description(&self) -> &str { &self.description }

    /// Returns the underlying diagnostic source, if present.
    ///
    /// This allows consumers to traverse the error chain.
    fn source(&self) -> Option<&dyn IDiagnostic> {
        self.source.as_deref().map(|f| f as &dyn IDiagnostic)
    }

    /// Returns the optional error code associated with the diagnostic.
    fn code(&self) -> Option<&str> {
        #[cfg(feature = "fancy")]
        return self.code.as_deref();
        #[cfg(not(feature = "fancy"))]
        None
    }

    /// Returns the severity level associated with the diagnostic.
    fn severity(&self) -> Option<crate::Severity> {
        #[cfg(feature = "fancy")]
        return self.severity;
        #[cfg(not(feature = "fancy"))]
        None
    }

    /// Returns optional help text describing how the issue might be resolved.
    fn help(&self) -> Option<&str> {
        #[cfg(feature = "fancy")]
        return self.help.as_deref();
        #[cfg(not(feature = "fancy"))]
        None
    }

    /// Returns the optional documentation URL for the diagnostic.
    fn url(&self) -> Option<&str> {
        #[cfg(feature = "fancy")]
        return self.url.as_deref();
        #[cfg(not(feature = "fancy"))]
        None
    }
}
