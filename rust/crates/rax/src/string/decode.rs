extern crate alloc;
use alloc::string::ToString;

use crate::RaxError;
use crate::string::{IGlobalRule, IStrFlowRule};
pub trait IDecode<E>: Sized {
    fn decode(parser: &mut Decoder) -> Result<Self, E>;
}
/// Maintains parsing state for string-based parsers.
///
/// [`Decoder`] stores the full input string and a pointer
/// to the remaining portion of the string that has not yet been consumed.
/// It provides utilities to take, skip, and apply rules sequentially.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Decoder<'a> {
    /// The full input string.
    full: &'a str,
    /// Pointer to the remaining unconsumed portion of the input.
    rest: &'a str,
}

impl<'a> Decoder<'a> {
    pub fn new<S: AsRef<str> + ?Sized>(input: &'a S) -> Self {
        let s = input.as_ref();
        Self { full: s, rest: s }
    }

    /// Returns the full input string.
    pub fn full_str(&self) -> &str { self.full }

    /// Returns the remaining unparsed portion of the input.
    ///
    /// # Safety
    ///
    /// Internally uses a raw pointer to the string slice.
    pub fn rest_str(&self) -> &str { self.rest }

    /// Resets the parser to the start of the input.
    pub fn reset(&mut self) -> &mut Self {
        self.rest = self.full;
        self
    }
}

impl<'a> Decoder<'a> {
    /// Attempts to take a value using a flow rule.
    ///
    /// Advances the parser's `rest` pointer if the rule matches.
    pub fn take<R>(&mut self, rule: &R) -> Option<R::Output>
    where
        R: IStrFlowRule<'a>,
    {
        match rule.apply(self.rest) {
            (Some(result), rest) => {
                self.rest = rest;
                Some(result)
            }
            (None, rest) => {
                self.rest = rest;
                None
            }
        }
    }

    /// Strictly takes a value using a flow rule.
    ///
    /// Returns an error if the rule does not match.
    pub fn take_strict<R>(&mut self, rule: &'a R) -> Result<R::Output, RaxError>
    where
        R: IStrFlowRule<'a>,
    {
        match self.take(rule) {
            Some(s) => Ok(s),
            None => Err(RaxError::VerbError {
                verb: "TakeStrict".to_string(),
                rule: core::any::type_name::<R>(),
            }),
        }
    }

    /// Skips input matching a rule, ignoring the output.
    ///
    /// Advances the `rest` pointer regardless of match success.
    pub fn skip<R>(&mut self, rule: &R) -> &mut Self
    where
        R: IStrFlowRule<'a>,
    {
        self.take(rule);
        self
    }

    /// Strictly skips input matching a rule.
    ///
    /// Returns an error if the rule does not match.
    pub fn skip_strict<R>(&mut self, rule: &'a R) -> Result<&mut Self, RaxError>
    where
        R: IStrFlowRule<'a>,
    {
        match self.take_strict(rule) {
            Ok(_) => Ok(self),
            Err(_) => Err(RaxError::VerbError {
                verb: "SkipStrict".to_string(),
                rule: core::any::type_name::<R>(),
            }),
        }
    }

    /// Applies a global rule to the full input.
    ///
    /// Unlike flow rules, global rules operate on the entire input
    /// and do not modify the parser's `rest` pointer.
    pub fn global<R>(&mut self, rule: &R) -> Result<R::Output, R::Error>
    where
        R: IGlobalRule<'a>,
    {
        rule.apply(self.full)
    }
}
impl<'a> Decoder<'a> {
    pub fn decode<D, E>(&mut self) -> Result<D, E>
    where
        D: IDecode<E>,
    {
        D::decode(self)
    }
}
