//! Character-set filters used by the character-set rules.
//!
//! A filter answers whether a single character belongs to a set. [`CharSetFilter`]
//! stores an arbitrary set of characters, while [`AsciiCharSetFilter`] stores an
//! ASCII set backed by a 128-bit bitmap. [`ICharSetFilter`] is the trait both
//! implement, and the `CHAR_SET_*` constants provide common predefined sets.

mod char_set;

pub use self::char_set::{
    AsciiCharSetFilter, CHAR_SET_ASCII_LETTERS, CHAR_SET_ASCII_LETTERS_DIGITS,
    CHAR_SET_ASCII_LETTERS_LOWER, CHAR_SET_ASCII_LETTERS_UPPER, CHAR_SET_DIGITS, CharSetFilter,
    ICharSetFilter,
};

/// Trait representing a generic filter over some input type `I`.
///
/// Implementors define the `filter` method to determine whether a given
/// input satisfies the filter criteria.
pub trait IFilter<I> {
    /// Returns `true` if the input passes the filter, `false` otherwise.
    fn filter(&self, input: I) -> bool;
}
