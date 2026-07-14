use crate::string::filters::IFilter;
extern crate alloc;

/// A fixed, sorted set of characters for efficient membership testing.
///
/// The `table` must be sorted and contain unique characters. The `filter`
/// method uses a simple linear search, which is effectively O(N) but very
/// fast for small sets and `const` friendly. No nightly features are required.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CharSetFilter<const N: usize> {
    table: [char; N],
}

impl<const N: usize> CharSetFilter<N> {
    /// Creates a new `CharSetFilter`.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `table` is sorted and contains unique
    /// characters.
    pub const fn new(table: [char; N]) -> Self { Self { table } }
}

impl<const N: usize> IFilter<&char> for CharSetFilter<N> {
    /// Returns `true` if the character is in the set, `false` otherwise.
    fn filter(&self, input: &char) -> bool {
        clerk::trace!(
            "CharSetFilter: checking if '{}' is in the set {:?}",
            input,
            self.table
        );
        self.table.contains(input)
    }
}

// Predefined filters

/// Digits 0–9.
pub const DIGITS: CharSetFilter<10> =
    CharSetFilter::new(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']);

/// ASCII letters, uppercase and lowercase.
pub const ASCII_LETTERS: CharSetFilter<52> = CharSetFilter::new([
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
]);

/// ASCII letters and digits.
pub const ASCII_LETTERS_DIGITS: CharSetFilter<62> = CharSetFilter::new([
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b',
    'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u',
    'v', 'w', 'x', 'y', 'z',
]);

#[cfg(test)]
mod tests {
    extern crate std;

    use clerk::{LevelFilter, init_log_with_level};

    use super::*;
    #[test]
    fn test_char_set_filter() {
        init_log_with_level(LevelFilter::TRACE);
        let filter = CharSetFilter::<_>::new(['a', '1', ',', 'あ']);
        assert!(filter.filter(&'a'));
        assert!(filter.filter(&','));
        assert!(filter.filter(&'あ'));

        assert!(!filter.filter(&'b'));
        assert!(!filter.filter(&'2'));
        assert!(!filter.filter(&'-'));
        assert!(!filter.filter(&'い'));
    }
}
