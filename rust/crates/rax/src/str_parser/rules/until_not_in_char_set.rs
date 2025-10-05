use super::IStrFlowRule;
use crate::str_parser::IRule;
use crate::str_parser::filters::{CharSetFilter, IFilter};
use crate::str_parser::rules::UntilMode;

/// Rule that extracts a prefix from the input string consisting of consecutive
/// characters that are in the provided character set, stopping at the first
/// character not in the set.
///
/// # Fields
///
/// - `filter`: A [`CharSetFilter`] that defines the allowed characters.
/// - `mode`: Determines how the first character *not* in the set is treated:
///   - [`UntilMode::Discard`]: Exclude the first non-matching character from
///     the prefix and remove it from the rest.
///   - [`UntilMode::KeepLeft`]: Include the first non-matching character at the
///     end of the prefix.
///   - [`UntilMode::KeepRight`]: Keep the first non-matching character at the
///     start of the rest.
///
/// # Behavior
///
/// - Returns `(Some(prefix), rest)` when a non-matching character is found,
///   split according to `mode`.
/// - Returns `(None, input)` if all characters in the input are in the set.
/// - Respects UTF-8 character boundaries.
/// - Logs debug information at each split or if all characters are in the set.
pub struct UntilNotInCharSet<'a, const N: usize> {
    pub filter: &'a CharSetFilter<N>,
    pub mode: UntilMode,
}

impl<'a, const N: usize> core::fmt::Debug for UntilNotInCharSet<'a, N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "UntilNotInCharSet<N={}> {{ mode: {:?} }}", N, self.mode)
    }
}

impl<'a, const N: usize> core::fmt::Display for UntilNotInCharSet<'a, N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { write!(f, "{:?}", self) }
}

impl<'a, const N: usize> IRule for UntilNotInCharSet<'a, N> {}

impl<'a, const N: usize> IStrFlowRule<'a> for UntilNotInCharSet<'a, N> {
    type Output = &'a str;

    fn apply(&self, input: &'a str) -> (Option<&'a str>, &'a str) {
        for (i, c) in input.char_indices() {
            if !self.filter.filter(&c) {
                let (prefix, rest) = match self.mode {
                    UntilMode::Discard => (&input[..i], &input[i + c.len_utf8()..]),
                    UntilMode::KeepLeft => (&input[..i + c.len_utf8()], &input[i + c.len_utf8()..]),
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
            "{}: all characters in set, returning None, input='{}'",
            self,
            input
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
