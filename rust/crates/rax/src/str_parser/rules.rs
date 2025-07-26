mod byte_count;
use std::fmt::Display;

pub use byte_count::*;
mod char_count;
pub use char_count::*;
mod until_str;
pub use until_str::*;
mod one_in_char_set;
pub use one_in_char_set::*;
mod char;
pub use self::char::*;
mod until_one_in_char_set;
pub use until_one_in_char_set::*;
mod until_not_in_char_set;
pub use until_not_in_char_set::*;
mod until_n_in_char_set;
pub use until_n_in_char_set::*;
mod n_in_charset;
pub use n_in_charset::*;
mod until_char;
pub use until_char::*;
#[derive(Clone, Copy, Debug)]
pub enum UntilMode {
    /// Drop the delimiter completely → ("a", "b")
    Discard,
    /// Keep the delimiter on the left side → ("a,", "b")
    KeepLeft,
    /// Keep the delimiter on the right side → ("a", ",b")
    KeepRight,
}
impl Display for UntilMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UntilMode::Discard => write!(f, "Discard"),
            UntilMode::KeepLeft => write!(f, "KeepLeft"),
            UntilMode::KeepRight => write!(f, "KeepRight"),
        }
    }
}
pub trait IRule {
    fn name(&self) -> &str;
}
pub trait IStrFlowRule<'a>: IRule {
    type Output;
    fn apply(&self, input: &'a str) -> (Option<Self::Output>, &'a str);
}
pub trait IStrGlobalRule<'a>: IRule {
    type Output;
    fn apply(&self, input: &'a str) -> Self::Output;
}
