extern crate alloc;
use alloc::vec::Vec;

use crate::protocol::{IIndent, ITree};

#[derive(Debug, Clone)]
pub struct Tree<D: AsRef<str>, I: IIndent> {
    content: D,
    leaves: Vec<Tree<D, I>>,
    indent: Option<I>,
}
impl<D: AsRef<str>, I: IIndent> ITree for Tree<D, I> {
    type Leave = Tree<D, I>;
    type Indent = I;
    fn content(&self) -> &str { self.content.as_ref() }
    fn leaves(&self) -> &[Self::Leave] { &self.leaves }
    fn indent(&self) -> &Option<I> { &self.indent }
}

impl<D: AsRef<str>, I: IIndent + Clone> Tree<D, I> {
    pub fn new(content: D) -> Self {
        Self {
            content,
            leaves: Vec::new(),
            indent: None,
        }
    }
    pub fn new_with_indent(content: D, indent: I) -> Self {
        Self {
            content,
            leaves: Vec::new(),
            indent: Some(indent),
        }
    }
    pub fn with_leaves(mut self, leaves: impl IntoIterator<Item = impl Into<Tree<D, I>>>) -> Self {
        self.leaves = leaves.into_iter().map(Into::into).collect();
        self
    }
    pub fn with_indent(mut self, indent: I) -> Self {
        self.indent = Some(indent);
        self
    }
}
impl<D: AsRef<str>, I: IIndent + Clone> From<D> for Tree<D, I> {
    fn from(value: D) -> Self { Tree::new(value) }
}
