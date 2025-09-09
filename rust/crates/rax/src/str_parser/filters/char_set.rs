use std::str::FromStr;

use crate::str_parser::RaxError;
use crate::str_parser::filters::IFilter;

/// A fixed, sorted set of characters for efficient membership testing.
///
/// The `table` must be sorted and contain unique characters. The `filter`
/// method uses a simple linear search, which is effectively O(N) but very
/// fast for small sets and `const` friendly. No nightly features are required.
pub struct CharSetFilter<const N: usize> {
    table: [char; N],
}

impl<const N: usize> core::fmt::Debug for CharSetFilter<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "CharSetFilter<N={}>{{table: {:?}}}", N, &self.table)
    }
}

impl<const N: usize> core::fmt::Display for CharSetFilter<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { write!(f, "{:?}", self) }
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

impl<const N: usize> FromStr for CharSetFilter<N> {
    type Err = crate::str_parser::RaxError;

    /// Parses a string into a `CharSetFilter`.
    ///
    /// The string must have exactly `N` characters, otherwise a `RaxError` is
    /// returned.
    fn from_str(s: &str) -> Result<Self, crate::str_parser::RaxError> {
        let mut chars = [0 as char; N];
        let mut i = 0;
        for c in s.chars() {
            if i < N {
                chars[i] = c;
                i += 1;
            } else {
                return Err(RaxError::FilterError(format!(
                    "String too long for CharSet, expected {} but got {}",
                    N,
                    i + 1
                )));
            }
        }
        if i != N {
            return Err(RaxError::FilterError(format!(
                "String length does not match CharSet size, expected {} but got {}",
                N, i
            )));
        }
        Ok(Self::new(chars))
    }
}

/// Predefined filters

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

    use clerk::{LogLevel, init_log_with_level};

    use super::*;
    #[test]
    fn test_char_set_filter() -> mischief::Result<()> {
        init_log_with_level(LogLevel::TRACE);
        let filter = CharSetFilter::<3>::from_str("abc")?;
        assert!(filter.filter(&'a'));
        assert!(filter.filter(&'b'));
        assert!(filter.filter(&'c'));
        assert!(!filter.filter(&'d'));
        assert!(!filter.filter(&'1'));
        Ok(())
    }
    #[test]
    fn test_char_set_filter_from_str() -> mischief::Result<()> {
        init_log_with_level(LogLevel::TRACE);
        let filter: CharSetFilter<3> = CharSetFilter::from_str("abc")?;
        assert!(filter.filter(&'a'));
        assert!(filter.filter(&'b'));
        assert!(filter.filter(&'c'));
        assert!(!filter.filter(&'d'));
        assert!(!filter.filter(&'1'));

        let filter: CharSetFilter<2> = CharSetFilter::from_str(",*")?;
        assert!(filter.filter(&','));
        assert!(filter.filter(&'*'));
        Ok(())
    }
    #[test]
    fn test_char_set_filter_invalid_length() -> mischief::Result<()> {
        init_log_with_level(LogLevel::TRACE);
        let result = CharSetFilter::<3>::from_str("abcd");
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(
                format!("{:?}", e).contains("String too long for CharSet, expected 3 but got 4")
            );
        }
        Ok(())
    }
    #[test]
    fn test_char_set_filter_too_short() -> mischief::Result<()> {
        init_log_with_level(LogLevel::TRACE);
        let result = CharSetFilter::<3>::from_str("ab");
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(
                format!("{:?}", e)
                    .contains("String length does not match CharSet size, expected 3 but got 2")
            );
        }
        Ok(())
    }
    #[test]
    fn test_char_set_filter_empty() -> mischief::Result<()> {
        init_log_with_level(LogLevel::TRACE);
        let result = CharSetFilter::<3>::from_str("");
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(
                format!("{:?}", e)
                    .contains("String length does not match CharSet size, expected 3 but got 0")
            );
        }
        Ok(())
    }
    #[test]
    fn test_char_set_filter_invalid_chars() -> mischief::Result<()> {
        init_log_with_level(LogLevel::TRACE);
        let result = CharSetFilter::<3>::from_str("abce");
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(
                format!("{:?}", e).contains("String too long for CharSet, expected 3 but got 4")
            );
        }
        Ok(())
    }

    #[test]
    fn test_char_set_filter_unicode() -> mischief::Result<()> {
        init_log_with_level(LogLevel::TRACE);
        let filter = CharSetFilter::<3>::from_str("あいう")?;
        assert!(filter.filter(&'あ'));
        assert!(filter.filter(&'い'));
        assert!(filter.filter(&'う'));
        assert!(!filter.filter(&'え'));
        assert!(!filter.filter(&'1'));
        Ok(())
    }
    #[test]
    fn test_char_set_filter_unicode_invalid_length() -> mischief::Result<()> {
        init_log_with_level(LogLevel::TRACE);
        let result = CharSetFilter::<3>::from_str("あいうえ");
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(
                format!("{:?}", e).contains("String too long for CharSet, expected 3 but got 4")
            );
        }
        Ok(())
    }
    #[test]
    fn test_char_set_filter_unicode_too_short() -> mischief::Result<()> {
        init_log_with_level(LogLevel::TRACE);
        let result = CharSetFilter::<3>::from_str("あい");
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(
                format!("{:?}", e)
                    .contains("String length does not match CharSet size, expected 3 but got 2")
            );
        }
        Ok(())
    }
    #[test]
    fn test_char_set_filter_unicode_empty() -> mischief::Result<()> {
        init_log_with_level(LogLevel::TRACE);
        let result = CharSetFilter::<3>::from_str("");
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(
                format!("{:?}", e)
                    .contains("String length does not match CharSet size, expected 3 but got 0")
            );
        }
        Ok(())
    }
}
