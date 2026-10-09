use core::fmt::Debug;

use super::IFlowRule;
use crate::error::RuleError;
use crate::text::rules::IRule;

/// Rule that extracts a fixed number of bytes from the input string.
///
/// The `ByteCount<N, IS_ASCII>` rule attempts to extract the first `N` bytes
/// of the input string.
///
/// If the input contains at least `N` bytes and the split occurs at a valid
/// UTF-8 character boundary, the rule returns `Ok((prefix, N))`, where
/// `prefix` is the extracted substring and `N` is its byte length.
///
/// If the input is shorter than `N` bytes or the split falls inside a UTF-8
/// character, the rule returns a [`RuleError`].
///
/// The `IS_ASCII` const parameter specifies whether the input is assumed to
/// contain only ASCII characters. The extraction itself uses [`str::get`],
/// which checks UTF-8 character boundaries regardless of this parameter.
///
/// This rule is useful for parsing fixed-width fields in text-based protocols
/// and other formats where field widths are measured in bytes.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Char<const C: char, const IS_ASCII: bool>;

impl<const C: char, const IS_ASCII: bool> IRule for Char<C, IS_ASCII> {}

impl<const C: char> IFlowRule<true> for Char<C, true> {
    type Output<'a> = char;

    fn apply<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, usize), RuleError> {
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
