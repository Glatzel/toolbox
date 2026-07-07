extern crate alloc;

use alloc::string::ToString;
use core::fmt::Debug;

use super::IStrFlowRule;
use crate::error::RuleError;
use crate::string::IRule;
use crate::string::filters::{CharSetFilter, IFilter};

/// Rule that matches if the first `N` characters of the input are all in a
/// specified character set.
///
/// `NInCharSet<'a, N, M>` takes a reference to a [`CharSetFilter<M>`] and
/// checks the first `N` characters of the input string. If all `N` characters
/// are present in the character set, it returns a tuple `(Some(matched), rest)`
/// where `matched` is the substring of the first `N` characters and `rest` is
/// the remainder of the input. Otherwise, it returns `(None, input)`.
///
/// This rule respects UTF-8 boundaries and stops immediately on the first
/// character that does not belong to the set, or if the input is too short.
///
/// # Type Parameters
///
/// - `'a`: Lifetime of the character set reference.
/// - `N`: Number of characters to match at the start of the input.
/// - `M`: Size of the character set (length of the `CharSetFilter`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NInCharSet<'a, const N: usize, const M: usize>(pub &'a CharSetFilter<M>);

impl<'a, const N: usize, const M: usize> IRule for NInCharSet<'a, N, M> {}

impl<'a, const N: usize, const M: usize> IStrFlowRule<'a> for NInCharSet<'a, N, M> {
    type Output = &'a str;

    /// Applies the `NInCharSet` rule to the input string.
    ///
    /// # Returns
    ///
    /// - `(Some(matched), rest)` if the first `N` characters are all in the
    ///   character set.
    /// - `(None, input)` if a character is not in the set before reaching `N`,
    ///   or if the input has fewer than `N` characters.
    ///
    /// # Logging
    ///
    /// - Debug-level logs indicate matches, unmatched characters, and
    ///   insufficient input.
    fn apply(&self, input: &'a str) -> Result<(Self::Output, &'a str), RuleError> {
        let mut count = 0;
        for (i, c) in input.char_indices() {
            if self.0.filter(&c) {
                count += 1;
                let end_idx = i + c.len_utf8();
                if count == N {
                    let matched = &input[..end_idx];
                    let rest = &input[end_idx..];
                    clerk::debug!("{:?} matched: '{}', rest='{:?}'", self, matched, rest);
                    return Ok((matched, rest));
                }
            } else {
                clerk::debug!(
                    "{:?} did not match: char '{}' not in set at pos {}",
                    self,
                    c,
                    i
                );
                return Err(RuleError {
                    reason: "char not in set".to_string(),
                });
            }
        }
        clerk::debug!(
            "{:?} did not match: input too short or not enough chars in set",
            self
        );
        Err(RuleError {
            reason: "input too short or not enough chars in set".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use core::marker::PhantomData;
    use std::str::FromStr;
    extern crate std;
    use std::format;

    use clerk::{LevelFilter, init_log_with_level};

    use super::*;
    use crate::string::filters::{ASCII_LETTERS_DIGITS, DIGITS};
    #[rstest::rstest]
    #[case("match","abc123", PhantomData::<NInCharSet<4,_>>,&ASCII_LETTERS_DIGITS)]
    #[case("no_match","12abc", PhantomData::<NInCharSet<3,_>>,&DIGITS)]
    #[case("too_short","ab", PhantomData::<NInCharSet<4,_>>,&ASCII_LETTERS_DIGITS)]
    #[case("empty_input","", PhantomData::<NInCharSet<1,_>>,&ASCII_LETTERS_DIGITS)]
    #[case("unicode","你好世界", PhantomData::<NInCharSet<2,2>>,&CharSetFilter::from_str("你好").unwrap())]
    fn test_n_in_charset<const N: usize, const M: usize>(
        #[case] name: &str,
        #[case] input: &str,
        #[case] _rule: PhantomData<NInCharSet<N, M>>,
        #[case] charset: &CharSetFilter<M>,
    ) {
        init_log_with_level(LevelFilter::TRACE);
        let result = NInCharSet::<N, M>(charset).apply(input);
        insta::assert_debug_snapshot!(format!("{}", name), result);
    }
}
