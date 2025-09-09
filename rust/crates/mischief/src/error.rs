extern crate alloc;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use core::fmt::Display;

use crate::IDiagnostic;

pub struct MischiefError {
    description: String,
    pub(crate) source: Option<Box<MischiefError>>,
    code: Option<String>,
    severity: Option<crate::Severity>,
    help: Option<String>,
    url: Option<String>,
}

impl MischiefError {
    pub fn new<D>(
        description: D,
        source: Option<Box<MischiefError>>,
        code: Option<D>,
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
            severity,
            help: help.map(|s| s.to_string()),
            url: url.map(|s| s.to_string()),
        }
    }
    pub fn description(&self) -> Option<&str> { Some(&self.description) }
    pub fn source(&self) -> Option<&MischiefError> { self.source.as_deref() }
}

impl IDiagnostic for MischiefError {
    fn description(&self) -> &str { &self.description }

    fn source(&self) -> Option<&dyn IDiagnostic> { self.source().map(|f| f as &dyn IDiagnostic) }

    fn code(&self) -> Option<&str> { self.code.as_deref() }

    fn severity(&self) -> Option<crate::protocol::Severity> { self.severity }

    fn help(&self) -> Option<&str> { self.help.as_deref() }

    fn url(&self) -> Option<&str> { self.url.as_deref() }
}
