extern crate alloc;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use core::error::Error;
use core::fmt::{Debug, Display, Write};
pub trait IDiagnostic {
    fn description<'a>(&'a self) -> Option<alloc::boxed::Box<dyn Display + 'a>>;
    fn source(&self) -> Option<&dyn IDiagnostic>;
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

    fn source(&self) -> Option<&dyn IDiagnostic> { self.source.as_deref() }
}
impl<T> From<T> for MischiefError
where
    T: Error,
{
    fn from(value: T) -> Self {
        let description = value.to_string();
        let source = value.source().map(|src| {
            Box::new(MischiefError::new(src.to_string(), None)) as Box<dyn IDiagnostic>
        });
        MischiefError::new(description, source)
    }
}
