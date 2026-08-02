use thiserror::Error;
extern crate alloc;
use alloc::borrow::Cow;
use alloc::string::String;
use core::fmt::Debug;

use crate::string::{IRule, Verb};
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[error("Rule Error: {reason}")]
pub struct RuleError {
    pub reason: Cow<'static, str>,
}
impl RuleError {
    #[cold]
    pub fn to_verb<R>(self, verb: Verb, input: &str) -> VerbError
    where
        R: IRule,
    {
        VerbError {
            verb,
            rule: R::type_name(),
            input: Cow::Owned(input.into()),
            rule_error: self,
        }
    }
}
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[error("Verb Error: verb={verb:?}, rule={rule}, input={input}, rule_error={rule_error}")]
pub struct VerbError {
    pub verb: Verb,
    pub rule: &'static str,
    pub input: Cow<'static, str>,
    pub rule_error: RuleError,
}
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[error("Filter Error: {0}")]
pub struct FilterError(pub String);
