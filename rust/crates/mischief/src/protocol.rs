extern crate alloc;

use core::fmt::{Debug, Display};

/// Trait for diagnostic information.
///
/// Implement this trait for any custom error type that can provide
/// structured diagnostic information. This is similar in spirit to
/// `miette::Diagnostic` or `anyhow::Error` with context.
pub trait IDiagnostic {
    /// Returns a textual description of the error.
    fn description(&self) -> &str;

    /// Returns the underlying source error, if any.
    fn source(&self) -> Option<&dyn IDiagnostic>;

    /// Returns an optional error code.
    fn code(&self) -> Option<&str>;

    /// Returns the severity level of the error.
    fn severity(&self) -> Option<Severity>;

    /// Returns optional help text describing how to fix the issue.
    fn help(&self) -> Option<&str>;

    /// Returns an optional URL for further documentation or reference.
    fn url(&self) -> Option<&str>;

    // Optional future extensions for source code snippets or labels:
    // fn source_code<'a>(&'a self) -> Option<&'a str>;
    // fn labels<'a>(&'a self) -> Option<&'a str>;
}

/// Represents the severity of a diagnostic message.
///
/// # Variants
///
/// - `Advice` — Just some guidance or a suggestion.
/// - `Warning` — A cautionary message; something may be wrong.
/// - `Error` — Critical failure. The default severity if none is specified.
#[derive(Default, Debug, Clone, Copy)]
pub enum Severity {
    /// Just some help or suggestion.
    Advice,
    /// Warning. Please take note.
    Warning,
    /// Critical failure; program cannot continue.
    #[default]
    Error,
}

impl Display for Severity {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Severity::Advice => write!(f, "Advice"),
            Severity::Warning => write!(f, "Warning"),
            Severity::Error => write!(f, "Error"),
        }
    }
}
