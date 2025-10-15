use super::IStrFlowRule;
use crate::str_parser::IRule;
use crate::str_parser::rules::UntilMode;

/// Rule that extracts a prefix from the input string up to the first occurrence
/// of a specified substring delimiter.
///
/// # Fields
///
/// - `pattern`: The delimiter substring to search for.
/// - `mode`: Determines how the delimiter is treated:
///   - [`UntilMode::Discard`]: Exclude the delimiter from the prefix and remove
///     it from the rest.
///   - [`UntilMode::KeepLeft`]: Include the delimiter in the prefix.
///   - [`UntilMode::KeepRight`]: Keep the delimiter at the start of the rest.
///
/// # Behavior
///
/// - Returns `(Some(prefix), rest)` if the delimiter is found, split according
///   to `mode`.
/// - Returns `(None, input)` if the delimiter is not found.
/// - Logs debug information for each split or when no match is found.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct UntilStr {
    pub pattern: &'static str,
    pub mode: UntilMode,
}

impl core::fmt::Debug for UntilStr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "UntilStr {{ pattern: {:?}, mode: {:?} }}",
            self.pattern, self.mode
        )
    }
}

impl core::fmt::Display for UntilStr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { write!(f, "{:?}", self) }
}

impl IRule for UntilStr {}

impl<'a> IStrFlowRule<'a> for UntilStr {
    type Output = &'a str;

    fn apply(&self, input: &'a str) -> (Option<&'a str>, &'a str) {
        clerk::trace!(
            "{}: input='{}', delimiter='{}', mode={}",
            self,
            input,
            self.pattern,
            self.mode
        );

        match input.find(self.pattern) {
            Some(idx) => {
                let end = idx + self.pattern.len();
                let (prefix, rest) = match self.mode {
                    UntilMode::Discard => (&input[..idx], &input[end..]),
                    UntilMode::KeepLeft => (&input[..end], &input[end..]),
                    UntilMode::KeepRight => (&input[..idx], &input[idx..]),
                };
                clerk::debug!("{} matched: prefix='{}', rest='{}'", self, prefix, rest);
                (Some(prefix), rest)
            }
            None => {
                clerk::debug!(
                    "{}: delimiter '{}' not found, returning None",
                    self,
                    self.pattern
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
