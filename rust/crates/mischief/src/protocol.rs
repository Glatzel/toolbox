extern crate alloc;

use core::fmt::{Debug, Display};

/// Trait representing structured diagnostic information.
///
/// Types implementing this trait describe errors or diagnostic conditions
/// in a structured form. A diagnostic may contain a human-readable
/// description, optional metadata such as severity and error codes,
/// and links to related documentation.
///
/// This abstraction enables richer error reporting than a plain
/// `Display` implementation, allowing tools and renderers to extract
/// structured information for formatting or analysis.
///
/// The trait is intentionally minimal and does not prescribe how
/// diagnostics are rendered. Higher-level utilities may use this
/// information to construct formatted reports, diagnostic trees,
/// or machine-readable outputs.
pub trait IDiagnostic {
    /// Returns a human-readable description of the diagnostic.
    ///
    /// This should provide a concise explanation of the problem.
    /// It is typically used as the primary message when displaying
    /// the diagnostic.
    fn description(&self) -> &str;

    /// Returns the underlying cause of this diagnostic, if any.
    ///
    /// This allows diagnostics to form a causal chain where each
    /// diagnostic describes a higher-level context around a more
    /// fundamental failure.
    ///
    /// Returning `None` indicates that this diagnostic has no
    /// underlying source.
    fn source(&self) -> Option<&dyn IDiagnostic>;

    /// Returns a stable error code identifying this diagnostic.
    ///
    /// Error codes are useful for categorization, documentation,
    /// and machine-readable analysis. They may correspond to
    /// documented error identifiers within a project or library.
    ///
    /// Returning `None` indicates that the diagnostic does not
    /// define a specific code.
    fn code(&self) -> Option<&str>;

    /// Returns the severity level associated with this diagnostic.
    ///
    /// Severity conveys how serious the diagnostic is and may
    /// influence how it is presented to users.
    ///
    /// If `None` is returned, renderers may assume a default
    /// severity such as [`Severity::Error`].
    fn severity(&self) -> Option<Severity>;

    /// Returns optional help text describing how the issue might
    /// be resolved.
    ///
    /// Help text typically provides actionable guidance such as
    /// configuration changes, corrective actions, or suggestions
    /// for resolving the diagnostic.
    fn help(&self) -> Option<&str>;

    /// Returns an optional URL pointing to external documentation.
    ///
    /// This may link to a webpage containing detailed explanations,
    /// troubleshooting guides, or reference documentation related
    /// to the diagnostic.
    fn url(&self) -> Option<&str>;

    // Optional future extensions for source code snippets or labels:
    // fn source_code<'a>(&'a self) -> Option<&'a str>;
    // fn labels<'a>(&'a self) -> Option<&'a str>;
}

/// Represents the severity level associated with a diagnostic.
///
/// Severity indicates how serious a diagnostic message is and
/// may affect how it is displayed or handled by tooling.
///
/// When a diagnostic does not explicitly specify a severity,
/// [`Severity::Error`] is typically assumed.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Severity {
    /// Informational guidance or suggestions that may help the user.
    ///
    /// Advice diagnostics do not indicate a problem but may
    /// provide useful recommendations.
    Advice,

    /// A non-fatal issue that may lead to incorrect behavior
    /// or unexpected results.
    ///
    /// Programs may still continue running after a warning.
    Warning,

    /// A critical failure that prevents the operation from
    /// completing successfully.
    ///
    /// This is the default severity when none is specified.
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
