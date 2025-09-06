use super::IStrFlowRule;
use crate::str_parser::IRule;
use crate::str_parser::filters::{CharSetFilter, IFilter};
use crate::str_parser::rules::UntilMode;

/// Rule that extracts a prefix from the input string up to (but not including)
/// the first character that is NOT in the provided character set filter.
/// Returns a tuple of (prefix, rest) where `prefix` contains all consecutive
/// characters from the start of the input that are in the set, and `rest` is
/// the remainder of the string starting from the first character not in the
/// set. If all characters are in the set, returns (None, input).
/// If `include` is true, the first character not in the set is included in the
/// prefix.
pub struct UntilNotInCharSet<'a, const N: usize> {
    pub filter: &'a CharSetFilter<N>,
    pub mode: super::UntilMode,
}

impl<'a, const N: usize> IRule for UntilNotInCharSet<'a, N> {
    fn name(&self) -> &str { "UntilNotInCharSet" }
}

impl<'a, const N: usize> IStrFlowRule<'a> for UntilNotInCharSet<'a, N> {
    type Output = &'a str;

    /// Applies the rule to the input string, returning the prefix of characters
    /// in the set and the rest of the string starting from the first character
    /// not in the set. If all characters are in the set, returns (None, input).
    /// If `include` is true, the first character not in the set is included in
    /// the prefix.
    fn apply(&self, input: &'a str) -> (Option<&'a str>, &'a str) {
        // Iterate over each character and its byte index in the input string
        for (i, c) in input.char_indices() {
            // If the character is NOT in the set, split here
            if !self.filter.filter(&c) {
                match self.mode {
                    UntilMode::Discard => {
                        let prefix = &input[..i];
                        let rest = &input[i + c.len_utf8()..];
                        clerk::debug!(
                            "UntilNotInCharSet(include): prefix='{}', rest='{}', i={}, c='{}'",
                            prefix,
                            rest,
                            i,
                            c
                        );
                        return (Some(prefix), rest);
                    }
                    UntilMode::KeepLeft => {
                        let prefix = &input[..i + c.len_utf8()];
                        let rest = &input[i + c.len_utf8()..];
                        clerk::debug!(
                            "UntilNotInCharSet(include): prefix='{}', rest='{}', i={}, c='{}'",
                            prefix,
                            rest,
                            i,
                            c
                        );
                        return (Some(prefix), rest);
                    }
                    UntilMode::KeepRight => {
                        let prefix = &input[..i];
                        let rest = &input[i..];
                        clerk::debug!(
                            "UntilNotInCharSet(not include): prefix='{}', rest='{}', i={}, c='{}'",
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
        // If all characters are in the set, return None and the original input
        clerk::debug!(
            "UntilNotInCharSet: all characters in set, returning None, input='{}'",
            input
        );
        (None, input)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use clerk::{LogLevel, init_log_with_level};

    use super::*;
    use crate::str_parser::filters::DIGITS;

    #[test]
    fn test_until_not_in_char_set_discard() {
        init_log_with_level(LogLevel::TRACE);
        let rule = UntilNotInCharSet {
            filter: &DIGITS,
            mode: UntilMode::Discard,
        };
        let input = "123abc";
        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, Some("123"));
        assert_eq!(rest, "bc");
    }

    #[test]
    fn test_until_not_in_char_set_keep_left() {
        init_log_with_level(LogLevel::TRACE);
        let rule = UntilNotInCharSet {
            filter: &DIGITS,
            mode: UntilMode::KeepLeft,
        };
        let input = "123abc";
        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, Some("123a"));
        assert_eq!(rest, "bc");
    }

    #[test]
    fn test_until_not_in_char_set_keep_right() {
        init_log_with_level(LogLevel::TRACE);
        let rule = UntilNotInCharSet {
            filter: &DIGITS,
            mode: UntilMode::KeepRight,
        };
        let input = "123abc";
        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, Some("123"));
        assert_eq!(rest, "abc");
    }

    #[test]
    fn test_until_not_in_char_set_all_in_set() {
        init_log_with_level(LogLevel::TRACE);
        let rule = UntilNotInCharSet {
            filter: &DIGITS,
            mode: UntilMode::Discard,
        };
        let input = "123456";
        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, None);
        assert_eq!(rest, "123456");
    }

    #[test]
    fn test_until_not_in_char_set_first_char_not_in_set() {
        init_log_with_level(LogLevel::TRACE);
        let rule = UntilNotInCharSet {
            filter: &DIGITS,
            mode: UntilMode::KeepLeft,
        };
        let input = "a123";
        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, Some("a"));
        assert_eq!(rest, "123");
    }

    #[test]
    fn test_until_not_in_char_set_empty_input() {
        init_log_with_level(LogLevel::TRACE);
        let rule = UntilNotInCharSet {
            filter: &DIGITS,
            mode: UntilMode::Discard,
        };
        let input = "";
        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, None);
        assert_eq!(rest, "");
    }

    #[test]
    fn test_until_not_in_char_set_unicode() -> mischief::Result<()> {
        init_log_with_level(LogLevel::TRACE);
        let filter: CharSetFilter<2> = CharSetFilter::from_str("好你")?;
        let rule = UntilNotInCharSet {
            filter: &filter,
            mode: UntilMode::KeepLeft,
        };
        let input = "你好世界";
        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, Some("你好世"));
        assert_eq!(rest, "界");
        Ok(())
    }
}
