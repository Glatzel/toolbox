use super::IStrFlowRule;
use crate::str_parser::IRule;
use crate::str_parser::filters::{CharSetFilter, IFilter};

/// Rule to match if the first N characters of the input are all in a given
/// character set. If so, returns a tuple of (matched_str, rest_of_input).
/// Otherwise, returns None.
pub struct NInCharSet<'a, const N: usize, const M: usize>(pub &'a CharSetFilter<M>);

impl<'a, const N: usize, const M: usize> IRule for NInCharSet<'a, N, M> {
    fn name(&self) -> &str { "NInCharSet" }
}

impl<'a, const N: usize, const M: usize> IStrFlowRule<'a> for NInCharSet<'a, N, M> {
    type Output = &'a str;
    /// Applies the NInCharSet rule to the input string.
    /// If the first N characters are all in the set, returns the matched
    /// substring and the rest. Otherwise, returns None and the original
    /// input.
    fn apply(&self, input: &'a str) -> (Option<&'a str>, &'a str) {
        let mut count = 0;
        for (i, c) in input.char_indices() {
            if self.0.filter(&c) {
                count += 1;
                let end_idx = i + c.len_utf8();
                if count == N {
                    let matched = &input[..end_idx];
                    let rest = &input[end_idx..];
                    clerk::debug!("NInCharSet matched: '{}', rest='{}'", matched, rest);
                    return (Some(matched), rest);
                }
            } else {
                // Found a char not in the set before reaching N
                clerk::debug!(
                    "NInCharSet did not match: char '{}' not in set at pos {}",
                    c,
                    i
                );
                return (None, input);
            }
        }
        // Not enough characters in input
        clerk::debug!("NInCharSet did not match: input too short or not enough chars in set");
        (None, input)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    extern crate std;
    use clerk::{LogLevel, init_log_with_level};

    use super::*;
    use crate::str_parser::filters::{ASCII_LETTERS_DIGITS, DIGITS};

    #[test]
    fn test_n_in_charset_match() {
        init_log_with_level(LogLevel::TRACE);
        let rule = NInCharSet::<3, 62>(&ASCII_LETTERS_DIGITS);
        let input = "abc123";
        let (matched, rest) = rule.apply(input);
        assert_eq!(matched, Some("abc"));
        assert_eq!(rest, "123");
    }

    #[test]
    fn test_n_in_charset_no_match() {
        init_log_with_level(LogLevel::TRACE);
        let rule = NInCharSet::<3, 10>(&DIGITS);
        let input = "12abc";
        let (matched, rest) = rule.apply(input);
        assert_eq!(matched, None);
        assert_eq!(rest, "12abc");
    }

    #[test]
    fn test_n_in_charset_too_short() {
        init_log_with_level(LogLevel::TRACE);
        let rule = NInCharSet::<4, 62>(&ASCII_LETTERS_DIGITS);
        let input = "ab";
        let (matched, rest) = rule.apply(input);
        assert_eq!(matched, None);
        assert_eq!(rest, "ab");
    }

    #[test]
    fn test_n_in_charset_empty_input() {
        init_log_with_level(LogLevel::TRACE);
        let rule = NInCharSet::<1, 62>(&ASCII_LETTERS_DIGITS);
        let input = "";
        let (matched, rest) = rule.apply(input);
        assert_eq!(matched, None);
        assert_eq!(rest, "");
    }

    #[test]
    fn test_n_in_charset_unicode() -> mischief::Result<()> {
        init_log_with_level(LogLevel::TRACE);
        let filter: CharSetFilter<2> = CharSetFilter::from_str("你好")?;
        let rule = NInCharSet::<2, 2>(&filter);
        let input = "你好世界";
        let (matched, rest) = rule.apply(input);
        assert_eq!(matched, Some("你好"));
        assert_eq!(rest, "世界");
        Ok(())
    }
}
