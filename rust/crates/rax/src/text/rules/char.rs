use core::fmt::Debug;

use super::IFlowRule;
use crate::error::RuleError;
use crate::text::rules::IRule;

/// Rule that matches a specific character at the start of the input string.
///
/// `Char<C>` checks if the first character of the input string is equal to the
/// expected character `C`. If the first character matches, it returns a tuple:
/// `(Some(C), rest)` where `rest` is the remainder of the input after the
/// matched character. Otherwise, it returns `(None, input)`.
///
/// This rule respects UTF-8 character boundaries and only examines the first
/// character of the input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Char<const C: char, const IS_ASCII: bool>;

impl<const C: char, const IS_ASCII: bool> IRule for Char<C, IS_ASCII> {}

impl<const C: char> IFlowRule<true> for Char<C, true> {
    type Output<'a> = char;

    fn apply<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, usize), RuleError> {
        clerk::trace!("{:?}: input='{:?}', expected='{:?}'", self, input, C);
        if C.is_ascii() {
            // C is a const generic, so `C.is_ascii()` and `C as u8` are
            // compile-time constants
            match input.as_bytes().first() {
                Some(&b) if b == C as u8 => Ok((C, 1)),
                _ => Err(RuleError {
                    reason: "expected character not found".into(),
                }),
            }
        } else {
            Err(RuleError {
                reason: "expected character not found".into(),
            })
        }
    }
}
impl<const C: char> IFlowRule<false> for Char<C, false> {
    type Output<'a> = char;

    fn apply<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, usize), RuleError> {
        clerk::trace!("{:?}: input='{:?}', expected='{:?}'", self, input, C);
        if input.starts_with(C) {
            Ok((C, C.len_utf8()))
        } else {
            Err(RuleError {
                reason: "expected character not found".into(),
            })
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_rule;

    test_rule!(ascii_match, "a123", Char::<'a', true>);
    test_rule!(ascii_no_match, "abc", Char::<'d', true>);
    test_rule!(ascii_empty_input, "", Char::<'a', true>);
    test_rule!(utf8_match, "你好", Char::<'你', false>);
}
