use super::IStrFlowRule;
use crate::str_parser::IRule;
use crate::str_parser::filters::{CharSetFilter, IFilter};
use crate::str_parser::rules::UntilMode;

/// Rule that extracts a prefix from the input string up to (but not including)
/// the position where N or more characters in the set have been seen.
/// Returns a tuple of (prefix, rest) where `prefix` contains all characters
/// up to the N-th character in the set, and `rest` is the remainder of the
/// string starting from that character. If fewer than N characters in the set
/// are found, returns (None, input).
/// The `mode` determines whether the N-th matched character is included in the
/// prefix, excluded, or kept as the first character of the rest.
pub struct UntilNInCharSet<'a, const N: usize, const M: usize> {
    pub filter: &'a CharSetFilter<M>,
    pub mode: UntilMode,
}

impl<'a, const N: usize, const M: usize> IRule for UntilNInCharSet<'a, N, M> {
    fn name(&self) -> &str { "UntilNInCharSet" }
}

impl<'a, const N: usize, const M: usize> IStrFlowRule<'a> for UntilNInCharSet<'a, N, M> {
    type Output = &'a str;

    /// Applies the rule to the input string, returning the prefix up to the
    /// N-th character in the set, and the rest of the string starting from
    /// that character. If fewer than N characters in the set are found,
    /// returns (None, input).
    /// The `mode` determines whether the N-th matched character is included in
    /// the prefix, excluded, or kept as the first character of the rest.
    fn apply(&self, input: &'a str) -> (Option<&'a str>, &'a str) {
        // How many more matches do we still need?
        let mut remaining = N;

        for (idx, ch) in input.char_indices() {
            // Ask the user‑supplied filter whether this character is in the set.
            if self.filter.filter(&ch) {
                remaining -= 1;
                if remaining == 0 {
                    // `idx` points to the first byte of the N‑th match.
                    // `after` points to the first byte *after* it.
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
        // Fewer than N occurrences found.
        clerk::debug!(
            "UntilNInCharSet: fewer than {} matches found, returning None, input='{}'",
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
