use core::fmt::Debug;

use super::IStrFlowRule;
use crate::error::RuleError;
use crate::string::ByteCount;
use crate::string::rules::IRule;

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
pub struct CharCount<const N: usize>;

impl<const N: usize> IRule for CharCount<N> {}

impl<'a, const N: usize> IStrFlowRule<'a> for CharCount<N> {
    type Output = &'a str;

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
    fn apply(&self, input: &'a str, is_ascii: bool) -> Result<(Self::Output, usize), RuleError> {
        if N == 0 {
            clerk::warn!(
                "{:?}: count is zero, returning empty prefix and full input.",
                self
            );

            return Ok(("", 0));
        }
        clerk::trace!("{:?}: input='{:?}', count={:?}", self, input, N);
        if is_ascii {
            return ByteCount::<N>.apply(input, is_ascii);
        }
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

#[cfg(test)]
mod tests {
    extern crate std;

    use core::marker::PhantomData;
    use std::format;

    use clerk::{LevelFilter, init_log_with_level};

    use super::*;

    #[rstest::rstest]
    #[case("ascii_exact_length","test", PhantomData::<CharCount<4>>)]
    #[case("ascii_less_than_length","hello", PhantomData::<CharCount<2>>)]
    #[case("ascii_more_than_length","short", PhantomData::<CharCount<10>>)]
    #[case("ascii_zero","abc", PhantomData::<CharCount<0>>)]
    #[case("ascii_empty_input","", PhantomData::<CharCount<0>>)]
    #[case("utf8_less_than_length","你好世界", PhantomData::<CharCount<2>>)]
    fn test_char_count<const C: usize>(
        #[case] name: &str,
        #[case] input: &str,
        #[case] _rule: PhantomData<CharCount<C>>,
    ) {
        init_log_with_level(LevelFilter::TRACE);
        let result = CharCount::<C>
            .apply(input, input.is_ascii())
            .map(|(out, idx)| (out, input.get(idx..).unwrap()));
        insta::assert_debug_snapshot!(format!("{}", name), result);
    }
}
