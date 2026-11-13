//! Error types produced by parsing rules.
//!
//! A rule that does not match produces a [`RuleError`]. When a rule is driven
//! by a parser, that error is wrapped in a [`VerbError`] recording the
//! [`Verb`] attempted, the rule's type name, and the input.
//! [`FilterError`] is returned by character-set filters.

extern crate alloc;
use alloc::borrow::Cow;
use alloc::string::String;
use core::fmt::{self, Debug};

use mischief::IDiagnosis;

use crate::text::{IRule, Verb};
/// Error returned when a rule does not match the input.
#[derive(Debug, Clone, PartialEq, Eq, IDiagnosis)]
#[diagnosis(error,display, description_from = reason)]
pub struct RuleError {
    /// Human-readable reason the rule failed.
    pub reason: Cow<'static, str>,
}
impl RuleError {
    /// Converts this error into a [`VerbError`] for a specific verb.
    ///
    /// # Arguments
    ///
    /// * `verb` - The parser operation that was applying the rule.
    /// * `input` - The input the rule was applied to.
    ///
    /// # Returns
    ///
    /// A [`VerbError`] recording `verb`, the type name of `R`, `input`, and
    /// this error.
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
/// Error returned when a parser operation fails while applying a rule.
#[derive(Debug, Clone, PartialEq, Eq, IDiagnosis)]
#[diagnosis(error, display, description = "parser verb failed")]
pub struct VerbError {
    /// The parser operation that was being applied.
    pub verb: Verb,
    /// Fully qualified type name of the rule that failed.
    pub rule: &'static str,
    /// Input the rule was applied to.
    pub input: Cow<'static, str>,
    /// Underlying rule error.
    pub rule_error: RuleError,
}
/// Error returned by a character-set filter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilterError(
    /// Description of the failure.
    pub String,
);

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn error_display_is_unchanged() {
        let rule = RuleError {
            reason: Cow::Borrowed("expected a digit"),
        };
        assert_eq!(rule.to_string(), "Rule Error: expected a digit");

        let filter = FilterError(String::from("missing character"));
        assert_eq!(filter.to_string(), "Filter Error: missing character");
    }
}
