extern crate alloc;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use core::error::Error;
use core::fmt::{Debug, Display};

use crate::IDiagnostic;

pub struct MischiefError {
    description: String,
    source: Option<Box<MischiefError>>,
    code: Option<String>,
    severity: Option<crate::Severity>,
    help: Option<String>,
    url: Option<String>,
}

impl MischiefError {
    pub fn new<D>(
        description: &D,
        source: Option<Box<MischiefError>>,
        code: Option<&D>,
        severity: Option<crate::Severity>,
        help: Option<D>,
        url: Option<D>,
    ) -> Self
    where
        D: Display,
    {
        Self {
            description: description.to_string(),
            source,
            code: code.map(|s| s.to_string()),
            severity: severity,
            help: help.map(|s| s.to_string()),
            url: url.map(|s| s.to_string()),
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
impl Display for MischiefError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.description().unwrap_or_default())
    }
}
impl IDiagnostic for MischiefError {
    fn description<'a>(&'a self) -> &'a str { &self.description }

    fn source(&self) -> Option<&dyn IDiagnostic> { self.source().map(|f| f as &dyn IDiagnostic) }

    fn code<'a>(&'a self) -> Option<&'a str> { self.code.as_deref() }

    fn severity(&self) -> Option<crate::protocol::Severity> { self.severity.clone() }

    fn help<'a>(&'a self) -> Option<&'a str> { self.help.as_deref() }

    fn url<'a>(&'a self) -> Option<&'a str> { self.url.as_deref() }
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
                code: None,
                severity: None,
                help: None,
                url: None,
            }
        }

        convert(&value)
    }
}
