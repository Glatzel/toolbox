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

pub use byte_count::ByteCount;
pub use char_count::CharCount;
pub use n_in_char_set::NInCharSet;
pub use one_in_char_set::OneOfCharSet;
pub use until_char::UntilChar;
pub use until_n_in_char_set::UntilNInCharSet;
pub use until_not_in_char_set::UntilNotInCharSet;
pub use until_one_in_char_set::UntilOneInCharSet;
pub use until_str::UntilStr;

pub use self::char::Char;
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
pub trait IFlowRule<const IS_ASCII: bool>: IRule {
    /// Type of the value produced by this rule.
    type Output<'a>;

    /// Apply the rule to the given input.
    ///
    /// Returns `(Some(output), remaining)` if the rule matches,
    /// or `(None, remaining)` if it does not match.
    fn apply<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, usize), RuleError>;
}

/// Trait for rules that operate on the entire input (global rules).
///
/// Global rules return a value based on the full input string
/// and do not consume or track the remaining input.
pub trait IGlobalRule<const IS_ASCII: bool>: IRule {
    /// Type of the value produced by this rule.
    type Output<'a>;

    /// Apply the rule to the full input.
    fn apply<'a>(&self, input: &'a str) -> Result<Self::Output<'a>, RuleError>;
}
#[cfg(test)]
#[cfg_attr(test, macro_export)]
macro_rules! test_rule {
    ($name:ident, $input:expr, $rule:expr) => {
        #[test]
        fn $name() {
            let result = $rule
                .apply($input)
                .map(|(out, idx)| (out, $input.get(idx..).unwrap()));

            insta::assert_debug_snapshot!(stringify!($name), result);
        }
    };
}
