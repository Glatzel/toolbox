use core::fmt::Debug;

use super::IFlowRule;
use crate::error::RuleError;
use crate::text::ByteCount;
use crate::text::rules::IRule;

/// Rule that extracts a fixed number of characters from the input string.
///
/// The `CharCount<N, IS_ASCII>` rule attempts to extract the first `N`
/// characters from the input string.
///
/// For `CharCount<N, false>`, characters are counted as Unicode scalar values,
/// so multi-byte UTF-8 characters are handled correctly. For
/// `CharCount<N, true>`, the input is assumed to contain only ASCII characters,
/// and extraction is performed by [`ByteCount`].
///
/// If the input contains at least `N` characters, the rule returns
/// `Ok((prefix, consumed_bytes))`, where:
/// - `prefix` is the first `N` characters of the input,
/// - `consumed_bytes` is the number of bytes consumed.
///
/// If the input contains fewer than `N` characters, the rule returns a
/// [`RuleError`]. When `N` is zero, the rule returns an empty prefix and
/// consumes zero bytes.
///
/// This rule is useful for parsing fixed-width fields when field widths are
/// measured in characters rather than bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CharCount<const N: usize, const IS_ASCII: bool>;

impl<const N: usize, const IS_ASCII: bool> IRule for CharCount<N, IS_ASCII> {}

impl<const N: usize> IFlowRule<false> for CharCount<N, false> {
    type Output<'a> = &'a str;

    /// Applies the `CharCount` rule to the input string.
    ///
    /// # Returns
    ///
    /// - `Ok((prefix, consumed_bytes))` if the input contains at least `N`
    ///   Unicode scalar values.
    /// - `Err(RuleError)` if the input contains fewer than `N` characters.
    ///
    /// For `N == 0`, returns an empty prefix and consumes zero bytes.
    fn apply<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, usize), RuleError> {
        if N == 0 {
            return Ok(("", 0));
        }

        let result = input
            .char_indices()
            .nth(N)
            .map(|(idx, _)| idx)
            .or_else(|| {
                // If the input contains exactly N characters, consume it
                // entirely.
                (input.chars().count() == N).then_some(input.len())
            })
            .map(|idx| {
                // SAFETY: `idx` comes from `char_indices()` or `input.len()`,
                // both of which are valid UTF-8 character boundaries.
                unsafe { (input.get_unchecked(..idx), idx) }
            })
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
    /// - `Ok((prefix, consumed_bytes))` if the input contains at least `N`
    ///   ASCII characters.
    /// - `Err(RuleError)` if the input contains fewer than `N` bytes.
    ///
    /// For `N == 0`, returns an empty prefix and consumes zero bytes.
    ///
    /// The input must contain only ASCII characters for the result to
    /// correspond to a character count.
    fn apply<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, usize), RuleError> {
        if N == 0 {
            return Ok(("", 0));
        }

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
