use super::IFlowRule;
use crate::error::RuleError;
use crate::text::IRule;
use crate::text::filters::{AsciiCharSetFilter, CharSetFilter, ICharSetFilter, IFilter};

/// Matches exactly `N` consecutive characters from a character set.
///
/// The `NInCharSet` rule checks whether the first `N` characters of the input
/// belong to the wrapped character set filter.
///
/// For `NInCharSet<..., true, AsciiCharSetFilter<...>>`, matching is performed
/// by byte because each ASCII character occupies exactly one byte. The
/// character set is represented by a bitmask, allowing membership checks
/// without iterating over UTF-8 character boundaries.
///
/// For `NInCharSet<..., false, CharSetFilter<...>>`, matching is performed
/// by Unicode scalar value using UTF-8 character boundaries.
///
/// If the first `N` characters belong to the character set, the rule returns
/// `Ok((prefix, consumed_bytes))`, where:
/// - `prefix` is the matched substring,
/// - `consumed_bytes` is the number of bytes consumed.
///
/// If the input contains fewer than `N` characters or any of the first `N`
/// characters does not belong to the character set, the rule returns a
/// [`RuleError`]. When `N` is zero, the rule succeeds with an empty prefix
/// and consumes zero bytes.
///
/// The ASCII implementation assumes that the input contains only ASCII
/// characters. This assumption is not validated by the rule.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NInCharSet<
    'f,
    const N: usize,
    const IS_ASCII: bool,
    F: ICharSetFilter<N_CHAR_SET>,
    const N_CHAR_SET: usize,
>(pub &'f F);

impl<const N: usize, const IS_ASCII: bool, F: ICharSetFilter<N_CHAR_SET>, const N_CHAR_SET: usize>
    IRule for NInCharSet<'_, N, IS_ASCII, F, N_CHAR_SET>
{
}

impl<const N: usize, const N_CHAR_SET: usize> IFlowRule<true>
    for NInCharSet<'_, N, true, AsciiCharSetFilter<N_CHAR_SET>, N_CHAR_SET>
{
    type Output<'a> = &'a str;

    /// Applies the `NInCharSet` rule to ASCII input.
    ///
    /// # Returns
    ///
    /// - `Ok((prefix, consumed_bytes))` if the first `N` bytes belong to the
    ///   character set.
    /// - `Err(RuleError)` if the input is shorter than `N` bytes or contains a
    ///   byte that does not belong to the character set.
    ///
    /// For `N == 0`, returns an empty prefix and consumes zero bytes.
    ///
    /// The input must contain only ASCII characters for the matched byte
    /// count to correspond to a character count.
    fn apply<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, usize), RuleError> {
        if N == 0 {
            return Ok(("", 0));
        }

        let bytes = input.as_bytes();

        if bytes.len() < N {
            return Err(RuleError {
                reason: "input too short or not enough chars in set".into(),
            });
        }

        // Check each byte using the character set bitmask.
        for &b in bytes.iter().take(N) {
            if self.0.mask() & (1_u128 << u32::from(b)) == 0 {
                return Err(RuleError {
                    reason: "char not in set".into(),
                });
            }
        }

        // SAFETY: N <= bytes.len(), and N is a UTF-8 boundary because the
        // input is assumed to contain only ASCII characters.
        Ok(unsafe { (input.get_unchecked(..N), N) })
    }
}

impl<const N: usize, const N_CHAR_SET: usize> IFlowRule<false>
    for NInCharSet<'_, N, false, CharSetFilter<N_CHAR_SET>, N_CHAR_SET>
{
    type Output<'a> = &'a str;

    /// Applies the `NInCharSet` rule to Unicode input.
    ///
    /// # Returns
    ///
    /// - `Ok((prefix, consumed_bytes))` if the first `N` Unicode scalar values
    ///   belong to the character set.
    /// - `Err(RuleError)` if the input contains fewer than `N` characters or
    ///   any of the first `N` characters does not belong to the character set.
    ///
    /// For `N == 0`, returns an empty prefix and consumes zero bytes.
    fn apply<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, usize), RuleError> {
        if N == 0 {
            return Ok(("", 0));
        }

        let mut count = 0;

        for (i, c) in input.char_indices() {
            if !self.0.filter(&c) {
                return Err(RuleError {
                    reason: "char not in set".into(),
                });
            }

            count += 1;

            if count == N {
                let advanced = i + c.len_utf8();

                // SAFETY: `advanced` is the end byte offset of a character
                // yielded by `char_indices()`, so it is a valid UTF-8 boundary.
                return Ok(unsafe { (input.get_unchecked(..advanced), advanced) });
            }
        }

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
        NInCharSet::<4, true, _, _>(&CHAR_SET_ASCII_LETTERS_DIGITS)
    );

    test_rule!(
        ascii_no_match,
        "12abc",
        NInCharSet::<3, true, _, _>(&CHAR_SET_DIGITS)
    );

    test_rule!(
        ascii_too_short,
        "ab",
        NInCharSet::<4, true, _, _>(&CHAR_SET_ASCII_LETTERS_DIGITS)
    );

    test_rule!(
        ascii_empty_input,
        "",
        NInCharSet::<1, true, _, _>(&CHAR_SET_ASCII_LETTERS_DIGITS)
    );

    test_rule!(
        utf8_match,
        "你好世界",
        NInCharSet::<2, false, _, _>(&CharSetFilter::new(['你', '好']))
    );

    test_rule!(
        utf8_too_short,
        "你",
        NInCharSet::<5, false, _, _>(&CharSetFilter::new(['你', '好']))
    );

    test_rule!(
        zero_n,
        "abc123",
        NInCharSet::<0, false, _, _>(&CharSetFilter::new(['你', '好']))
    );
}
