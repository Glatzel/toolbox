extern crate alloc;

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
    fn apply(&self, input: &'a str) -> Result<(char, usize), RuleError> {
        clerk::trace!("OneOfCharSet rule: input='{}'", input);

        let c = input.chars().next().ok_or_else(|| RuleError {
            reason: "empty input".into(),
        })?;

        if !self.0.filter(&c) {
            return Err(RuleError {
                reason: "character not in set".into(),
            });
        }

        Ok((c, c.len_utf8()))
    }
}

#[cfg(test)]
mod tests {
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
    #[case("unicode","你好世界", PhantomData::<OneOfCharSet<1>>,&CharSetFilter::new(['你']))]
    fn test_one_in_char_set<const N: usize>(
        #[case] name: &str,
        #[case] input: &str,
        #[case] _rule: PhantomData<OneOfCharSet<N>>,
        #[case] charset: &CharSetFilter<N>,
    ) {
        init_log_with_level(LevelFilter::TRACE);
        let result = OneOfCharSet::<N>(charset)
            .apply(input)
            .map(|(out, rest)| (out, &input[rest..]));
        insta::assert_debug_snapshot!(format!("{}", name), result);
    }
}
