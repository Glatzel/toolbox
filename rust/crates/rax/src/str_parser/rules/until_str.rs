use super::IStrFlowRule;
use crate::str_parser::IRule;
use crate::str_parser::rules::UntilMode;

/// Rule to extract everything from the input string up to (but not including)
/// the first occurrence of a specified delimiter substring.
/// Returns a tuple of (prefix, rest) if the delimiter is found,
/// otherwise returns None.
/// If `include` is true, the delimiter is included in the prefix.
pub struct UntilStr {
    pub pattern: &'static str,
    pub mode: super::UntilMode,
}

impl IRule for UntilStr {
    fn name(&self) -> &str { "Until" }
}

impl<'a> IStrFlowRule<'a> for UntilStr {
    type Output = &'a str;
    /// Applies the Until rule to the input string.
    /// If the delimiter is found, returns the substring before the delimiter
    /// and the rest of the string (starting with the delimiter).
    /// If `include` is true, the delimiter is included in the prefix.
    /// Otherwise, returns None.
    fn apply(&self, input: &'a str) -> (Option<&'a str>, &'a str) {
        // Log the input and delimiter at trace level.
        clerk::trace!(
            "Until rule: input='{}', delimiter='{}', mode={}",
            input,
            self.pattern,
            self.mode
        );
        match input.find(self.pattern) {
            Some(idx) => match self.mode {
                UntilMode::Discard => {
                    let end = idx + self.pattern.len();
                    clerk::debug!(
                        "Until rule matched (include): prefix='{}', rest='{}'",
                        &input[..idx],
                        &input[end..]
                    );
                    (Some(&input[..idx]), &input[end..])
                }
                UntilMode::KeepLeft => {
                    let end = idx + self.pattern.len();
                    clerk::debug!(
                        "Until rule matched (include): prefix='{}', rest='{}'",
                        &input[..end],
                        &input[end..]
                    );
                    (Some(&input[..end]), &input[end..])
                }
                UntilMode::KeepRight => {
                    clerk::debug!(
                        "Until rule matched: prefix='{}', rest='{}'",
                        &input[..idx],
                        &input[idx..]
                    );
                    (Some(&input[..idx]), &input[idx..])
                }
            },
            None => {
                clerk::debug!(
                    "Until rule did not match: delimiter '{}' not found in '{}'",
                    self.pattern,
                    input
                );
                (None, input)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use clerk::{LogLevel, init_log_with_level};

    use super::*;

    #[test]
    fn test_until_basic_not_include() {
        init_log_with_level(LogLevel::TRACE);
        let rule = UntilStr {
            pattern: ";",
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
        let rule = UntilStr {
            pattern: ";",
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
        let rule = UntilStr {
            pattern: ";",
            mode: super::UntilMode::KeepRight,
        };
        let input = "abc;def";
        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, Some("abc"));
        assert_eq!(rest, ";def");
    }

    #[test]
    fn test_until_no_delimiter() {
        init_log_with_level(LogLevel::TRACE);
        let rule = UntilStr {
            pattern: ";",
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
        let rule = UntilStr {
            pattern: ";",
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
        let rule = UntilStr {
            pattern: ";",
            mode: super::UntilMode::Discard,
        };
        let input = "";
        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, None);
        assert_eq!(rest, "");
    }
}
