use core::fmt::{self, Debug, Display};

use super::IStrFlowRule;
use crate::str_parser::rules::IRule;

/// Rule that matches a specific character at the start of the input string.
///
/// `Char<C>` checks if the first character of the input string is equal to the
/// expected character `C`. If the first character matches, it returns a tuple:
/// `(Some(C), rest)` where `rest` is the remainder of the input after the
/// matched character. Otherwise, it returns `(None, input)`.
///
/// This rule respects UTF-8 character boundaries and only examines the first
/// character of the input.
pub struct Char<const C: char>;

impl<const C: char> Debug for Char<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "Char<{:?}>", C) }
}

impl<const C: char> Display for Char<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{:?}", self) }
}

impl<const C: char> IRule for Char<C> {}

impl<'a, const C: char> IStrFlowRule<'a> for Char<C> {
    type Output = char;

    /// Applies the `Char` rule to the input string.
    ///
    /// # Returns
    ///
    /// - `(Some(C), rest)` if the first character of the input matches `C`.
    /// - `(None, input)` if the first character does not match `C` or the input
    ///   is empty.
    ///
    /// # Logging
    ///
    /// - Trace-level logs show the input and the expected character.
    /// - Debug-level logs show whether a match occurred and the resulting rest
    ///   of the input.
    fn apply(&self, input: &'a str) -> (Option<char>, &'a str) {
        clerk::trace!("{self}: input='{}', expected='{}'", input, C);

        let mut chars = input.char_indices();

        if let Some((_, first_char)) = chars.next() {
            if first_char == C {
                // Find the next char boundary or end of string
                let (end, _) = chars.next().unwrap_or((input.len(), '\0'));
                clerk::debug!("{self} matched: '{}', rest='{}'", first_char, &input[end..]);
                (Some(first_char), &input[end..])
            } else {
                clerk::debug!(
                    "{self} did not match: found '{}', expected '{}'",
                    first_char,
                    C
                );
                (None, input)
            }
        } else {
            // Input is empty
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
