extern crate alloc;

use core::fmt::Debug;

pub trait IDiagnostic: Debug {
    fn description<'a>(&'a self) -> &'a str;
    fn source(&self) -> Option<&dyn IDiagnostic>;
    fn code(&self) -> Option<&str>;
    fn severity(&self) -> Option<Severity>;
    fn help(&self) -> Option<&str>;
    fn url(&self) -> Option<&str>;
    //  fn source_code<'a>(&'a self) -> Option<&'a str>;
    //  fn labels<'a>(&'a self) -> Option<&'a str>;
}

#[derive(Default, Debug, Clone)]
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
