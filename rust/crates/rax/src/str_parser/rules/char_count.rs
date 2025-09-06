use super::IStrFlowRule;
use crate::str_parser::rules::IRule;

/// Rule to extract a fixed number of characters from the input string.
/// Returns a tuple of (prefix, rest) if enough characters are present,
/// otherwise returns None.
pub struct CharCount<const N: usize>;

impl<const N: usize> IRule for CharCount<N> {
    fn name(&self) -> &str { "CharCount" }
}

impl<'a, const N: usize> IStrFlowRule<'a> for CharCount<N> {
    type Output = &'a str;
    /// Applies the CharCount rule to the input string.
    /// If the input contains at least `self.0` characters, returns
    /// the first `self.0` characters and the rest of the string.
    /// Otherwise, returns None.
    fn apply(&self, input: &'a str) -> (Option<&'a str>, &'a str) {
        // Log the input and the requested character count at trace level.
        clerk::trace!("CharCount rule: input='{}', count={}", input, N);

        // If count is zero, return empty prefix and full input.
        if N == 0 {
            clerk::debug!("CharCount: count is zero, returning empty prefix and full input.");
            return (Some(""), input);
        }

        // Count the number of characters in the input.
        let length = input.chars().count();

        // If count matches input length, return the whole input as prefix.
        if N == length {
            clerk::debug!("CharCount: count matches input length, returning whole input.");
            return (Some(input), "");
        }

        // Iterate over char boundaries to find the split point.
        for (count, (idx, _)) in input.char_indices().enumerate() {
            if count == N {
                // Found the split point at the requested character count.
                clerk::debug!(
                    "CharCount: found split at char {}, byte idx {}: prefix='{}', rest='{}'",
                    count,
                    idx,
                    &input[..idx],
                    &input[idx..]
                );
                return (Some(&input[..idx]), &input[idx..]);
            }
        }

        // Not enough characters in the input.
        clerk::warn!(
            "CharCount: not enough chars in input (needed {}, found {})",
            N,
            length
        );
        (None, input)
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use clerk::{LogLevel, init_log_with_level};

    use super::*;

    #[test]
    fn test_count_exact_length() {
        init_log_with_level(LogLevel::TRACE);
        let rule = CharCount::<4>;
        let input = "test";
        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, Some("test"));
        assert_eq!(rest, "");
    }

    #[test]
    fn test_count_less_than_length() {
        init_log_with_level(LogLevel::TRACE);
        let rule = CharCount::<2>;
        let input = "hello";
        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, Some("he"));
        assert_eq!(rest, "llo");
    }

    #[test]
    fn test_count_more_than_length() {
        init_log_with_level(LogLevel::TRACE);
        let rule = CharCount::<10>;
        let input = "short";
        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, None);
        assert_eq!(rest, "short");
    }

    #[test]
    fn test_count_zero() {
        init_log_with_level(LogLevel::TRACE);
        let rule = CharCount::<0>;
        let input = "abc";
        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, Some(""));
        assert_eq!(rest, "abc");
    }

    #[test]
    fn test_count_empty_input() {
        init_log_with_level(LogLevel::TRACE);
        let rule = CharCount::<0>;
        let input = "";
        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, Some(""));
        assert_eq!(rest, "");
    }

    #[test]
    fn test_count_non_ascii() {
        init_log_with_level(LogLevel::TRACE);
        let rule = CharCount::<2>;
        let input = "你好世界";
        // Should return first 2 chars ("你", "好") and the rest ("世界")
        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, Some("你好"));
        assert_eq!(rest, "世界");
    }
}
