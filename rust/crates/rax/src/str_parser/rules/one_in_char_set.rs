use core::fmt::{self, Debug, Display};

use super::IStrFlowRule;
use crate::str_parser::IRule;
use crate::str_parser::filters::{CharSetFilter, IFilter};

/// Rule that matches the first character of the input string if it belongs to
/// a specified character set.
///
/// `OneOfCharSet<'a, N>` takes a reference to a [`CharSetFilter<N>`] and checks
/// the first character of the input. If the first character is in the set, it
/// returns a tuple `(Some(matched), rest)` where `matched` is the character and
/// `rest` is the remainder of the input. Otherwise, it returns `(None, input)`.
///
/// This rule respects UTF-8 boundaries and stops immediately on the first
/// character if it is not in the set, or if the input is empty.
///
/// # Type Parameters
///
/// - `'a`: Lifetime of the character set reference.
/// - `N`: Size of the character set (length of the `CharSetFilter`).
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct OneOfCharSet<'a, const N: usize>(pub &'a CharSetFilter<N>);

impl<'a, const N: usize> Debug for OneOfCharSet<'a, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "OneOfCharSet<N={}>", N) }
}

impl<'a, const N: usize> Display for OneOfCharSet<'a, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{:?}", self) }
}

impl<'a, const N: usize> IRule for OneOfCharSet<'a, N> {}

impl<'a, const N: usize> IStrFlowRule<'a> for OneOfCharSet<'a, N> {
    type Output = char;

    /// Applies the `OneOfCharSet` rule to the input string.
    ///
    /// # Returns
    ///
    /// - `(Some(matched), rest)` if the first character is in the character
    ///   set.
    /// - `(None, input)` if the first character is not in the set or the input
    ///   is empty.
    ///
    /// # Logging
    ///
    /// - Trace-level logs show the input string.
    /// - Debug-level logs indicate matches or mismatches.
    fn apply(&self, input: &'a str) -> (Option<char>, &'a str) {
        clerk::trace!("OneOfCharSet rule: input='{}'", input);

        if let Some((_, c)) = input.char_indices().next() {
            if self.0.filter(&c) {
                let next_i = input.char_indices().nth(1).map_or(input.len(), |(j, _)| j);
                clerk::debug!("OneOfCharSet matched: '{}', rest='{}'", c, &input[next_i..]);
                (Some(c), &input[next_i..])
            } else {
                clerk::debug!("OneOfCharSet did not match: found '{}', not in set", c);
                (None, input)
            }
        } else {
            (None, input)
        }
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
    fn test_char_match() {
        init_log_with_level(LogLevel::TRACE);
        let rule = OneOfCharSet(&ASCII_LETTERS_DIGITS);
        let input = "a123";
        let (matched, rest) = rule.apply(input);
        assert_eq!(matched, Some('a'));
        assert_eq!(rest, "123");
    }

    #[test]
    fn test_char_no_match() {
        init_log_with_level(LogLevel::TRACE);
        let rule = OneOfCharSet(&DIGITS);
        let input = "abc";
        let (matched, rest) = rule.apply(input);
        assert_eq!(matched, None);
        assert_eq!(rest, "abc");
    }

    #[test]
    fn test_char_empty_input() {
        init_log_with_level(LogLevel::TRACE);
        let rule = OneOfCharSet(&ASCII_LETTERS_DIGITS);
        let input = "";
        let (matched, rest) = rule.apply(input);
        assert_eq!(matched, None);
        assert_eq!(rest, "");
    }

    #[test]
    fn test_char_unicode() -> mischief::Result<()> {
        init_log_with_level(LogLevel::TRACE);
        let filter: CharSetFilter<1> = CharSetFilter::from_str("你")?;
        let rule = OneOfCharSet(&filter);
        let input = "你好";
        let (matched, rest) = rule.apply(input);
        assert_eq!(matched, Some('你'));
        assert_eq!(rest, "好");
        Ok(())
    }
}
