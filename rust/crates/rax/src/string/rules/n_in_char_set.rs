extern crate alloc;

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
    fn apply(&self, input: &'a str, is_ascii: bool) -> Result<(Self::Output, usize), RuleError> {
        if N == 0 {
            clerk::warn!("N is 0, returning empty string");
            return Ok(("", 0));
        }

        if is_ascii {
            let bytes = input.as_bytes();

            if bytes.len() < N {
                return Err(RuleError {
                    reason: "input too short or not enough chars in set".into(),
                });
            }

            if let Some(mask) = self.0.ascii_mask() {
                // Fast path: bitmask, no per-byte filter() dispatch
                for (i, &b) in bytes.iter().enumerate().take(N) {
                    if mask & (1u128 << b as u32) == 0 {
                        clerk::debug!(
                            "{:?} did not match: char '{}' not in set at byte pos {}",
                            self,
                            b as char,
                            i
                        );
                        return Err(RuleError {
                            reason: "char not in set".into(),
                        });
                    }
                }
                return Ok(unsafe { (input.get_unchecked(..N), N) });
            }

            // Fallback: table has non-ASCII entries
            for (i, &b) in bytes.iter().enumerate().take(N) {
                let c = b as char;
                if !self.0.filter(&c) {
                    clerk::debug!(
                        "{:?} did not match: char '{}' not in set at byte pos {}",
                        self,
                        c,
                        i
                    );
                    return Err(RuleError {
                        reason: "char not in set".into(),
                    });
                }
            }
            return Ok(unsafe { (input.get_unchecked(..N), N) });
        }
        let mut count = 0;
        for (i, c) in input.char_indices() {
            if !self.0.filter(&c) {
                clerk::debug!(
                    "{:?} did not match: char '{}' not in set at byte pos {}",
                    self,
                    c,
                    i
                );

                return Err(RuleError {
                    reason: "char not in set".into(),
                });
            }

            count += 1;

            if count == N {
                let advanced = i + c.len_utf8();
                return Ok(unsafe { (input.get_unchecked(..advanced), advanced) });
            }
        }
        clerk::debug!(
            "{:?} did not match: input too short or not enough chars in set",
            self
        );
        Err(RuleError {
            reason: "input too short or not enough chars in set".into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use core::marker::PhantomData;

    extern crate std;
    use std::format;

    use clerk::{LevelFilter, init_log_with_level};

    use super::*;
    use crate::string::filters::{CHAR_SET_ASCII_LETTERS_DIGITS, CHAR_SET_DIGITS};
    #[rstest::rstest]
    #[case("ascii_match","abc123", PhantomData::<NInCharSet<4,_>>,&CHAR_SET_ASCII_LETTERS_DIGITS)]
    #[case("ascii_no_match","12abc", PhantomData::<NInCharSet<3,_>>,&CHAR_SET_DIGITS)]
    #[case("ascii_too_short","ab", PhantomData::<NInCharSet<4,_>>,&CHAR_SET_ASCII_LETTERS_DIGITS)]
    #[case("ascii_empty_input","", PhantomData::<NInCharSet<1,_>>,&CHAR_SET_ASCII_LETTERS_DIGITS)]
    #[case("utf8_match","你好世界", PhantomData::<NInCharSet<2,_>>,&CharSetFilter::new(['你', '好']))]
    #[case("utf8_no_match","你好世界", PhantomData::<NInCharSet<3,_>>,&CHAR_SET_DIGITS)]
    #[case("utf8_too_short","你", PhantomData::<NInCharSet<5,_>>,&CharSetFilter::new(['你', '好']))]
    #[case("zero_n","abc123", PhantomData::<NInCharSet<0,_>>,&CharSetFilter::new(['你', '好']))]
    fn test_n_in_charset<const N: usize, const M: usize>(
        #[case] name: &str,
        #[case] input: &str,
        #[case] _rule: PhantomData<NInCharSet<N, M>>,
        #[case] charset: &CharSetFilter<M>,
    ) {
        init_log_with_level(LevelFilter::TRACE);
        let result = NInCharSet::<N, M>(charset)
            .apply(input, input.is_ascii())
            .map(|(out, idx)| (out, input.get(idx..).unwrap()));
        insta::assert_debug_snapshot!(format!("{}", name), result);
    }
}
