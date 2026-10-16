use core::fmt::Debug;

use crate::error::VerbError;
use crate::text::{IFlowRule, IGlobalRule};

/// The kind of parser operation through which a rule is applied.
///
/// Recorded in a [`VerbError`] to indicate how a rule failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Verb {
    /// Applies a flow rule and returns its output ([`StrParser::take`]).
    Take,
    /// Applies a flow rule and discards its output ([`StrParser::skip`]).
    Skip,
    /// Applies a global rule to the entire input ([`StrParser::global`]).
    Global,
}

/// Maintains parsing state for string-based parsers.
///
/// [`StrParser`] stores the full input string and a pointer
/// to the remaining portion of the string that has not yet been consumed.
/// It provides utilities to take, skip, and apply rules sequentially.
///
/// The lifetime `'a` is tied to the input string reference.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StrParser<'a> {
    /// The full input string.
    full: &'a str,
    /// Pointer to the remaining unconsumed portion of the input.
    cursor: usize,
}

impl<'a> StrParser<'a> {
    /// Creates a parser over `input`, with the cursor at the start.
    pub fn new<S>(input: &'a S) -> Self
    where
        S: AsRef<str> + ?Sized,
    {
        let s = input.as_ref();
        Self { full: s, cursor: 0 }
    }

    /// Replaces the parsed input and resets the cursor to the start.
    pub const fn set_str(&mut self, input: &'a str) -> &mut Self {
        self.full = input;
        self.cursor = 0;
        self
    }

    /// Returns the full input string.
    pub const fn full_str(&self) -> &str { self.full }

    /// Returns the remaining unparsed portion of the input.
    pub fn rest_str(&self) -> &str { unsafe { self.full.get_unchecked(self.cursor..) } }

    /// Resets the parser to the start of the input.
    pub const fn reset(&mut self) -> &mut Self {
        self.cursor = 0;
        self
    }
}

impl Default for StrParser<'_> {
    fn default() -> Self { Self::new("") }
}

impl<'a> StrParser<'a> {
    /// Applies a flow rule and consumes its matched prefix.
    ///
    /// # Returns
    ///
    /// The value produced by `rule`; the cursor advances by the number of
    /// bytes the rule consumed.
    ///
    /// # Errors
    ///
    /// Returns a [`VerbError`] wrapping the rule's
    /// [`RuleError`](crate::error::RuleError) if `rule` does not match, leaving
    /// the cursor unchanged.
    pub fn take<R, const IS_ASCII: bool>(&mut self, rule: &R) -> Result<R::Output<'a>, VerbError>
    where
        R: IFlowRule<IS_ASCII>,
    {
        match rule.apply(unsafe { self.full.get_unchecked(self.cursor..) }) {
            Ok((v, advanced)) => {
                self.cursor += advanced;
                Ok(v)
            }
            Err(e) => Err(e.to_verb::<R>(Verb::Take, self.rest_str())),
        }
    }

    /// Applies a flow rule and discards its matched prefix.
    ///
    /// # Returns
    ///
    /// The parser itself, for chaining; the cursor advances by the number of
    /// bytes the rule consumed.
    ///
    /// # Errors
    ///
    /// Returns a [`VerbError`] if `rule` does not match, leaving the cursor
    /// unchanged.
    pub fn skip<R, const IS_ASCII: bool>(&mut self, rule: &R) -> Result<&mut Self, VerbError>
    where
        R: IFlowRule<IS_ASCII>,
    {
        match rule.apply(unsafe { self.full.get_unchecked(self.cursor..) }) {
            Ok((_, advanced)) => {
                self.cursor += advanced;
                Ok(self)
            }
            Err(e) => Err(e.to_verb::<R>(Verb::Skip, self.rest_str())),
        }
    }

    /// Applies a global rule to the whole of `input`.
    ///
    /// Unlike flow rules, global rules do not consume input and do not affect
    /// the parser cursor.
    ///
    /// # Returns
    ///
    /// The value produced by `rule`.
    ///
    /// # Errors
    ///
    /// Returns a [`VerbError`] if `rule` does not match.
    pub fn global<R, const IS_ASCII: bool>(
        input: &'a str,
        rule: &R,
    ) -> Result<R::Output<'a>, VerbError>
    where
        R: IGlobalRule<IS_ASCII>,
    {
        rule.apply(input)
            .map_err(|e| e.to_verb::<R>(Verb::Global, input))
    }
}
