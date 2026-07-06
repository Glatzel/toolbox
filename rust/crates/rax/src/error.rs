use thiserror::Error;
extern crate alloc;
use alloc::string::{String, ToString};
use core::fmt::Debug;

use crate::string::{IRule, Verb};
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[error("Rule Error: {reason}")]
pub struct RuleError {
    pub reason: String,
}
impl RuleError {
    pub fn to_verb<R: IRule>(self, verb: Verb, input: &str) -> VerbError {
        VerbError {
            verb,
            rule: R::type_name(),
            input: input.to_string(),
            rule_error: self,
        }
    }
}
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[error("Verb Error: verb={verb:?}, rule={rule}, input={input}, rule_error={rule_error}")]
pub struct VerbError {
    pub verb: Verb,
    pub rule: &'static str,
    pub input: String,
    pub rule_error: RuleError,
}
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[error("Filter Error: {0}")]
pub struct FilterError(pub String);
