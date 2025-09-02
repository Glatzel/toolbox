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
    pub fn description<'a>(&'a self) -> Option<&'a str> { Some(&self.description) }
    pub fn source(&self) -> Option<&MischiefError> { self.source.as_deref() }
}
impl<T> From<T> for MischiefError
where
    T: Error,
{
    fn from(value: T) -> Self {
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

impl MischiefError {
    /// Wrap anything that implements Display (preferred for user-facing).
    pub fn from_display<D>(desc: D) -> Self
    where
        D: Display,
    {
        Self {
            description: desc.to_string(),
            source: None,
        }
    }

    /// Wrap anything that only implements Debug.
    pub fn from_debug<D>(dbg: D) -> Self
    where
        D: Debug,
    {
        let mut description = String::new();
        write!(description, "{:?}", dbg).unwrap();
        Self {
            description,
            source: None,
        }
    }
}
