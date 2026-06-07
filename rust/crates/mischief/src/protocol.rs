extern crate alloc;

use core::fmt::{Debug, Display};

/// Trait representing structured diagnosis information.
///
/// Types implementing this trait describe errors or diagnosis conditions
/// in a structured form. A diagnosis may contain a human-readable
/// description, optional metadata such as severity and error codes,
/// and links to related documentation.
///
/// This abstraction enables richer error reporting than a plain
/// `Display` implementation, allowing tools and renderers to extract
/// structured information for formatting or analysis.
///
/// The trait is intentionally minimal and does not prescribe how
/// diagnosiss are rendered. Higher-level utilities may use this
/// information to construct formatted reports, diagnosis trees,
/// or machine-readable outputs.
pub trait IDiagnosis {
    /// Returns a human-readable description of the diagnosis.
    ///
    /// This should provide a concise explanation of the problem.
    /// It is typically used as the primary message when displaying
    /// the diagnosis.
    fn description(&self) -> &str;

    /// Returns the underlying cause of this diagnosis, if any.
    ///
    /// This allows diagnosiss to form a causal chain where each
    /// diagnosis describes a higher-level context around a more
    /// fundamental failure.
    ///
    /// Returning `None` indicates that this diagnosis has no
    /// underlying source.
    fn source(&self) -> Option<&dyn IDiagnosis>;

    /// Returns a stable error code identifying this diagnosis.
    ///
    /// Error codes are useful for categorization, documentation,
    /// and machine-readable analysis. They may correspond to
    /// documented error identifiers within a project or library.
    ///
    /// Returning `None` indicates that the diagnosis does not
    /// define a specific code.
    fn code(&self) -> Option<&str>;

    /// Returns the severity level associated with this diagnosis.
    ///
    /// Severity conveys how serious the diagnosis is and may
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
    /// for resolving the diagnosis.
    fn help(&self) -> Option<&str>;

    /// Returns an optional URL pointing to external documentation.
    ///
    /// This may link to a webpage containing detailed explanations,
    /// troubleshooting guides, or reference documentation related
    /// to the diagnosis.
    fn url(&self) -> Option<&str>;

    // Optional future extensions for source code snippets or labels:
    // fn source_code<'a>(&'a self) -> Option<&'a str>;
    // fn labels<'a>(&'a self) -> Option<&'a str>;
}

/// Represents the severity level associated with a diagnosis.
///
/// Severity indicates how serious a diagnosis message is and
/// may affect how it is displayed or handled by tooling.
///
/// When a diagnosis does not explicitly specify a severity,
/// [`Severity::Error`] is typically assumed.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Severity {
    /// Informational guidance or suggestions that may help the user.
    ///
    /// Advice diagnosiss do not indicate a problem but may
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
