use core::fmt::Debug;

use crate::error::VerbError;
use crate::text::{IFlowRule, IGlobalRule};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Verb {
    Take,
    Skip,
    Global,
}

pub trait IParseStr<E>: Sized {
    fn parse_str(parser: &mut StrParser<'_>) -> Result<Self, E>;
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
    is_ascii: bool,
}

impl<'a> StrParser<'a> {
    pub fn parse<D, E>(&mut self) -> Result<D, E>
    where
        D: IParseStr<E>,
    {
        D::parse_str(self)
    }
}

impl<'a> StrParser<'a> {
    pub fn new<S>(input: &'a S) -> Self
    where
        S: AsRef<str> + ?Sized,
    {
        let s = input.as_ref();
        Self {
            full: s,
            cursor: 0,
            is_ascii: s.is_ascii(),
        }
    }

    pub fn set_str(&mut self, input: &'a str) -> &mut Self {
        self.full = input;
        self.cursor = 0;
        self.is_ascii = input.is_ascii();
        self
    }

    /// Returns the full input string.
    pub const fn full_str(&self) -> &str { self.full }

    /// Returns the remaining unparsed portion of the input.
    ///
    /// # Safety
    ///
    /// Internally uses a raw pointer to the string slice.
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
    /// Strictly takes a value using a flow rule.
    ///
    /// Returns an error if the rule does not match.
    pub fn take<R>(&mut self, rule: &R) -> Result<R::Output<'a>, VerbError>
    where
        R: IFlowRule,
    {
        match rule.apply(
            unsafe { self.full.get_unchecked(self.cursor..) },
            self.is_ascii,
        ) {
            Ok((v, advanced)) => {
                self.cursor += advanced;
                Ok(v)
            }
            Err(e) => Err(e.to_verb::<R>(Verb::Take, self.rest_str())),
        }
    }

    /// Strictly skips input matching a rule.
    ///
    /// Returns an error if the rule does not match.
    pub fn skip<R>(&mut self, rule: &R) -> Result<&mut Self, VerbError>
    where
        R: IFlowRule,
    {
        match rule.apply(
            unsafe { self.full.get_unchecked(self.cursor..) },
            self.is_ascii,
        ) {
            Ok((_, advanced)) => {
                self.cursor += advanced;
                Ok(self)
            }
            Err(e) => Err(e.to_verb::<R>(Verb::Skip, self.rest_str())),
        }
    }

    /// Applies a global rule to the full input.
    ///
    /// Unlike flow rules, global rules operate on the entire input
    /// and do not modify the parser's `rest` pointer.
    pub fn global<R>(&mut self, rule: &R) -> Result<R::Output<'_>, VerbError>
    where
        R: IGlobalRule,
    {
        rule.apply(self.full)
            .map_err(|e| e.to_verb::<R>(Verb::Global, self.full))
    }
}

impl StrParser<'_> {
    pub fn decode<D, E>(&mut self) -> Result<D, E>
    where
        D: IParseStr<E>,
    {
        D::parse_str(self)
    }
}
