use super::IStrFlowRule;
use crate::str_parser::rules::IRule;

/// Rule to match a specific character at the start of the input string.
/// If the first character matches the expected character, returns a tuple of
/// (matched_char, rest_of_input). Otherwise, returns None.
pub struct Char<const C: char>;

impl<const C: char> IRule for Char<C> {
    fn name(&self) -> &str { "char" }
}

impl<'a, const C: char> IStrFlowRule<'a> for Char<C> {
    type Output = char;
    /// Applies the Char rule to the input string.
    /// If the first character matches `C`, returns the character and the
    /// rest of the string. Otherwise, returns None.
    fn apply(&self, input: &'a str) -> (Option<char>, &'a str) {
        // Log the input and the expected character at trace level.
        clerk::trace!("Char rule: input='{}', expected='{}'", input, C);
        let mut chars = input.char_indices();

        // Get the first character and its byte offset.
        if let Some((_, out)) = chars.next() {
            // first char's byte offset (0)
            if out == C {
                // If the character matches, find the next char boundary (or end of string).
                let (end, _) = chars.next().unwrap_or((input.len(), '\0')); // second char or end of string
                clerk::debug!("Char rule matched: '{}', rest='{}'", out, &input[end..]);
                (Some(out), &input[end..])
            } else {
                // If the character does not match, log and return None.
                clerk::debug!("Char rule did not match: found '{}', expected '{}'", out, C);
                (None, input)
            }
        } else {
            // No character in input
            (None, input)
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use clerk::{LogLevel, init_log_with_level};

    use super::*;

    #[test]
    fn test_char_match() {
        init_log_with_level(LogLevel::TRACE);
        let rule = Char::<'a'>;
        let input = "a123";
        let (matched, rest) = rule.apply(input);
        assert_eq!(matched, Some('a'));
        assert_eq!(rest, "123");
    }

    #[test]
    fn test_char_no_match() {
        init_log_with_level(LogLevel::TRACE);
        let rule = Char::<'d'>;
        let input = "abc";
        let (matched, rest) = rule.apply(input);
        assert_eq!(matched, None);
        assert_eq!(rest, "abc");
    }

    #[test]
    fn test_char_empty_input() {
        init_log_with_level(LogLevel::TRACE);
        let rule = Char::<'a'>;
        let input = "";
        let (matched, rest) = rule.apply(input);
        assert_eq!(matched, None);
        assert_eq!(rest, "");
    }

    #[test]
    fn test_char_unicode() {
        init_log_with_level(LogLevel::TRACE);
        let rule = Char::<'你'>;
        let input = "你好";
        let (matched, rest) = rule.apply(input);
        assert_eq!(matched, Some('你'));
        assert_eq!(rest, "好");
    }
}
