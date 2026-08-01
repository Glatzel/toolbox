extern crate alloc;
use core::fmt::Debug;

mod byte_count;
mod char;
mod char_count;
mod n_in_char_set;
mod one_in_char_set;
mod until_char;
mod until_n_in_char_set;
mod until_not_in_char_set;
mod until_one_in_char_set;
mod until_str;

pub use byte_count::*;
pub use char_count::*;
pub use n_in_char_set::*;
pub use one_in_char_set::*;
pub use until_char::*;
pub use until_n_in_char_set::*;
pub use until_not_in_char_set::*;
pub use until_one_in_char_set::*;
pub use until_str::*;

pub use self::char::*;
use crate::error::RuleError;

/// Determines how a parser should treat the delimiter when splitting strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum::AsRefStr)]
pub enum UntilMode {
    /// Drop the delimiter completely → result like ("a", "b")
    #[strum(serialize = "discard")]
    Discard,
    /// Keep the delimiter on the left side → result like ("a,", "b")
    #[strum(serialize = "keep_left")]
    KeepInOutput,
    /// Keep the delimiter on the right side → result like ("a", ",b")
    #[strum(serialize = "keep_right")]
    KeepInRest,
}
impl UntilMode {
    pub fn split_str(self, input: &str, left: usize, length: usize) -> (&str, usize) {
        unsafe {
            match self {
                Self::Discard => (input.get_unchecked(..left), left + length),
                Self::KeepInOutput => {
                    let idx = left + length;
                    (input.get_unchecked(..idx), idx)
                }
                Self::KeepInRest => (input.get_unchecked(..left), left),
            }
        }
    }
}

/// Base trait for all parser rules.
pub trait IRule {
    fn type_name() -> &'static str { core::any::type_name::<Self>() }
}

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
    fn apply(&self, input: &'a str, is_ascii: bool) -> Result<(Self::Output, usize), RuleError>;
}

/// Trait for rules that operate on the entire input (global rules).
///
/// Global rules return a value based on the full input string
/// and do not consume or track the remaining input.
pub trait IGlobalRule<'a>: IRule {
    /// Type of the value produced by this rule.
    type Output;

    /// Apply the rule to the full input.
    fn apply(&self, input: &'a str) -> Result<Self::Output, RuleError>;
}
