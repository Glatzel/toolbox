use core::fmt::Debug;

use super::IStrFlowRule;
use crate::string::rules::IRule;

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
pub struct ByteCount<const N: usize>;

impl<const N: usize> IRule for ByteCount<N> {}

impl<'a, const N: usize> IStrFlowRule<'a> for ByteCount<N> {
    type Output = &'a str;
    type Error = &'static str;

    /// Applies the `ByteCount` rule to the input string.
    ///
    /// # Returns
    ///
    /// - `(Some(prefix), rest)` if the input contains at least `N` bytes and
    ///   the split occurs on a valid UTF-8 boundary.
    /// - `(None, input)` otherwise.
    fn apply(&self, input: &'a str) -> Result<(Self::Output, &'a str), Self::Error> {
        // Trace input and requested byte count
        clerk::trace!("{:?}: input='{}', byte_count={}", self, input, N);

        match input.get(..N) {
            Some(out) => {
                let rest = &input[N..];
                clerk::debug!("{:?}: matched prefix='{}', rest='{}'", self, out, rest);
                Ok((out, rest))
            }
            None => {
                clerk::debug!(
                    "{:?}: not enough bytes or invalid UTF-8 boundary for count {} in '{}'",
                    self,
                    N,
                    input
                );
                Err("input too short or invalid UTF-8 boundary.")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    #[cfg(test)]
    use core::marker::PhantomData;
    use std::format;

    use clerk::{LevelFilter, init_log_with_level};

    use super::*;

    #[rstest::rstest]
    #[case("count_exact_length","test", PhantomData::<ByteCount<4>>)]
    #[case("count_less_than_length","hello", PhantomData::<ByteCount<2>>)]
    #[case("count_more_than_length","short", PhantomData::<ByteCount<10>>)]
    #[case("count_zero","abc", PhantomData::<ByteCount<0>>)]
    #[case("count_empty_input","", PhantomData::<ByteCount<0>>)]
    #[case("valid_utf8_boundary","你好世界", PhantomData::<ByteCount< 3>>)]
    #[case("invalid_utf8_boundary","你好世界", PhantomData::<ByteCount<2>>)]
    fn test_byte_count<const N: usize>(
        #[case] name: &str,
        #[case] input: &str,
        #[case] _rule: PhantomData<ByteCount<N>>,
    ) {
        init_log_with_level(LevelFilter::TRACE);
        let result = ByteCount::<N>.apply(input);
        insta::assert_debug_snapshot!(format!("{}", name), result);
    }
}
