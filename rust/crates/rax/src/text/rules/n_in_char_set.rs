use super::IFlowRule;
use crate::error::RuleError;
use crate::text::IRule;
use crate::text::filters::{CharSetFilter, ICharSetFilter, IFilter};

/// Matches exactly `N` characters from an ASCII-only char set.
///
/// Because the wrapped [`AsciiCharSetFilter`] guarantees ASCII-only
/// entries at compile time, every matched character is exactly one byte,
/// so this walks `input` by byte index instead of by UTF-8 char boundary
/// (no `char_indices` overhead), and membership testing is
/// `AsciiCharSetFilter::contains` — a single shift + mask, no per-call
/// `Option` unwrap.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NInAsciiCharSet<'f, F: ICharSetFilter<'f, N>, const N: usize, const M: usize>(pub &'f F);

impl<'f, F: ICharSetFilter<'f, N>, const N: usize, const M: usize> IRule
    for NInAsciiCharSet<'f, F, N, M>
{
}

impl<'f, F: ICharSetFilter<'f, N>, const N: usize, const M: usize> IFlowRule<true>
    for NInAsciiCharSet<'f, F, N, M>
{
    type Output<'a> = &'a str;

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

        for (i, &b) in bytes.iter().enumerate().take(N) {
            // `b` may be >= 0x80 (a lead/continuation byte of a multi-byte
            // UTF-8 char). `contains` checks `is_ascii()` first and
            // returns `false` for those rather than shifting out of
            // range, so this is also a correctness fix over shifting a
            // u128 by a raw byte value up to 255 directly.
            if !self.0.contains(b as char) {
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

        Ok(unsafe { (input.get_unchecked(..N), N) })
    }
}

/// Matches exactly `N` characters from a general (possibly non-ASCII)
/// char set.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NInCharSet<'f, const N: usize, const M: usize>(pub &'f CharSetFilter<M>);

impl<const N: usize, const M: usize> IRule for NInCharSet<'_, N, M> {}

impl<'f, const N: usize, const M: usize> IFlowRule<false> for NInCharSet<'f, N, M> {
    type Output<'a> = &'a str;

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
