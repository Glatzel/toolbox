pub mod filters;
pub mod rules;
extern crate alloc;
use alloc::string::String;
mod parse_opt;
pub use parse_opt::*;
pub use rules::{IRule, IStrFlowRule, IStrGlobalRule};

use crate::RaxError;

pub struct StrParserContext {
    full: String,
    rest: *const str,
}

impl Default for StrParserContext {
    fn default() -> Self { Self::new() }
}

impl StrParserContext {
    pub fn new() -> Self {
        Self {
            full: String::new(),
            rest: "",
        }
    }
    pub fn init(&mut self, input: String) -> &mut Self {
        self.full = input;
        self.rest = self.full.as_str();
        self
    }

    pub fn full_str(&self) -> &str { self.full.as_str() }
    pub fn rest_str(&self) -> &str { unsafe { &*self.rest } }
    pub fn reset(&mut self) -> &mut Self {
        self.rest = self.full.as_str();
        self
    }
}

impl<'a> StrParserContext {
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
}

impl<'a> StrParserContext {
    pub fn skip<R>(&mut self, rule: &R) -> &mut Self
    where
        R: IStrFlowRule<'a>,
    {
        self.take(rule);
        self
    }
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
}
impl<'a> StrParserContext {
    pub fn global<R>(&'a mut self, rule: &R) -> R::Output
    where
        R: IStrGlobalRule<'a>,
    {
        rule.apply(&self.full)
    }
}
