extern crate alloc;
use alloc::string::ToString;
use core::fmt::Debug;

use crate::error::VerbError;
use crate::string::{IGlobalRule, IStrFlowRule};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Verb {
    Take,
    Skip,
    Global,
}
pub trait IDecode<E>: Sized {
    fn decode(parser: &mut Decoder) -> Result<Self, E>;
}
/// Maintains parsing state for string-based parsers.
///
/// [`Decoder`] stores the full input string and a pointer
/// to the remaining portion of the string that has not yet been consumed.
/// It provides utilities to take, skip, and apply rules sequentially.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
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
    /// Strictly takes a value using a flow rule.
    ///
    /// Returns an error if the rule does not match.
    pub fn take<R>(&mut self, rule: &R) -> Result<R::Output, VerbError>
    where
        R: IStrFlowRule<'a>,
    {
        match rule.apply(self.rest) {
            Ok((v, rest)) => {
                self.rest = rest;
                Ok(v)
            }
            Err(e) => Err(VerbError {
                verb: Verb::Take,
                rule: R::type_name(),
                input: self.rest.to_string(),
                rule_error: e,
            }),
        }
    }

    /// Strictly skips input matching a rule.
    ///
    /// Returns an error if the rule does not match.
    pub fn skip<R>(&mut self, rule: &R) -> Result<&mut Self, VerbError>
    where
        R: IStrFlowRule<'a>,
    {
        match rule.apply(self.rest) {
            Ok((_, rest)) => {
                self.rest = rest;
                Ok(self)
            }
            Err(e) => Err(VerbError {
                verb: Verb::Skip,
                rule: R::type_name(),
                input: self.rest.to_string(),
                rule_error: e,
            }),
        }
    }

    /// Applies a global rule to the full input.
    ///
    /// Unlike flow rules, global rules operate on the entire input
    /// and do not modify the parser's `rest` pointer.
    pub fn global<R>(&mut self, rule: &R) -> Result<R::Output, VerbError>
    where
        R: IGlobalRule<'a>,
    {
        rule.apply(self.full).map_err(|e| VerbError {
            verb: Verb::Global,
            rule: R::type_name(),
            input: self.full.to_string(),
            rule_error: e,
        })
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
