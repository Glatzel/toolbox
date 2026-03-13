extern crate alloc;
use alloc::vec::Vec;

use crate::protocol::{IComplexTree, IIndent, ITree};
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
            content,
            leaves: Vec::new(),
        }
    }
    pub fn with_leaves(mut self, leaves: impl IntoIterator<Item = impl Into<Tree<D>>>) -> Self {
        self.leaves = leaves.into_iter().map(Into::into).collect();
        self
    }
}
impl<D: AsRef<str>> From<D> for Tree<D> {
    fn from(value: D) -> Self { Tree::new(value) }
}

#[derive(Debug, Clone)]
pub struct ComplexTree<D: AsRef<str>, I: IIndent> {
    content: D,
    leaves: Vec<ComplexTree<D, I>>,
    indent: Option<I>,
}
impl<D: AsRef<str>, I: IIndent> ITree for ComplexTree<D, I> {
    type Leave = ComplexTree<D, I>;
    fn content(&self) -> &str { self.content.as_ref() }
    fn leaves(&self) -> &[Self::Leave] { &self.leaves }
}
impl<D: AsRef<str>, I: IIndent> IComplexTree for ComplexTree<D, I> {
    type Indent = I;
    fn indent(&self) -> &Option<I> { &self.indent }
}

impl<D: AsRef<str>, I: IIndent + Clone> ComplexTree<D, I> {
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
    pub fn with_leaves(
        mut self,
        leaves: impl IntoIterator<Item = impl Into<ComplexTree<D, I>>>,
    ) -> Self {
        self.leaves = leaves.into_iter().map(Into::into).collect();
        self
    }
    pub fn with_indent(mut self, indent: I) -> Self {
        self.indent = Some(indent);
        self
    }
}
impl<D: AsRef<str>, I: IIndent + Clone> From<D> for ComplexTree<D, I> {
    fn from(value: D) -> Self { ComplexTree::new(value) }
}
