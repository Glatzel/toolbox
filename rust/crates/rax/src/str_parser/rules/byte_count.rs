use core::fmt::{self, Debug, Display};

use super::IStrFlowRule;
use crate::str_parser::rules::IRule;

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
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ByteCount<const N: usize>;

impl<const N: usize> Debug for ByteCount<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "ByteCount<{}>", N) }
}

impl<const N: usize> Display for ByteCount<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{:?}", self) }
}

impl<const N: usize> IRule for ByteCount<N> {}

impl<'a, const N: usize> IStrFlowRule<'a> for ByteCount<N> {
    type Output = &'a str;

    /// Applies the `ByteCount` rule to the input string.
    ///
    /// # Returns
    ///
    /// - `(Some(prefix), rest)` if the input contains at least `N` bytes and
    ///   the split occurs on a valid UTF-8 boundary.
    /// - `(None, input)` otherwise.
    ///
    /// # Logging
    ///
    /// Logs trace messages showing the input and requested byte count,
    /// and debug messages showing the matched prefix and remaining input.
    fn apply(&self, input: &'a str) -> (Option<&'a str>, &'a str) {
        // Trace input and requested byte count
        clerk::trace!("{}: input='{}', byte_count={}", self, input, N);

        match input.get(..N) {
            Some(out) => {
                let rest = &input[N..];
                clerk::debug!("{}: matched prefix='{}', rest='{}'", self, out, rest);
                (Some(out), rest)
            }
            None => {
                clerk::debug!(
                    "{}: not enough bytes or invalid UTF-8 boundary for count {} in '{}'",
                    self,
                    N,
                    input
                );
                (None, input)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use clerk::{LogLevel, init_log_with_level};

    use super::*;

    #[test]
    fn test_count_exact_length() {
        init_log_with_level(LogLevel::TRACE);
        let rule = ByteCount::<4>;
        let input = "test";
        let result = rule.apply(input);
        assert_eq!(result, (Some("test"), ""));
    }

    #[test]
    fn test_count_less_than_length() {
        init_log_with_level(LogLevel::TRACE);
        let rule = ByteCount::<2>;
        let input = "hello";
        let result = rule.apply(input);
        assert_eq!(result, (Some("he"), "llo"));
    }

    #[test]
    fn test_count_more_than_length() {
        init_log_with_level(LogLevel::TRACE);
        let rule = ByteCount::<10>;
        let input = "short";
        let result = rule.apply(input);
        assert_eq!(result, (None, "short"));
    }

    #[test]
    fn test_count_zero() {
        init_log_with_level(LogLevel::TRACE);
        let rule = ByteCount::<0>;
        let input = "abc";
        let result = rule.apply(input);
        assert_eq!(result, (Some(""), "abc"));
    }

    #[test]
    fn test_count_empty_input() {
        init_log_with_level(LogLevel::TRACE);
        let rule = ByteCount::<0>;
        let input = "";
        let result = rule.apply(input);
        assert_eq!(result, (Some(""), ""));
    }

    #[test]
    fn test_count_non_ascii() {
        init_log_with_level(LogLevel::TRACE);
        let rule = ByteCount::<2>;
        let input = "你好世界";

        // Each Chinese character is 3 bytes, but .get(..n) is by byte index, not char
        // index. So Count(2) will get the first 2 bytes, which is not a valid
        // UTF-8 boundary. This should return None.
        let result = rule.apply(input);
        assert_eq!(result, (None, "你好世界"));
    }
}
