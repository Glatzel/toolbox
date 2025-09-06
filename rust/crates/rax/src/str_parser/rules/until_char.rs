use super::IStrFlowRule;
use crate::str_parser::IRule;
use crate::str_parser::rules::UntilMode;

pub struct UntilChar<const C: char> {
    pub mode: super::UntilMode,
}

impl<const C: char> IRule for UntilChar<C> {
    fn name(&self) -> &str { "Until" }
}

impl<'a, const C: char> IStrFlowRule<'a> for UntilChar<C> {
    type Output = &'a str;
    /// Applies the Until rule to the input string.
    /// If the delimiter is found, returns the substring before the delimiter
    /// and the rest of the string (starting with the delimiter).
    /// If `include` is true, the delimiter is included in the prefix.
    /// Otherwise, returns None.
    fn apply(&self, input: &'a str) -> (Option<&'a str>, &'a str) {
        // Log the input and delimiter at trace level.
        clerk::trace!(
            "Until Char rule: input='{}', char='{}', mode={}",
            input,
            C,
            self.mode
        );
        for (i, c) in input.char_indices() {
            if c == C {
                match self.mode {
                    UntilMode::Discard => {
                        let end = i + C.len_utf8();
                        clerk::debug!(
                            "Until rule matched (include): prefix='{}', rest='{}'",
                            &input[..i],
                            &input[end..]
                        );
                        return (Some(&input[..i]), &input[end..]);
                    }
                    UntilMode::KeepLeft => {
                        let end = i + C.len_utf8();
                        clerk::debug!(
                            "Until rule matched (include): prefix='{}', rest='{}'",
                            &input[..end],
                            &input[end..]
                        );
                        return (Some(&input[..end]), &input[end..]);
                    }
                    UntilMode::KeepRight => {
                        clerk::debug!(
                            "Until rule matched (include): prefix='{}', rest='{}'",
                            &input[..i],
                            &input[i..]
                        );
                        return (Some(&input[..i]), &input[i..]);
                    }
                }
            }
        }
        (None, input)
    }
}

#[cfg(test)]
mod tests {
    use clerk::{LogLevel, init_log_with_level};
    extern crate std;
    use super::*;

    #[test]
    fn test_until_basic_not_include() {
        init_log_with_level(LogLevel::TRACE);
        let rule = UntilChar::<';'> {
            mode: super::UntilMode::Discard,
        };
        let input = "abc;def";
        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, Some("abc"));
        assert_eq!(rest, "def");
    }

    #[test]
    fn test_until_basic_include() {
        init_log_with_level(LogLevel::TRACE);
        let rule = UntilChar::<';'> {
            mode: super::UntilMode::KeepLeft,
        };
        let input = "abc;def";
        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, Some("abc;"));
        assert_eq!(rest, "def");
    }

    #[test]
    fn test_until_keep_right() {
        init_log_with_level(LogLevel::TRACE);
        let rule = UntilChar::<';'> {
            mode: super::UntilMode::KeepRight,
        };
        let input = "abc;def";
        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, Some("abc"));
        assert_eq!(rest, ";def");
    }
    #[test]
    fn test_until_first() {
        init_log_with_level(LogLevel::TRACE);
        let rule = UntilChar::<';'> {
            mode: super::UntilMode::Discard,
        };
        let input = ";abcdef";
        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, Some(""));
        assert_eq!(rest, "abcdef");
    }
    #[test]
    fn test_until_no_delimiter() {
        init_log_with_level(LogLevel::TRACE);
        let rule = UntilChar::<';'> {
            mode: super::UntilMode::Discard,
        };
        let input = "abcdef";
        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, None);
        assert_eq!(rest, "abcdef");
    }

    #[test]
    fn test_until_delimiter_at_start() {
        init_log_with_level(LogLevel::TRACE);
        let rule = UntilChar::<';'> {
            mode: super::UntilMode::KeepLeft,
        };
        let input = ";abcdef";
        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, Some(";"));
        assert_eq!(rest, "abcdef");
    }

    #[test]
    fn test_until_empty_input() {
        init_log_with_level(LogLevel::TRACE);
        let rule = UntilChar::<';'> {
            mode: super::UntilMode::Discard,
        };
        let input = "";
        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, None);
        assert_eq!(rest, "");
    }
}
