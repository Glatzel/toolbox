use super::IFlowRule;
use crate::error::RuleError;
use crate::text::IRule;
use crate::text::filters::{CharSetFilter, IFilter};

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
pub struct NInCharSet<'f, const N: usize, const M: usize, const IS_ASCII: bool>(
    pub &'f CharSetFilter<M>,
);

impl<const N: usize, const M: usize, const IS_ASCII: bool> IRule
    for NInCharSet<'_, N, M, IS_ASCII>
{
}

impl<'f, const N: usize, const M: usize> IFlowRule<true> for NInCharSet<'f, N, M, true> {
    type Output<'a> = &'a str;

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
    fn apply<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, usize), RuleError> {
        if N == 0 {
            clerk::warn!("N is 0, returning empty string");
            return Ok(("", 0));
        }

        let bytes = input.as_bytes();

        if bytes.len() < N {
            return Err(RuleError {
                reason: "input too short or not enough chars in set".into(),
            });
        }

        if let Some(mask) = self.0.ascii_mask() {
            // Fast path: bitmask, no per-byte filter() dispatch
            for (i, &b) in bytes.iter().enumerate().take(N) {
                if mask & (1_u128 << u32::from(b)) == 0 {
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
}
impl<'f, const N: usize, const M: usize> IFlowRule<false> for NInCharSet<'f, N, M, false> {
    type Output<'a> = &'a str;

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
    fn apply<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, usize), RuleError> {
        if N == 0 {
            clerk::warn!("N is 0, returning empty string");
            return Ok(("", 0));
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

    use super::*;
    use crate::test_rule;
    use crate::text::filters::{CHAR_SET_ASCII_LETTERS_DIGITS, CHAR_SET_DIGITS};
    test_rule!(
        ascii_match,
        "abc123",
        NInCharSet::<4, _, true>(&CHAR_SET_ASCII_LETTERS_DIGITS)
    );

    test_rule!(
        ascii_no_match,
        "12abc",
        NInCharSet::<3, _, true>(&CHAR_SET_DIGITS)
    );

    test_rule!(
        ascii_too_short,
        "ab",
        NInCharSet::<4, _, true>(&CHAR_SET_ASCII_LETTERS_DIGITS)
    );

    test_rule!(
        ascii_empty_input,
        "",
        NInCharSet::<1, _, true>(&CHAR_SET_ASCII_LETTERS_DIGITS)
    );

    test_rule!(
        ascii_fallback_match,
        "abc123",
        NInCharSet::<4, _, true>(&CharSetFilter::new(['a', 'b', 'c', '1', '你']))
    );

    test_rule!(
        ascii_fallback_no_match,
        "abx123",
        NInCharSet::<4, _, true>(&CharSetFilter::new(['a', 'b', 'c', '1', '你']))
    );

    test_rule!(
        ascii_fallback_too_short,
        "abc",
        NInCharSet::<4, _, true>(&CharSetFilter::new(['a', 'b', 'c', '1', '你']))
    );

    test_rule!(
        utf8_match,
        "你好世界",
        NInCharSet::<2, _, false>(&CharSetFilter::new(['你', '好']))
    );

    test_rule!(
        utf8_no_match,
        "你好世界",
        NInCharSet::<3, _, false>(&CHAR_SET_DIGITS)
    );

    test_rule!(
        utf8_too_short,
        "你",
        NInCharSet::<5, _, false>(&CharSetFilter::new(['你', '好']))
    );

    test_rule!(
        zero_n,
        "abc123",
        NInCharSet::<0, _, false>(&CharSetFilter::new(['你', '好']))
    );
}
