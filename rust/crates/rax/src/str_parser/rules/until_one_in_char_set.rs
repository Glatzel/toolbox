use super::IStrFlowRule;
use crate::str_parser::IRule;
use crate::str_parser::filters::{CharSetFilter, IFilter};
use crate::str_parser::rules::UntilMode;

/// Rule to extract everything from the input string up to (but not including)
/// the first occurrence of any character in the provided character set.
/// Returns a tuple of (prefix, rest) where `prefix` contains all characters
/// up to the first character in the set, and `rest` is the remainder of the
/// string starting from that character. If no character in the set is found,
/// returns (None, input).
pub struct UntilOneInCharSet<'a, const N: usize> {
    pub filter: &'a CharSetFilter<N>,
    pub mode: super::UntilMode,
}

impl<'a, const N: usize> IRule for UntilOneInCharSet<'a, N> {
    fn name(&self) -> &str { "UntilOneInCharSet" }
}

impl<'a, const N: usize> IStrFlowRule<'a> for UntilOneInCharSet<'a, N> {
    type Output = &'a str;

    /// Applies the UntilOneInCharSet rule to the input string.
    /// If a character in the set is found, returns the substring before the
    /// character and the rest of the string (starting with the character).
    /// If `include` is true, the matched character is included in the prefix.
    /// If `include` is false and the first character is in the set, returns
    /// None. If no character in the set is found, returns None and the
    /// original input.
    fn apply(&self, input: &'a str) -> (Option<&'a str>, &'a str) {
        for (i, c) in input.char_indices() {
            if self.filter.filter(&c) {
                match self.mode {
                    UntilMode::Discard => {
                        // Include the matched character in the prefix
                        let prefix = &input[..i];
                        let rest = &input[i + c.len_utf8()..];
                        clerk::debug!(
                            "UntilOneInCharSet(include): prefix='{}', rest='{}', i={}, c='{}'",
                            prefix,
                            rest,
                            i,
                            c
                        );
                        return (Some(prefix), rest);
                    }
                    UntilMode::KeepLeft => {
                        // Include the matched character in the prefix
                        let end_of_char = i + c.len_utf8();
                        let prefix = &input[..end_of_char];
                        let rest = &input[end_of_char..];
                        clerk::debug!(
                            "UntilOneInCharSet(include): prefix='{}', rest='{}', i={}, c='{}'",
                            prefix,
                            rest,
                            i,
                            c
                        );
                        return (Some(prefix), rest);
                    }

                    UntilMode::KeepRight => {
                        // Not include, and not first char
                        let prefix = &input[..i];
                        let rest = &input[i..];
                        clerk::debug!(
                            "UntilOneInCharSet(not include): prefix='{}', rest='{}', i={}, c='{}'",
                            prefix,
                            rest,
                            i,
                            c
                        );
                        return (Some(prefix), rest);
                    }
                }
            }
        }
        // No character in the set found
        clerk::debug!(
            "UntilOneInCharSet: no match found, returning None, input='{}'",
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
