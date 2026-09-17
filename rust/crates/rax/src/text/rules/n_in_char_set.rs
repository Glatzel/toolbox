use super::IFlowRule;
use crate::error::RuleError;
use crate::text::IRule;
use crate::text::filters::{AsciiCharSetFilter, CharSetFilter, ICharSetFilter, IFilter};

/// Matches exactly `N` characters from an ASCII-only char set.
///
/// Because the wrapped [`AsciiCharSetFilter`] guarantees ASCII-only
/// entries at compile time, every matched character is exactly one byte,
/// so this walks `input` by byte index instead of by UTF-8 char boundary
/// (no `char_indices` overhead), and membership testing is
/// `AsciiCharSetFilter::contains` — a single shift + mask, no per-call
/// `Option` unwrap.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NInCharSet<
    'f,
    const N: usize,
    F: ICharSetFilter<N_CHAR_SET>,
    const N_CHAR_SET: usize,
    const IS_ASCII: bool,
>(pub &'f F);

impl<
    'f,
    F: ICharSetFilter<N_CHAR_SET>,
    const N: usize,
    const N_CHAR_SET: usize,
    const IS_ASCII: bool,
> IRule for NInCharSet<'f, N, F, N_CHAR_SET, IS_ASCII>
{
}

impl<'f, const N: usize, const N_CHAR_SET: usize> IFlowRule<true>
    for NInCharSet<'f, N, AsciiCharSetFilter<N_CHAR_SET>, N_CHAR_SET, true>
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

        // Fast path: bitmask, no per-byte filter() dispatch
        for (i, &b) in bytes.iter().enumerate().take(N) {
            if self.0.mask() & (1_u128 << u32::from(b)) == 0 {
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
}
impl<'f, const N: usize, const N_CHAR_SET: usize> IFlowRule<true>
    for NInCharSet<'f, N, CharSetFilter<N_CHAR_SET>, N_CHAR_SET, true>
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

impl<'f, const N: usize, const N_CHAR_SET: usize> IFlowRule<false>
    for NInCharSet<'f, N, CharSetFilter<N_CHAR_SET>, N_CHAR_SET, false>
{
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
        NInCharSet::<4, _, _, true>(&CHAR_SET_ASCII_LETTERS_DIGITS)
    );

    test_rule!(
        ascii_no_match,
        "12abc",
        NInCharSet::<3, _, _, true>(&CHAR_SET_DIGITS)
    );

    test_rule!(
        ascii_too_short,
        "ab",
        NInCharSet::<4, _, _, true>(&CHAR_SET_ASCII_LETTERS_DIGITS)
    );

    test_rule!(
        ascii_empty_input,
        "",
        NInCharSet::<1, _, _, true>(&CHAR_SET_ASCII_LETTERS_DIGITS)
    );

    test_rule!(
        ascii_fallback_match,
        "abc123",
        NInCharSet::<4, _, _, true>(&CharSetFilter::new(['a', 'b', 'c', '1', '你']))
    );

    test_rule!(
        ascii_fallback_no_match,
        "abx123",
        NInCharSet::<4, _, _, true>(&CharSetFilter::new(['a', 'b', 'c', '1', '你']))
    );

    test_rule!(
        ascii_fallback_too_short,
        "abc",
        NInCharSet::<4, _, _, true>(&CharSetFilter::new(['a', 'b', 'c', '1', '你']))
    );

    test_rule!(
        utf8_match,
        "你好世界",
        NInCharSet::<2, _, _, false>(&CharSetFilter::new(['你', '好']))
    );

    test_rule!(
        utf8_no_match,
        "你好世界",
        NInCharSet::<3, _, _, false>(&CHAR_SET_DIGITS)
    );

    test_rule!(
        utf8_too_short,
        "你",
        NInCharSet::<5, _, _, false>(&CharSetFilter::new(['你', '好']))
    );

    test_rule!(
        zero_n,
        "abc123",
        NInCharSet::<0, _, _, false>(&CharSetFilter::new(['你', '好']))
    );
}
