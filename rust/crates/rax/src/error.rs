use thiserror::Error;
extern crate alloc;
use alloc::string::String;
use core::fmt::Debug;

use crate::string::{IRule, Verb};
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[error("Rule Error: {reason}")]
pub struct RuleError {
    pub reason: String,
}
impl RuleError {
    pub fn to_verb<'a, R: IRule>(self, verb: Verb, input: &'a str) -> VerbError<'a> {
        VerbError {
            verb,
            rule: R::type_name(),
            input,
            rule_error: self,
        }
    }
}
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[error("Verb Error: verb={verb:?}, rule={rule}, input={input}, rule_error={rule_error}")]
pub struct VerbError<'a> {
    pub verb: Verb,
    pub rule: &'static str,
    pub input: &'a str,
    pub rule_error: RuleError,
}
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[error("Filter Error: {0}")]
pub struct FilterError(pub String);
