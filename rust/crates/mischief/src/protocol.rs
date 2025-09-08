extern crate alloc;

use core::fmt::{Debug, Display};

pub trait IDiagnostic {
    fn description(&self) -> &str;
    fn source(&self) -> Option<&dyn IDiagnostic>;
    fn code(&self) -> Option<&str>;
    fn severity(&self) -> Option<Severity>;
    fn help(&self) -> Option<&str>;
    fn url(&self) -> Option<&str>;
    //  fn source_code<'a>(&'a self) -> Option<&'a str>;
    //  fn labels<'a>(&'a self) -> Option<&'a str>;
}

#[derive(Default, Debug, Clone, Copy)]
pub enum Severity {
    /// Just some help. Here's how you could be doing it better.
    Advice,
    /// Warning. Please take note.
    Warning,
    /// Critical failure. The program cannot continue.
    /// This is the default severity, if you don't specify another one.
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
