extern crate alloc;
use alloc::string::String;
use core::fmt::Display;
pub trait IDiagnostic {
    fn description<'a>(&'a self) -> Option<alloc::boxed::Box<dyn Display + 'a>>;
    fn source<'a>(&self) -> Option<&dyn IDiagnostic>;
}
pub struct Diagnostic {
    description: alloc::string::String,
    source: Option<alloc::boxed::Box<dyn IDiagnostic>>,
}
impl Diagnostic {
    pub fn new(description: String, source: Option<alloc::boxed::Box<dyn IDiagnostic>>) -> Self {
        Self {
            description,
            source,
        }
    }
}
impl IDiagnostic for Diagnostic {
    fn description<'a>(&'a self) -> Option<alloc::boxed::Box<dyn Display + 'a>> {
        Some(alloc::boxed::Box::new(&self.description))
    }

    fn source<'a>(&self) -> Option<&dyn IDiagnostic> { self.source.as_deref() }
}
