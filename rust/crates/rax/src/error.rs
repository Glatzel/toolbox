use thiserror::Error;
extern crate alloc;
use alloc::string::String;
use core::fmt::Debug;

use crate::string::Verb;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[error("Verb Error: verb={verb:?}, rule={rule}, input={input}, extra={extra}")]
pub struct VerbError<'a, E>
where
    E: Debug,
{
    pub verb: Verb,
    pub rule: &'static str,
    pub input: &'a str,
    pub extra: E,
}
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[error("Filter Error: {0}")]
pub struct FilterError(pub String);
