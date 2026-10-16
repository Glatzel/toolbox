use core::fmt::Debug;

use super::IFlowRule;
use crate::error::RuleError;
use crate::text::rules::IRule;

/// Rule that matches a single fixed character at the start of the input.
///
/// The `Char<C, IS_ASCII>` rule checks whether the input begins with the
/// character `C`. On a match it consumes exactly that character and returns
/// `Ok((C, consumed_bytes))`, where the output is the matched character `C`
/// and `consumed_bytes` is its UTF-8 length (`1` for ASCII, otherwise
/// [`char::len_utf8`]).
///
/// If the input is empty, or its first character is not `C`, the rule returns
/// a [`RuleError`].
///
/// The `IS_ASCII` const parameter selects the matching strategy:
///
/// - `Char<C, true>` compares the first input byte against `C as u8` and
///   consumes a single byte. This fast path requires `C` to be an ASCII
///   character; a non-ASCII `C` never matches.
/// - `Char<C, false>` uses [`str::starts_with`] and consumes `C.len_utf8()`
///   bytes, so multi-byte UTF-8 characters are handled correctly.
///
/// This rule is useful for matching literal delimiters, separators, or other
/// fixed punctuation in text-based formats.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Char<const C: char, const IS_ASCII: bool>;

impl<const C: char, const IS_ASCII: bool> IRule for Char<C, IS_ASCII> {}

impl<const C: char> IFlowRule<true> for Char<C, true> {
    type Output<'a> = char;

    /// Applies the `Char` rule using byte-based comparison.
    ///
    /// # Returns
    ///
    /// The matched character `C`, with a consumed length of `1`.
    ///
    /// # Errors
    ///
    /// Returns [`RuleError`] if the input is empty, its first byte is not
    /// `C as u8`, or `C` is not an ASCII character.
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

    /// Applies the `Char` rule using Unicode-aware comparison.
    ///
    /// # Returns
    ///
    /// The matched character `C`, with a consumed length of `C.len_utf8()`.
    ///
    /// # Errors
    ///
    /// Returns [`RuleError`] if the input is empty or does not start with `C`.
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
