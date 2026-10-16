use core::fmt::Debug;

use crate::text::filters::IFilter;

/// A character-set filter over `&char`.
///
/// Implemented by [`CharSetFilter`] and [`AsciiCharSetFilter`], and consumed by
/// the character-set rules to test membership.
pub trait ICharSetFilter<const N: usize>: for<'a> IFilter<&'a char> + Debug {}

/// A fixed set of characters for membership testing.
///
/// [`CharSetFilter::new`] sorts the supplied characters, and `filter` tests
/// membership with a binary search over the sorted table. Duplicate characters
/// are permitted and do not affect the result.
///
/// If every character is known to be ASCII *at compile time*, prefer
/// [`AsciiCharSetFilter`] instead: it stores the set as a 128-bit bitmap and
/// enforces the ASCII invariant as a compile error.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CharSetFilter<const N: usize> {
    table: [char; N],
}

impl<const N: usize> CharSetFilter<N> {
    /// Builds a filter from `table`, sorting it in place.
    ///
    /// # Returns
    ///
    /// A filter whose membership tests cover exactly the characters in `table`.
    pub const fn new(mut table: [char; N]) -> Self {
        let mut i = 1;
        while i < N {
            let key = table[i];
            let key_val = key as u32;
            let mut j = i;
            while j > 0 && (table[j - 1] as u32) > key_val {
                table[j] = table[j - 1];
                j -= 1;
            }
            table[j] = key;
            i += 1;
        }
        Self { table }
    }
}
impl<const N: usize> ICharSetFilter<N> for CharSetFilter<N> {}
impl<const N: usize> IFilter<&char> for CharSetFilter<N> {
    fn filter(&self, input: &char) -> bool { self.table.binary_search(input).is_ok() }
}

/// A fixed set of **ASCII** characters, verified entirely at compile time.
///
/// [`new`](AsciiCharSetFilter::new) is a `const fn` that panics during
/// const-evaluation if any character is not ASCII. When used to initialize a
/// `const` or `static` item (as every predefined filter below does), a
/// non-ASCII entry becomes a **compile error** rather than a runtime check.
///
/// Because ASCII-ness is guaranteed, the set is stored as a 128-bit bitmap and
/// [`contains`](AsciiCharSetFilter::contains) is itself `const`, so a lookup can
/// be evaluated at compile time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AsciiCharSetFilter<const N: usize> {
    table: [char; N],
    mask: u128,
}

impl<const N: usize> AsciiCharSetFilter<N> {
    /// Builds the filter, sorting `table` and computing its bitmask.
    ///
    /// # Panics
    ///
    /// Panics if any character in `table` is not ASCII. When `table` is
    /// known at compile time (e.g. initializing a `const`), this becomes
    /// a compile error instead of a runtime panic.
    pub const fn new(mut table: [char; N]) -> Self {
        // Sort (same insertion sort as `CharSetFilter::new`).
        let mut i = 1;
        while i < N {
            let key = table[i];
            let key_val = key as u32;
            let mut j = i;
            while j > 0 && (table[j - 1] as u32) > key_val {
                table[j] = table[j - 1];
                j -= 1;
            }
            table[j] = key;
            i += 1;
        }

        let mut mask: u128 = 0;
        let mut i = 0;
        while i < N {
            let c = table[i];
            assert!(
                c.is_ascii(),
                "AsciiCharSetFilter: all characters must be ASCII"
            );
            mask |= 1_u128 << (c as u32);
            i += 1;
        }

        Self { table, mask }
    }

    /// Tests whether `c` belongs to the set.
    ///
    /// A non-ASCII `c` returns `false`.
    ///
    /// # Notes
    ///
    /// `contains` is a `const fn`, so a lookup can be evaluated at compile time
    /// (for example `const FOO: bool = MY_FILTER.contains('a');`).
    pub const fn contains(&self, c: char) -> bool {
        c.is_ascii() && (self.mask >> (c as u32)) & 1 != 0
    }

    /// Returns the 128-bit bitmap backing [`contains`](AsciiCharSetFilter::contains).
    pub const fn mask(&self) -> u128 { self.mask }
}
impl<const N: usize> ICharSetFilter<N> for AsciiCharSetFilter<N> {}
impl<const N: usize> IFilter<&char> for AsciiCharSetFilter<N> {
    fn filter(&self, input: &char) -> bool { self.contains(*input) }
}

// Predefined filters
// All are ASCII by construction, so they use `AsciiCharSetFilter` and get
// O(1), fully-const-evaluated membership testing for free.

/// Digits 0–9.
pub const CHAR_SET_DIGITS: AsciiCharSetFilter<10> =
    AsciiCharSetFilter::new(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']);

/// ASCII letters, uppercase and lowercase.
pub const CHAR_SET_ASCII_LETTERS: AsciiCharSetFilter<52> = AsciiCharSetFilter::new([
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
]);

/// ASCII letters, lowercase.
pub const CHAR_SET_ASCII_LETTERS_LOWER: AsciiCharSetFilter<26> = AsciiCharSetFilter::new([
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
]);

/// ASCII letters, uppercase.
pub const CHAR_SET_ASCII_LETTERS_UPPER: AsciiCharSetFilter<26> = AsciiCharSetFilter::new([
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
]);

/// ASCII letters and digits.
pub const CHAR_SET_ASCII_LETTERS_DIGITS: AsciiCharSetFilter<62> = AsciiCharSetFilter::new([
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b',
    'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u',
    'v', 'w', 'x', 'y', 'z',
]);

#[cfg(test)]
mod tests {
    extern crate std;

    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case('a', true)]
    #[case('1', true)]
    #[case(',', true)]
    #[case('あ', true)]
    #[case('-', false)]
    #[case('b', false)]
    #[case('2', false)]
    #[case('い', false)]
    #[case('A', false)]
    #[case('B', false)]
    fn test_char_set_filter(#[case] input: char, #[case] in_set: bool) {
        let filter = CharSetFilter::<_>::new(['a', '1', ',', 'あ']);
        assert_eq!(filter.filter(&input), in_set);
    }

    // `AsciiCharSetFilter::new` is fully evaluated at compile time here —
    // if any of these chars were non-ASCII this wouldn't compile.
    const TEST_ASCII_FILTER: AsciiCharSetFilter<4> = AsciiCharSetFilter::new(['a', 'Z', '5', '!']);

    #[rstest]
    #[case('a', true)]
    #[case('Z', true)]
    #[case('5', true)]
    #[case('!', true)]
    #[case('b', false)]
    #[case('あ', false)] // non-ASCII query: must return false, not panic
    fn test_ascii_char_set_filter(#[case] input: char, #[case] in_set: bool) {
        assert_eq!(TEST_ASCII_FILTER.filter(&input), in_set);
        // `contains` is const, so this membership test can also be
        // evaluated entirely at compile time:
        assert_eq!(TEST_ASCII_FILTER.contains(input), in_set);
    }

    // `const CONST_CHECK: bool = TEST_ASCII_FILTER.contains('a');` would
    // work too, proving the whole lookup is compile-time evaluable.
    const _CONST_CHECK: bool = TEST_ASCII_FILTER.contains('a');

    #[test]
    fn non_ascii_table_panics_at_runtime() {
        // Outside a `const` context the same invariant is still enforced,
        // just as a runtime panic instead of a compile error.
        let result = std::panic::catch_unwind(|| AsciiCharSetFilter::new(['a', 'あ']));
        assert!(result.is_err());
    }
}
