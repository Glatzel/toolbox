use super::IStrFlowRule;
use crate::str_parser::IRule;
use crate::str_parser::filters::{CharSetFilter, IFilter};
use crate::str_parser::rules::UntilMode;

/// Rule that extracts a prefix from the input string up to the first occurrence
/// of any character in the provided character set.
///
/// # Fields
///
/// - `filter`: A [`CharSetFilter`] defining the set of characters to stop at.
/// - `mode`: Determines how the matched character is treated:
///   - [`UntilMode::Discard`]: Exclude the matched character from the prefix
///     and remove it from the rest.
///   - [`UntilMode::KeepLeft`]: Include the matched character in the prefix.
///   - [`UntilMode::KeepRight`]: Keep the matched character at the start of the
///     rest.
///
/// # Behavior
///
/// - Returns `(Some(prefix), rest)` when a character from the set is found,
///   split according to `mode`.
/// - Returns `(None, input)` if no character from the set is found.
/// - Respects UTF-8 character boundaries.
/// - Logs debug information for each split or if no match is found.
pub struct UntilOneInCharSet<'a, const N: usize> {
    pub filter: &'a CharSetFilter<N>,
    pub mode: UntilMode,
}

impl<'a, const N: usize> core::fmt::Debug for UntilOneInCharSet<'a, N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "UntilOneInCharSet<N={}> {{ mode: {:?} }}", N, self.mode)
    }
}

impl<'a, const N: usize> core::fmt::Display for UntilOneInCharSet<'a, N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { write!(f, "{:?}", self) }
}

impl<'a, const N: usize> IRule for UntilOneInCharSet<'a, N> {}

impl<'a, const N: usize> IStrFlowRule<'a> for UntilOneInCharSet<'a, N> {
    type Output = &'a str;

    fn apply(&self, input: &'a str) -> (Option<&'a str>, &'a str) {
        for (i, c) in input.char_indices() {
            if self.filter.filter(&c) {
                let (prefix, rest) = match self.mode {
                    UntilMode::Discard => (&input[..i], &input[i + c.len_utf8()..]),
                    UntilMode::KeepLeft => {
                        let end = i + c.len_utf8();
                        (&input[..end], &input[end..])
                    }
                    UntilMode::KeepRight => (&input[..i], &input[i..]),
                };
                clerk::debug!(
                    "{}: prefix='{}', rest='{}', i={}, c='{}'",
                    self,
                    prefix,
                    rest,
                    i,
                    c
                );
                return (Some(prefix), rest);
            }
        }

        clerk::debug!(
            "{}: no match found, returning None, input='{}'",
            self,
            input
        );
        (None, input)
    }
}

#[cfg(test)]
mod tests {
    use clerk::{LogLevel, init_log_with_level};
    extern crate std;
    use super::*;
    use crate::str_parser::filters::{ASCII_LETTERS, DIGITS};

    #[test]
    fn test_until_one_in_char_set_discard() {
        init_log_with_level(LogLevel::TRACE);
        let rule = UntilOneInCharSet {
            filter: &DIGITS,
            mode: UntilMode::Discard,
        };
        let input = "abc1def";
        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, Some("abc"));
        assert_eq!(rest, "def");
    }

    #[test]
    fn test_until_one_in_char_set_keep_left() {
        init_log_with_level(LogLevel::TRACE);
        let rule = UntilOneInCharSet {
            filter: &DIGITS,
            mode: UntilMode::KeepLeft,
        };
        let input = "abc1def";
        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, Some("abc1"));
        assert_eq!(rest, "def");
    }

    #[test]
    fn test_until_one_in_char_set_keep_right_first_char() {
        init_log_with_level(LogLevel::TRACE);
        let rule = UntilOneInCharSet {
            filter: &ASCII_LETTERS,
            mode: UntilMode::KeepRight,
        };
        let input = "a123";
        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, Some(""));
        assert_eq!(rest, "a123");
    }

    #[test]
    fn test_until_one_in_char_set_keep_right_not_first_char() {
        init_log_with_level(LogLevel::TRACE);
        let rule = UntilOneInCharSet {
            filter: &DIGITS,
            mode: UntilMode::KeepRight,
        };
        let input = "abc1def";
        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, Some("abc"));
        assert_eq!(rest, "1def");
    }

    #[test]
    fn test_until_one_in_char_set_no_match() {
        init_log_with_level(LogLevel::TRACE);
        let rule = UntilOneInCharSet {
            filter: &DIGITS,
            mode: UntilMode::Discard,
        };
        let input = "abcdef";
        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, None);
        assert_eq!(rest, "abcdef");
    }

    #[test]
    fn test_until_one_in_char_set_empty_input() {
        init_log_with_level(LogLevel::TRACE);
        let rule = UntilOneInCharSet {
            filter: &DIGITS,
            mode: UntilMode::Discard,
        };
        let input = "";
        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, None);
        assert_eq!(rest, "");
    }
}
