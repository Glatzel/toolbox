use super::IStrFlowRule;
use crate::str_parser::IRule;
use crate::str_parser::filters::{CharSetFilter, IFilter};

/// Rule to match if the first character of the input is in a given character
/// set. If the first character is in the set, returns a tuple of (matched_char,
/// rest_of_input). Otherwise, returns None.
pub struct OneOfCharSet<'a, const N: usize>(pub &'a CharSetFilter<N>);

impl<'a, const N: usize> IRule for OneOfCharSet<'a, N> {
    fn name(&self) -> &str { "OneOfCharSet" }
}

impl<'a, const N: usize> IStrFlowRule<'a> for OneOfCharSet<'a, N> {
    type Output = char;
    /// Applies the OneOfCharSet rule to the input string.
    /// If the first character is in the set, returns the character and the rest
    /// of the string. Otherwise, returns None.
    fn apply(&self, input: &'a str) -> (Option<char>, &'a str) {
        // Log the input at trace level.
        clerk::trace!("OneOfCharSet rule: input='{}'", input);
        // Get the first character and its byte offset.
        if let Some((_, c)) = input.char_indices().next() {
            if self.0.filter(&c) {
                // If the character is in the set, find the next char boundary (or end of
                // string).
                let next_i = input.char_indices().nth(1).map_or(input.len(), |(j, _)| j);
                clerk::debug!("OneOfCharSet matched: '{}', rest='{}'", c, &input[next_i..]);
                (Some(c), &input[next_i..])
            } else {
                // If the character is not in the set, log and return None.
                clerk::debug!("OneOfCharSet did not match: found '{}', not in set", c);
                (None, input)
            }
        } else {
            // No character in input, return None and the original input
            (None, input)
        }
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
    fn test_char_match() {
        init_log_with_level(LogLevel::TRACE);
        let rule = OneOfCharSet(&ASCII_LETTERS_DIGITS);
        let input = "a123";
        let (matched, rest) = rule.apply(input);
        assert_eq!(matched, Some('a'));
        assert_eq!(rest, "123");
    }

    #[test]
    fn test_char_no_match() {
        init_log_with_level(LogLevel::TRACE);
        let rule = OneOfCharSet(&DIGITS);
        let input = "abc";
        let (matched, rest) = rule.apply(input);
        assert_eq!(matched, None);
        assert_eq!(rest, "abc");
    }

    #[test]
    fn test_char_empty_input() {
        init_log_with_level(LogLevel::TRACE);
        let rule = OneOfCharSet(&ASCII_LETTERS_DIGITS);
        let input = "";
        let (matched, rest) = rule.apply(input);
        assert_eq!(matched, None);
        assert_eq!(rest, "");
    }

    #[test]
    fn test_char_unicode() -> mischief::Result<()> {
        init_log_with_level(LogLevel::TRACE);
        let filter: CharSetFilter<1> = CharSetFilter::from_str("你")?;
        let rule = OneOfCharSet(&filter);
        let input = "你好";
        let (matched, rest) = rule.apply(input);
        assert_eq!(matched, Some('你'));
        assert_eq!(rest, "好");
        Ok(())
    }
}
