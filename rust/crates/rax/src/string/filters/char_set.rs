use crate::string::filters::IFilter;

/// A fixed, sorted set of characters for efficient membership testing.
///
/// The `table` must be sorted and contain unique characters. The `filter`
/// method uses a simple linear search, which is effectively O(N) but very
/// fast for small sets and `const` friendly. No nightly features are required.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CharSetFilter<const N: usize> {
    table: [char; N],
    ascii_mask: Option<u128>,
}

impl<const N: usize> CharSetFilter<N> {
    /// # Safety
    ///
    /// The caller must guarantee that `table` is sorted and contains unique
    /// characters.
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
        let ascii_mask = Self::compute_ascii_mask(&table);
        Self { table, ascii_mask }
    }

    const fn compute_ascii_mask(table: &[char; N]) -> Option<u128> {
        let mut mask: u128 = 0;
        let mut i = 0;
        while i < N {
            let c = table[i];
            if !c.is_ascii() {
                return None;
            }
            mask |= 1_u128 << (c as u32);
            i += 1;
        }
        Some(mask)
    }

    pub const fn is_ascii(&self) -> bool { self.ascii_mask.is_some() }

    /// Cached bitmask — O(1) to read, computed once in `new`.
    pub const fn ascii_mask(&self) -> Option<u128> { self.ascii_mask }
}

impl<const N: usize> IFilter<&char> for CharSetFilter<N> {
    fn filter(&self, input: &char) -> bool {
        clerk::trace!(
            "CharSetFilter: checking if '{}' is in the set {:?}",
            input,
            self.table
        );
        self.table.binary_search(input).is_ok()
    }
}

// Predefined filters

/// Digits 0–9.
pub const CHAR_SET_DIGITS: CharSetFilter<10> =
    CharSetFilter::new(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']);

/// ASCII letters, uppercase and lowercase.
pub const CHAR_SET_ASCII_LETTERS: CharSetFilter<52> = CharSetFilter::new([
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
]);

/// ASCII letters and digits.
pub const CHAR_SET_ASCII_LETTERS_DIGITS: CharSetFilter<62> = CharSetFilter::new([
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b',
    'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u',
    'v', 'w', 'x', 'y', 'z',
]);

#[cfg(test)]
mod tests {
    extern crate std;

    use clerk::{LevelFilter, init_log_with_level};
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
        init_log_with_level(LevelFilter::TRACE);
        let filter = CharSetFilter::<_>::new(['a', '1', ',', 'あ']);
        assert_eq!(filter.filter(&input), in_set);
    }
}
