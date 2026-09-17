use super::IFlowRule;
use crate::error::RuleError;
use crate::text::rules::IRule;

/// Rule that extracts a fixed number of bytes from the input string.
///
/// The `ByteCount<N>` rule attempts to split the input string at exactly `N`
/// bytes. If the input has at least `N` bytes and the split is on a valid UTF-8
/// boundary, it returns a tuple `(Some(prefix), rest)` where:
/// - `prefix` is the first `N` bytes of the input,
/// - `rest` is the remainder of the input.
///
/// If the input is shorter than `N` bytes, or if `N` would split a UTF-8
/// character in half, the rule returns `(None, input)`.
///
/// This rule is useful for parsing fixed-width fields or binary-like data
/// represented as UTF-8 strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ByteCount<const N: usize, const IS_ASCII: bool>;

impl<const N: usize, const IS_ASCII: bool> IRule for ByteCount<N, IS_ASCII> {}

impl<const N: usize, const IS_ASCII: bool> IFlowRule<IS_ASCII> for ByteCount<N, IS_ASCII> {
    type Output<'a> = &'a str;

    /// Applies the `ByteCount` rule to the input string.
    ///
    /// # Returns
    ///
    /// - `(Some(prefix), rest)` if the input contains at least `N` bytes and
    ///   the split occurs on a valid UTF-8 boundary.
    /// - `(None, input)` otherwise.
    fn apply<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, usize), RuleError> {
        input.get(..N).map_or_else(
            || {
                Err(RuleError {
                    reason: "input too short or invalid UTF-8 boundary.".into(),
                })
            },
            |out| Ok((out, N)),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_rule;

    test_rule!(ascii_count_exact_length, "test", ByteCount::<4, true>);
    test_rule!(ascii_count_less_than_length, "hello", ByteCount::<2, true>);
    test_rule!(ascii_count_more_than_length, "short", ByteCount::<10, true>);
    test_rule!(ascii_count_zero, "abc", ByteCount::<0, true>);
    test_rule!(ascii_count_empty_input, "", ByteCount::<0, true>);
    test_rule!(utf8_valid_boundary, "你好世界", ByteCount::<3, false>);
    test_rule!(utf8_invalid_boundary, "你好世界", ByteCount::<2, false>);
}
