extern crate alloc;

use alloc::string::ToString;
use core::fmt::Debug;

use super::IStrFlowRule;
use crate::error::RuleError;
use crate::string::IRule;
use crate::string::filters::{CharSetFilter, IFilter};

/// Rule that matches the first character of the input string if it belongs to
/// a specified character set.
///
/// `OneOfCharSet<'a, N>` takes a reference to a [`CharSetFilter<N>`] and checks
/// the first character of the input. If the first character is in the set, it
/// returns a tuple `(Some(matched), rest)` where `matched` is the character and
/// `rest` is the remainder of the input. Otherwise, it returns `(None, input)`.
///
/// This rule respects UTF-8 boundaries and stops immediately on the first
/// character if it is not in the set, or if the input is empty.
///
/// # Type Parameters
///
/// - `'a`: Lifetime of the character set reference.
/// - `N`: Size of the character set (length of the `CharSetFilter`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OneOfCharSet<'a, const N: usize>(pub &'a CharSetFilter<N>);

impl<'a, const N: usize> IRule for OneOfCharSet<'a, N> {}

impl<'a, const N: usize> IStrFlowRule<'a> for OneOfCharSet<'a, N> {
    type Output = char;

    /// Applies the `OneOfCharSet` rule to the input string.
    ///
    /// # Returns
    ///
    /// - `(Some(matched), rest)` if the first character is in the character
    ///   set.
    /// - `(None, input)` if the first character is not in the set or the input
    ///   is empty.
    ///
    /// # Logging
    ///
    /// - Trace-level logs show the input string.
    /// - Debug-level logs indicate matches or mismatches.
    fn apply(&self, input: &'a str) -> Result<(char, &'a str), RuleError> {
        clerk::trace!("OneOfCharSet rule: input='{}'", input);

        if let Some((_, c)) = input.char_indices().next() {
            if self.0.filter(&c) {
                let next_i = input.char_indices().nth(1).map_or(input.len(), |(j, _)| j);
                clerk::debug!("OneOfCharSet matched: '{}', rest='{}'", c, &input[next_i..]);
                Ok((c, &input[next_i..]))
            } else {
                clerk::debug!("OneOfCharSet did not match: found '{}', not in set", c);
                Err(RuleError {
                    reason: "character not in set".to_string(),
                })
            }
        } else {
            Err(RuleError {
                reason: "empty input".to_string(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    extern crate std;
    use core::marker::PhantomData;
    use std::format;

    use clerk::{LevelFilter, init_log_with_level};

    use super::*;
    use crate::string::filters::{ASCII_LETTERS_DIGITS, DIGITS};
    #[rstest::rstest]
    #[case("match","a123", PhantomData::<OneOfCharSet<_>>,&ASCII_LETTERS_DIGITS)]
    #[case("no_match","abc", PhantomData::<OneOfCharSet<_>>,&DIGITS)]
    #[case("empty_input","", PhantomData::<OneOfCharSet<_>>,&ASCII_LETTERS_DIGITS)]
    #[case("unicode","你好世界", PhantomData::<OneOfCharSet<1>>,&CharSetFilter::from_str("你").unwrap())]
    fn test_one_in_char_set<const N: usize>(
        #[case] name: &str,
        #[case] input: &str,
        #[case] _rule: PhantomData<OneOfCharSet<N>>,
        #[case] charset: &CharSetFilter<N>,
    ) {
        init_log_with_level(LevelFilter::TRACE);
        let result = OneOfCharSet::<N>(charset).apply(input);
        insta::assert_debug_snapshot!(format!("{}", name), result);
    }
}
