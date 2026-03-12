extern crate alloc;
use alloc::vec::Vec;

use crate::protocol::ITree;

#[derive(Debug, Clone)]
pub struct Tree<D: AsRef<str>> {
    content: D,
    leaves: Vec<Tree<D>>,
}
impl<D: AsRef<str>> ITree for Tree<D> {
    type Leave = Tree<D>;
    fn content(&self) -> &str { self.content.as_ref() }
    fn leaves(&self) -> &[Self::Leave] { &self.leaves }
}
impl<D: AsRef<str>> Tree<D> {
    pub fn new(content: D) -> Self {
        Self {
            content: content,
            leaves: Vec::new(),
        }
    }
    pub fn with_leaves(mut self, leaves: impl IntoIterator<Item = impl Into<Tree<D>>>) -> Self {
        self.leaves = leaves.into_iter().map(Into::into).collect();
        self
    }
}
impl<D> From<D> for Tree<D>
where
    D: AsRef<str>,
{
    fn from(value: D) -> Self { Tree::new(value) }
}
