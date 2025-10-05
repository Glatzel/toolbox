use core::fmt::{self, Debug, Display};

use super::IStrFlowRule;
use crate::str_parser::IRule;
use crate::str_parser::filters::{CharSetFilter, IFilter};

/// Rule that matches if the first `N` characters of the input are all in a
/// specified character set.
///
/// `NInCharSet<'a, N, M>` takes a reference to a [`CharSetFilter<M>`] and
/// checks the first `N` characters of the input string. If all `N` characters
/// are present in the character set, it returns a tuple `(Some(matched), rest)`
/// where `matched` is the substring of the first `N` characters and `rest` is
/// the remainder of the input. Otherwise, it returns `(None, input)`.
///
/// This rule respects UTF-8 boundaries and stops immediately on the first
/// character that does not belong to the set, or if the input is too short.
///
/// # Type Parameters
///
/// - `'a`: Lifetime of the character set reference.
/// - `N`: Number of characters to match at the start of the input.
/// - `M`: Size of the character set (length of the `CharSetFilter`).
pub struct NInCharSet<'a, const N: usize, const M: usize>(pub &'a CharSetFilter<M>);

impl<'a, const N: usize, const M: usize> Debug for NInCharSet<'a, N, M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NInCharSet<N={}, M={}>", N, M)
    }
}

impl<'a, const N: usize, const M: usize> Display for NInCharSet<'a, N, M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{:?}", self) }
}

impl<'a, const N: usize, const M: usize> IRule for NInCharSet<'a, N, M> {}

impl<'a, const N: usize, const M: usize> IStrFlowRule<'a> for NInCharSet<'a, N, M> {
    type Output = &'a str;

    /// Applies the `NInCharSet` rule to the input string.
    ///
    /// # Returns
    ///
    /// - `(Some(matched), rest)` if the first `N` characters are all in the
    ///   character set.
    /// - `(None, input)` if a character is not in the set before reaching `N`,
    ///   or if the input has fewer than `N` characters.
    ///
    /// # Logging
    ///
    /// - Debug-level logs indicate matches, unmatched characters, and
    ///   insufficient input.
    fn apply(&self, input: &'a str) -> (Option<&'a str>, &'a str) {
        let mut count = 0;
        for (i, c) in input.char_indices() {
            if self.0.filter(&c) {
                count += 1;
                let end_idx = i + c.len_utf8();
                if count == N {
                    let matched = &input[..end_idx];
                    let rest = &input[end_idx..];
                    clerk::debug!("{} matched: '{}', rest='{}'", self, matched, rest);
                    return (Some(matched), rest);
                }
            } else {
                clerk::debug!(
                    "{} did not match: char '{}' not in set at pos {}",
                    self,
                    c,
                    i
                );
                return (None, input);
            }
        }

        clerk::debug!(
            "{} did not match: input too short or not enough chars in set",
            self
        );
        (None, input)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    extern crate std;
    use clerk::{LogLevel, init_log_with_level};

    use super::*;
    use crate::str_parser::filters::{ASCII_LETTERS_DIGITS, DIGITS};

    #[test]
    fn test_n_in_charset_match() {
        init_log_with_level(LogLevel::TRACE);
        let rule = NInCharSet::<3, 62>(&ASCII_LETTERS_DIGITS);
        let input = "abc123";
        let (matched, rest) = rule.apply(input);
        assert_eq!(matched, Some("abc"));
        assert_eq!(rest, "123");
    }

    #[test]
    fn test_n_in_charset_no_match() {
        init_log_with_level(LogLevel::TRACE);
        let rule = NInCharSet::<3, 10>(&DIGITS);
        let input = "12abc";
        let (matched, rest) = rule.apply(input);
        assert_eq!(matched, None);
        assert_eq!(rest, "12abc");
    }

    #[test]
    fn test_n_in_charset_too_short() {
        init_log_with_level(LogLevel::TRACE);
        let rule = NInCharSet::<4, 62>(&ASCII_LETTERS_DIGITS);
        let input = "ab";
        let (matched, rest) = rule.apply(input);
        assert_eq!(matched, None);
        assert_eq!(rest, "ab");
    }

    #[test]
    fn test_n_in_charset_empty_input() {
        init_log_with_level(LogLevel::TRACE);
        let rule = NInCharSet::<1, 62>(&ASCII_LETTERS_DIGITS);
        let input = "";
        let (matched, rest) = rule.apply(input);
        assert_eq!(matched, None);
        assert_eq!(rest, "");
    }

    #[test]
    fn test_n_in_charset_unicode() -> mischief::Result<()> {
        init_log_with_level(LogLevel::TRACE);
        let filter: CharSetFilter<2> = CharSetFilter::from_str("你好")?;
        let rule = NInCharSet::<2, 2>(&filter);
        let input = "你好世界";
        let (matched, rest) = rule.apply(input);
        assert_eq!(matched, Some("你好"));
        assert_eq!(rest, "世界");
        Ok(())
    }
}
