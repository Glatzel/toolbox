//! Text parsing built from composable rules and character filters.
//!
//! [`StrParser`] wraps an input string and applies flow rules sequentially,
//! while [`Verb`] identifies which kind of operation produced a
//! [`VerbError`](crate::error::VerbError). The [`rules`] module defines the
//! rule traits and concrete rules; [`filters`] provides the character-set
//! filters those rules consume.

pub mod filters;
mod parser;
pub mod rules;

pub use parser::{StrParser, Verb};
pub use rules::{
    ByteCount, Char, CharCount, IFlowRule, IGlobalRule, IRule, NInCharSet, OneOfCharSet, UntilChar,
    UntilMode, UntilNInCharSet, UntilNotInCharSet, UntilOneInCharSet, UntilStr,
};
