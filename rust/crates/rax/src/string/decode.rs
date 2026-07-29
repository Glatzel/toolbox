extern crate alloc;
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
    cursor: usize,
    is_ascii: bool,
}

impl<'a> Decoder<'a> {
    pub fn new<S: AsRef<str> + ?Sized>(input: &'a S) -> Self {
        let s = input.as_ref();
        Self {
            full: s,
            cursor: 0,
            is_ascii: s.is_ascii(),
        }
    }

    /// Returns the full input string.
    pub fn full_str(&self) -> &str { self.full }

    /// Returns the remaining unparsed portion of the input.
    ///
    /// # Safety
    ///
    /// Internally uses a raw pointer to the string slice.
    pub fn rest_str(&self) -> &str { unsafe { self.full.get_unchecked(self.cursor..) } }

    /// Resets the parser to the start of the input.
    pub fn reset(&mut self) -> &mut Self {
        self.cursor = 0;
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
        R: IStrFlowRule<'a>,
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
    pub fn global<R>(&mut self, rule: &R) -> Result<R::Output, VerbError>
    where
        R: IGlobalRule<'a>,
    {
        rule.apply(self.full)
            .map_err(|e| e.to_verb::<R>(Verb::Global, self.full))
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
