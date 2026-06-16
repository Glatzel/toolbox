mod char_set;
use core::fmt::{Debug, Display};

pub use self::char_set::*;

/// Trait representing a generic filter over some input type `I`.
///
/// Implementors define the `filter` method to determine whether a given
/// input satisfies the filter criteria.
pub trait IFilter<I>: Debug + Display {
    /// Returns `true` if the input passes the filter, `false` otherwise.
    fn filter(&self, input: I) -> bool;
}
