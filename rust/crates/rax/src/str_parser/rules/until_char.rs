use super::IStrFlowRule;
use crate::str_parser::IRule;
use crate::str_parser::rules::UntilMode;

/// Rule that extracts a substring from the start of the input until a
/// specified delimiter character is encountered.
///
/// `UntilChar<C>` searches the input string for the first occurrence of
/// character `C` and splits the input according to the selected [`UntilMode`].
///
/// # Fields
///
/// - `mode`: Determines how the delimiter is treated in the output:
///   - [`UntilMode::Discard`]: The delimiter is removed from the prefix and rest.
///   - [`UntilMode::KeepLeft`]: The delimiter is included at the end of the prefix.
///   - [`UntilMode::KeepRight`]: The delimiter is included at the start of the rest.
///
/// # Type Parameters
///
/// - `C`: The delimiter character to search for.
///
/// # Behavior
///
/// - Returns `(Some(prefix), rest)` if the delimiter is found.
/// - Returns `(None, input)` if the delimiter is not present in the input.
///
/// This rule respects UTF-8 character boundaries and logs trace/debug
/// information for each operation.
pub struct UntilChar<const C: char> {
    pub mode: super::UntilMode,
}

impl<const C: char> core::fmt::Debug for UntilChar<C> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "UntilChar<{}> {{ mode: {:?} }}", C, self.mode)
    }
}

impl<const C: char> core::fmt::Display for UntilChar<C> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<const C: char> IRule for UntilChar<C> {}

impl<'a, const C: char> IStrFlowRule<'a> for UntilChar<C> {
    type Output = &'a str;

    /// Applies the `UntilChar` rule to the input string.
    ///
    /// - Scans the input from the start until the delimiter `C` is found.
    /// - Returns a tuple `(prefix, rest)` split according to `self.mode`.
    /// - If the delimiter is not found, returns `(None, input)`.
    fn apply(&self, input: &'a str) -> (Option<&'a str>, &'a str) {
        clerk::trace!(
            "{self} rule: input='{}', char='{}', mode={}",
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
                            "{self} matched (discard): prefix='{}', rest='{}'",
                            &input[..i],
                            &input[end..]
                        );
                        return (Some(&input[..i]), &input[end..]);
                    }
                    UntilMode::KeepLeft => {
                        let end = i + C.len_utf8();
                        clerk::debug!(
                            "{self} matched (keep left): prefix='{}', rest='{}'",
                            &input[..end],
                            &input[end..]
                        );
                        return (Some(&input[..end]), &input[end..]);
                    }
                    UntilMode::KeepRight => {
                        clerk::debug!(
                            "{self} matched (keep right): prefix='{}', rest='{}'",
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
