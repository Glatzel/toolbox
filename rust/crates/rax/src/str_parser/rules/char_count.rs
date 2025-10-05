use core::fmt::{self, Debug, Display};

use super::IStrFlowRule;
use crate::str_parser::rules::IRule;

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
pub struct CharCount<const N: usize>;

impl<const N: usize> Debug for CharCount<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "CharCount<{}>", N) }
}

impl<const N: usize> Display for CharCount<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{:?}", self) }
}

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
    fn apply(&self, input: &'a str) -> (Option<&'a str>, &'a str) {
        // Trace input and requested character count
        clerk::trace!("{}: input='{}', count={}", self, input, N);

        if N == 0 {
            clerk::debug!(
                "{}: count is zero, returning empty prefix and full input.",
                self
            );
            return (Some(""), input);
        }

        let length = input.chars().count();

        if N == length {
            clerk::debug!(
                "{}: count matches input length, returning whole input.",
                self
            );
            return (Some(input), "");
        }

        for (count, (idx, _)) in input.char_indices().enumerate() {
            if count == N {
                clerk::debug!(
                    "{}: found split at char {}, byte idx {}: prefix='{}', rest='{}'",
                    self,
                    count,
                    idx,
                    &input[..idx],
                    &input[idx..]
                );
                return (Some(&input[..idx]), &input[idx..]);
            }
        }

        clerk::warn!(
            "CharCount: not enough chars in input (needed {}, found {})",
            N,
            length
        );
        (None, input)
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
        let rule = CharCount::<4>;
        let input = "test";
        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, Some("test"));
        assert_eq!(rest, "");
    }

    #[test]
    fn test_count_less_than_length() {
        init_log_with_level(LogLevel::TRACE);
        let rule = CharCount::<2>;
        let input = "hello";
        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, Some("he"));
        assert_eq!(rest, "llo");
    }

    #[test]
    fn test_count_more_than_length() {
        init_log_with_level(LogLevel::TRACE);
        let rule = CharCount::<10>;
        let input = "short";
        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, None);
        assert_eq!(rest, "short");
    }

    #[test]
    fn test_count_zero() {
        init_log_with_level(LogLevel::TRACE);
        let rule = CharCount::<0>;
        let input = "abc";
        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, Some(""));
        assert_eq!(rest, "abc");
    }

    #[test]
    fn test_count_empty_input() {
        init_log_with_level(LogLevel::TRACE);
        let rule = CharCount::<0>;
        let input = "";
        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, Some(""));
        assert_eq!(rest, "");
    }

    #[test]
    fn test_count_non_ascii() {
        init_log_with_level(LogLevel::TRACE);
        let rule = CharCount::<2>;
        let input = "你好世界";
        // Should return first 2 chars ("你", "好") and the rest ("世界")
        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, Some("你好"));
        assert_eq!(rest, "世界");
    }
}
