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
