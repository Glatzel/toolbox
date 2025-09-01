extern crate alloc;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use core::error::Error;
use core::fmt::{Debug, Display, Write};
pub trait IDiagnostic {
    fn description<'a>(&'a self) -> Option<alloc::boxed::Box<dyn Display + 'a>>;
    fn source<'a>(&self) -> Option<&dyn IDiagnostic>;
}

// Removed default implementation for Debug, as Rust does not support default
// trait methods outside nightly.
impl<T> IDiagnostic for T
where
    T: Debug,
{
    default fn description<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        let mut msg = String::new();
        let _ = write!(msg, "{:?}", self);
        Some(alloc::boxed::Box::new(msg))
    }

    default fn source<'a>(&self) -> Option<&dyn IDiagnostic> { None }
}
impl<T> IDiagnostic for T
where
    T: Error,
{
    fn description<'a>(&'a self) -> Option<alloc::boxed::Box<dyn Display + 'a>> {
        Some(alloc::boxed::Box::new(self))
    }

    fn source<'b>(&'b self) -> Option<&dyn IDiagnostic> {
        self.source().map(|e| e as &dyn IDiagnostic)
    }
}

pub struct MischiefError {
    description: alloc::string::String,
    source: Option<alloc::boxed::Box<dyn IDiagnostic>>,
}

impl MischiefError {
    pub fn new<D>(description: D, source: Option<alloc::boxed::Box<dyn IDiagnostic>>) -> Self
    where
        D: Display,
    {
        Self {
            description: description.to_string(),
            source,
        }
    }
}
impl IDiagnostic for MischiefError {
    fn description<'a>(&'a self) -> Option<alloc::boxed::Box<dyn Display + 'a>> {
        Some(Box::new(self.description.clone()))
    }

    fn source<'a>(&self) -> Option<&dyn IDiagnostic> { self.source.as_deref() }
}
