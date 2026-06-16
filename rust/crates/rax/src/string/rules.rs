extern crate alloc;
mod byte_count;
pub use byte_count::*;
mod char_count;
pub use char_count::*;
mod until_str;
pub use until_str::*;
mod one_in_char_set;
pub use one_in_char_set::*;
mod char;
pub use self::char::*;
mod until_one_in_char_set;
pub use until_one_in_char_set::*;
mod until_not_in_char_set;
pub use until_not_in_char_set::*;
mod until_n_in_char_set;
pub use until_n_in_char_set::*;
mod n_in_charset;
pub use n_in_charset::*;
mod until_char;

pub use until_char::*;

/// Determines how a parser should treat the delimiter when splitting strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum::AsRefStr)]
pub enum UntilMode {
    /// Drop the delimiter completely → result like ("a", "b")
    #[strum(serialize = "discard")]
    Discard,
    /// Keep the delimiter on the left side → result like ("a,", "b")
    #[strum(serialize = "keep_left")]
    KeepLeft,
    /// Keep the delimiter on the right side → result like ("a", ",b")
    #[strum(serialize = "keep_right")]
    KeepRight,
}

/// Base trait for all parser rules.
pub trait IRule {}

/// Trait for rules that consume input sequentially (flow rules).
///
/// Flow rules operate on a slice of the input string and return
/// a tuple of the parsed value (or `None` if no match) and the
/// remaining unparsed string.
pub trait IStrFlowRule<'a>: IRule {
    /// Type of the value produced by this rule.
    type Output;

    /// Apply the rule to the given input.
    ///
    /// Returns `(Some(output), remaining)` if the rule matches,
    /// or `(None, remaining)` if it does not match.
    fn apply(&self, input: &'a str) -> (Option<Self::Output>, &'a str);
}

/// Trait for rules that operate on the entire input (global rules).
///
/// Global rules return a value based on the full input string
/// and do not consume or track the remaining input.
pub trait IGlobalRule<'a>: IRule {
    /// Type of the value produced by this rule.
    type Output;

    /// Apply the rule to the full input.
    fn apply(&self, input: &'a str) -> Self::Output;
}
