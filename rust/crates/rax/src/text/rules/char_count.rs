use core::fmt::Debug;

use super::IFlowRule;
use crate::error::RuleError;
use crate::text::ByteCount;
use crate::text::rules::IRule;

/// Rule that extracts a fixed number of characters from the input string.
///
/// The `CharCount<N>` rule attempts to split the input string at exactly `N`
/// characters. If the input contains at least `N` characters, it returns a
/// tuple `(Some(prefix), rest)` where:
/// - `prefix` is the first `N` characters of the input,
/// - `rest` is the remainder of the input.
///
/// If the input contains fewer than `N` characters, the rule returns `(None,
/// input)`.
///
/// This rule operates on **character boundaries**, so it correctly handles
/// multi-byte UTF-8 characters. It is useful for parsing fixed-length
/// fields based on character count rather than byte count.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CharCount<const N: usize, const IS_ASCII: bool>;

impl<const N: usize, const IS_ASCII: bool> IRule for CharCount<N, IS_ASCII> {}

impl<const N: usize> IFlowRule<false> for CharCount<N, false> {
    type Output<'a> = &'a str;

    /// Applies the `CharCount` rule to the input string.
    ///
    /// # Returns
    ///
    /// - `(Some(prefix), rest)` if the input contains at least `N` characters.
    /// - `(None, input)` if the input is shorter than `N` characters.
    ///
    /// # Logging
    ///
    /// Logs trace messages showing the input and requested character count,
    /// debug messages showing the split position, and warnings if the input
    /// is too short.
    fn apply<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, usize), RuleError> {
        if N == 0 {
            clerk::warn!(
                "{:?}: count is zero, returning empty prefix and full input.",
                self
            );

            return Ok(("", 0));
        }
        clerk::trace!("{:?}: input='{:?}', count={:?}", self, input, N);

        let result = input
            .char_indices()
            .nth(N)
            .map(|(idx, _)| idx)
            .or_else(|| {
                // exactly N chars: consume the whole string
                (input.chars().count() == N).then_some(input.len())
            })
            .map(|idx| unsafe { (input.get_unchecked(..idx), idx) })
            .ok_or_else(|| RuleError {
                reason: "not enough chars in input".into(),
            })?;

        Ok(result)
    }
}
impl<const N: usize> IFlowRule<true> for CharCount<N, true> {
    type Output<'a> = &'a str;

    /// Applies the `CharCount` rule to the input string.
    ///
    /// # Returns
    ///
    /// - `(Some(prefix), rest)` if the input contains at least `N` characters.
    /// - `(None, input)` if the input is shorter than `N` characters.
    ///
    /// # Logging
    ///
    /// Logs trace messages showing the input and requested character count,
    /// debug messages showing the split position, and warnings if the input
    /// is too short.
    fn apply<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, usize), RuleError> {
        if N == 0 {
            clerk::warn!(
                "{:?}: count is zero, returning empty prefix and full input.",
                self
            );

            return Ok(("", 0));
        }
        clerk::trace!("{:?}: input='{:?}', count={:?}", self, input, N);

        ByteCount::<N, true>.apply(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_rule;

    test_rule!(ascii_exact_length, "test", CharCount::<4, true>);
    test_rule!(ascii_less_than_length, "hello", CharCount::<2, true>);
    test_rule!(ascii_more_than_length, "short", CharCount::<10, true>);
    test_rule!(ascii_zero, "abc", CharCount::<0, true>);
    test_rule!(ascii_empty_input, "", CharCount::<0, true>);
    test_rule!(utf8_less_than_length, "你好世界", CharCount::<2, false>);
}
