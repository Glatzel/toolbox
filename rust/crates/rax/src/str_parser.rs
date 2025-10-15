pub mod filters;
pub mod rules;

extern crate alloc;
use alloc::string::String;
mod parse_opt;
use alloc::string::ToString;

pub use parse_opt::*;
pub use rules::{IRule, IStrFlowRule, IStrGlobalRule};

use crate::RaxError;
/// Maintains parsing state for string-based parsers.
///
/// `StrParserContext` stores the full input string and a pointer
/// to the remaining portion of the string that has not yet been consumed.
/// It provides utilities to take, skip, and apply rules sequentially.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StrParserContext {
    /// The full input string.
    full: String,
    /// Pointer to the remaining unconsumed portion of the input.
    rest: *const str,
}

impl Default for StrParserContext {
    fn default() -> Self { Self::new() }
}

impl StrParserContext {
    /// Creates a new empty parser context.
    pub fn new() -> Self {
        Self {
            full: String::new(),
            rest: "",
        }
    }

    /// Initializes the parser context with a given input string.
    ///
    /// Resets the `rest` pointer to the beginning of the input.
    pub fn init(&mut self, input: String) -> &mut Self {
        self.full = input;
        self.rest = self.full.as_str();
        self
    }

    /// Returns the full input string.
    pub fn full_str(&self) -> &str { self.full.as_str() }

    /// Returns the remaining unparsed portion of the input.
    ///
    /// # Safety
    ///
    /// Internally uses a raw pointer to the string slice.
    pub fn rest_str(&self) -> &str { unsafe { &*self.rest } }

    /// Resets the parser to the start of the input.
    pub fn reset(&mut self) -> &mut Self {
        self.rest = self.full.as_str();
        self
    }
}

impl<'a> StrParserContext {
    /// Attempts to take a value using a flow rule.
    ///
    /// Advances the parser's `rest` pointer if the rule matches.
    pub fn take<R>(&mut self, rule: &R) -> Option<R::Output>
    where
        R: IStrFlowRule<'a>,
    {
        match rule.apply(unsafe { &*self.rest }) {
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
                rule: rule.to_string(),
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
                rule: rule.to_string(),
            }),
        }
    }

    /// Applies a global rule to the full input.
    ///
    /// Unlike flow rules, global rules operate on the entire input
    /// and do not modify the parser's `rest` pointer.
    pub fn global<R>(&'a mut self, rule: &R) -> R::Output
    where
        R: IStrGlobalRule<'a>,
    {
        rule.apply(&self.full)
    }
}
