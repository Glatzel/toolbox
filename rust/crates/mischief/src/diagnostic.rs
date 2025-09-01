extern crate alloc;
use alloc::string::{String, ToString};
use core::fmt::Display;

#[derive(Debug)]
pub struct Diagnostic {
    description: String,
    source: Option<alloc::boxed::Box<Diagnostic>>,
}
impl Diagnostic {
    pub(crate) fn new<D>(description: D, source: Option<alloc::boxed::Box<Diagnostic>>) -> Self
    where
        D: Display + Send + Sync + 'static,
    {
        Self {
            description: description.to_string(),
            source,
        }
    }
    pub(crate) fn description(&self) -> &str { self.description.as_str() }
    pub(crate) fn source(&self) -> Option<&Diagnostic> { self.source.as_deref() }
}
