mod char_set;
use core::fmt::{Debug, Display};

pub use self::char_set::*;

pub trait IFilter<I>: Debug + Display {
    fn filter(&self, input: I) -> bool;
}
