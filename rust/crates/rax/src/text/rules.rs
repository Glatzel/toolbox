//! Parser rules for splitting and matching text at the byte and character
//! level.
//!
//! The module defines the rule traits [`IRule`], [`IFlowRule`], and
//! [`IGlobalRule`], together with concrete rules such as [`Char`],
//! [`ByteCount`], and [`UntilStr`]. [`UntilMode`] controls how the `Until*`
//! family splits the input around a matched delimiter.

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
///
/// The variants differ in whether the delimiter is kept and on which side of
/// the split it ends up. [`UntilMode::split_str`] applies a mode to a raw
/// delimiter position and returns the resulting `(prefix, consumed)` pair.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum::AsRefStr)]
pub enum UntilMode {
    /// Drops the delimiter. Splitting `"a,b"` on `","` yields `("a", "b")`.
    #[strum(serialize = "discard")]
    Discard,
    /// Keeps the delimiter at the end of the prefix. Splitting `"a,b"` on
    /// `","` yields `("a,", "b")`.
    #[strum(serialize = "keep_left")]
    KeepInOutput,
    /// Keeps the delimiter at the start of the remainder. Splitting `"a,b"`
    /// on `","` yields `("a", ",b")`.
    #[strum(serialize = "keep_right")]
    KeepInRest,
}
impl UntilMode {
    /// Splits `input` around a delimiter that starts at byte index `left` and
    /// spans `length` bytes, according to this mode.
    ///
    /// # Returns
    ///
    /// A `(prefix, consumed)` pair, where `prefix` is the extracted prefix and
    /// `consumed` is the number of bytes taken from the start of `input`. The
    /// remainder is the slice `&input[consumed..]`.
    ///
    /// # Notes
    ///
    /// `left` and `left + length` must be valid UTF-8 boundary indices within
    /// `input`; the implementation slices without bounds or boundary checks.
    ///
    /// # Examples
    ///
    /// ```
    /// use rax::text::UntilMode;
    ///
    /// let (prefix, consumed) = UntilMode::Discard.split_str("a,b", 1, 1);
    /// assert_eq!(prefix, "a");
    /// assert_eq!(consumed, 2);
    /// ```
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
    /// Returns the fully qualified type name of the implementing rule.
    ///
    /// # Notes
    ///
    /// Useful for diagnostics and error messages.
    fn type_name() -> &'static str { core::any::type_name::<Self>() }
}

/// Trait for rules that consume a prefix of the input sequentially (flow
/// rules).
///
/// A flow rule inspects the start of the input and, on a match, returns the
/// produced value together with the number of bytes consumed. The remaining
/// input is the slice `&input[consumed..]`. A non-matching input yields a
/// [`RuleError`] rather than a zero-width match.
///
/// The `IS_ASCII` const parameter mirrors the rule's own type parameter: when
/// `true`, the rule may take an ASCII fast path.
pub trait IFlowRule<const IS_ASCII: bool>: IRule {
    /// Type of the value produced by this rule.
    type Output<'a>;

    /// Applies the rule to the given input.
    ///
    /// # Returns
    ///
    /// On a match, an `(output, consumed)` pair, where `output` is the produced
    /// value and `consumed` is the number of bytes taken from the start of
    /// `input`. The remainder is the slice `&input[consumed..]`.
    ///
    /// # Errors
    ///
    /// Returns [`RuleError`] if the input does not match this rule.
    fn apply<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, usize), RuleError>;
}

/// Trait for rules that operate on the entire input (global rules).
///
/// Global rules return a value derived from the full input string and do not
/// consume input or track a remainder. A non-matching input yields a
/// [`RuleError`].
///
/// The `IS_ASCII` const parameter mirrors the rule's own type parameter: when
/// `true`, the rule may take an ASCII fast path.
pub trait IGlobalRule<const IS_ASCII: bool>: IRule {
    /// Type of the value produced by this rule.
    type Output<'a>;

    /// Applies the rule to the full input.
    ///
    /// # Returns
    ///
    /// The value produced from the full input.
    ///
    /// # Errors
    ///
    /// Returns [`RuleError`] if the input does not match this rule.
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
