extern crate alloc;
use alloc::boxed::Box;
use alloc::string::ToString;
use core::error::Error;
use core::fmt::{Debug, Display};

use crate::IDiagnostic;

pub struct MischiefError {
    description: alloc::string::String,
    source: Option<Box<MischiefError>>,
}

impl MischiefError {
    pub fn new<D>(description: D, source: Option<Box<MischiefError>>) -> Self
    where
        D: Display,
    {
        Self {
            description: description.to_string(),
            source,
        }
    }
    pub fn description(&self) -> Option<&str> { Some(&self.description) }
    pub fn source(&self) -> Option<&MischiefError> { self.source.as_deref() }
}
impl Debug for MischiefError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.description().unwrap_or_default())
    }
}
impl IDiagnostic for MischiefError {
    fn description<'a>(&'a self) -> Option<alloc::boxed::Box<dyn Display + 'a>> {
        Some(Box::new(&self.description))
    }

    fn source(&self) -> Option<&dyn IDiagnostic> { self.source().map(|f| f as &dyn IDiagnostic) }
}
impl<E> From<E> for MischiefError
where
    E: Error,
{
    fn from(value: E) -> Self {
        // convert recursively
        fn convert(err: &dyn Error) -> MischiefError {
            MischiefError {
                description: err.to_string(),
                source: err.source().map(|src| Box::new(convert(src))),
            }
        }

        convert(&value)
    }
}
