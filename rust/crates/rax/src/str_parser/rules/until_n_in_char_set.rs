use super::IStrFlowRule;
use crate::str_parser::IRule;
use crate::str_parser::filters::{CharSetFilter, IFilter};
use crate::str_parser::rules::UntilMode;

/// Rule that extracts a prefix from the input string until the N-th character
/// matching a given character set is reached.
///
/// `UntilNInCharSet<N, M>` scans the input string from the start, counting
/// how many characters belong to the specified character set (defined by
/// `filter`).
///
/// # Fields
///
/// - `filter`: The [`CharSetFilter`] that defines the set of valid characters.
/// - `mode`: Determines how the N-th matched character is treated:
///   - [`UntilMode::Discard`]: The N-th character is excluded from the prefix
///     and removed from the rest.
///   - [`UntilMode::KeepLeft`]: The N-th character is included at the end of
///     the prefix.
///   - [`UntilMode::KeepRight`]: The N-th character is included at the start of
///     the rest.
///
/// # Type Parameters
///
/// - `N`: The number of matches required to stop scanning.
/// - `M`: The size of the character set (`CharSetFilter<M>`).
///
/// # Behavior
///
/// - Returns `(Some(prefix), rest)` when N characters in the set have been
///   seen, split according to `mode`.
/// - Returns `(None, input)` if fewer than N characters in the set are found.
/// - Respects UTF-8 character boundaries and logs trace/debug information.
pub struct UntilNInCharSet<'a, const N: usize, const M: usize> {
    pub filter: &'a CharSetFilter<M>,
    pub mode: UntilMode,
}

impl<'a, const N: usize, const M: usize> core::fmt::Debug for UntilNInCharSet<'a, N, M> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "UntilNInCharSet<N={}, M={}> {{ mode: {:?} }}",
            N, M, self.mode
        )
    }
}

impl<'a, const N: usize, const M: usize> core::fmt::Display for UntilNInCharSet<'a, N, M> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { write!(f, "{:?}", self) }
}

impl<'a, const N: usize, const M: usize> IRule for UntilNInCharSet<'a, N, M> {}

impl<'a, const N: usize, const M: usize> IStrFlowRule<'a> for UntilNInCharSet<'a, N, M> {
    type Output = &'a str;

    fn apply(&self, input: &'a str) -> (Option<&'a str>, &'a str) {
        let mut remaining = N;

        for (idx, ch) in input.char_indices() {
            if self.filter.filter(&ch) {
                remaining -= 1;
                if remaining == 0 {
                    let after = idx + ch.len_utf8();
                    let (prefix, rest) = match self.mode {
                        UntilMode::Discard => (&input[..idx], &input[after..]),
                        UntilMode::KeepLeft => (&input[..after], &input[after..]),
                        UntilMode::KeepRight => (&input[..idx], &input[idx..]),
                    };
                    clerk::debug!(
                        "UntilNInCharSet: mode={:?}, prefix='{}', rest='{}', idx={}, after={}, N={}",
                        self.mode,
                        prefix,
                        rest,
                        idx,
                        after,
                        N
                    );
                    return (Some(prefix), rest);
                }
            }
        }

        clerk::debug!(
            "{}: fewer than {} matches found, returning None, input='{}'",
            self,
            N,
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
    fn test_until_n_in_char_set_discard() {
        init_log_with_level(LogLevel::TRACE);
        let rule = UntilNInCharSet::<2, 10> {
            filter: &DIGITS,
            mode: UntilMode::Discard,
        };
        let input = "a1b2c3";
        let (prefix, rest) = rule.apply(input);
        // Should split before the second digit ('2'), so prefix is "a1b", rest is "2c3"
        assert_eq!(prefix, Some("a1b"));
        assert_eq!(rest, "c3");
    }

    #[test]
    fn test_until_n_in_char_set_keep_left() {
        init_log_with_level(LogLevel::TRACE);
        let rule = UntilNInCharSet::<2, 10> {
            filter: &DIGITS,
            mode: UntilMode::KeepLeft,
        };
        let input = "a1b2c3";
        let (prefix, rest) = rule.apply(input);
        // Should split after the second digit ('2'), so prefix is "a1b2", rest is "c3"
        assert_eq!(prefix, Some("a1b2"));
        assert_eq!(rest, "c3");
    }

    #[test]
    fn test_until_n_in_char_set_keep_right() {
        init_log_with_level(LogLevel::TRACE);
        let rule = UntilNInCharSet::<2, 10> {
            filter: &DIGITS,
            mode: UntilMode::KeepRight,
        };
        let input = "a1b2c3";
        let (prefix, rest) = rule.apply(input);
        // Should split before the second digit ('2'), so prefix is "a1b", rest is "2c3"
        assert_eq!(prefix, Some("a1b"));
        assert_eq!(rest, "2c3");
    }

    #[test]
    fn test_until_n_in_char_set_not_enough_matches() {
        init_log_with_level(LogLevel::TRACE);
        let rule = UntilNInCharSet::<4, 10> {
            filter: &DIGITS,
            mode: UntilMode::Discard,
        };
        let input = "a1b2c3";
        let (prefix, rest) = rule.apply(input);
        // Only 3 digits, so should return None and the original input
        assert_eq!(prefix, None);
        assert_eq!(rest, "a1b2c3");
    }

    #[test]
    fn test_until_n_in_char_set_empty_input() {
        init_log_with_level(LogLevel::TRACE);
        let rule = UntilNInCharSet::<1, 10> {
            filter: &DIGITS,
            mode: UntilMode::Discard,
        };
        let input = "";
        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, None);
        assert_eq!(rest, "");
    }

    #[test]
    fn test_until_n_in_char_set_unicode_keep_left() -> mischief::Result<()> {
        init_log_with_level(LogLevel::TRACE);
        let filter: CharSetFilter<3> = CharSetFilter::from_str("你世好")?;
        let rule = UntilNInCharSet::<2, 3> {
            filter: &filter,
            mode: UntilMode::KeepLeft,
        };
        let input = "你好世界";
        let (prefix, rest) = rule.apply(input);
        // Should split after the second match ('世'), so prefix is "你好世", rest is
        // "界"
        assert_eq!(prefix, Some("你好"));
        assert_eq!(rest, "世界");
        Ok(())
    }
}
