mod char_set;
pub use self::char_set::*;

pub trait IFilter<I> {
    fn name(&self) -> &str;
    fn filter(&self, input: I) -> bool;
}
