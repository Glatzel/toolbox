extern crate alloc;
use alloc::vec::Vec;

use crate::protocol::{IIndent, IOwnedTree, IStyledOwnedTree, ITreeContent};

#[derive(Debug, Clone)]
pub struct OwnedTree<D: AsRef<str>> {
    content: D,
    leaves: Vec<OwnedTree<D>>,
}
impl<D: AsRef<str>> ITreeContent for OwnedTree<D> {
    fn content(&self) -> impl AsRef<str> { self.content.as_ref() }
}
impl<D: AsRef<str>> IOwnedTree for OwnedTree<D> {
    type Leaf = OwnedTree<D>;
    type Leaves<'a>
        = core::slice::Iter<'a, Self::Leaf>
    where
        Self: 'a;
    fn leaves(&self) -> Self::Leaves<'_> { self.leaves.iter() }
}
impl<D: AsRef<str>> OwnedTree<D> {
    /// Creates a new tree node with no children.
    pub fn new(content: D) -> Self {
        Self {
            content,
            leaves: Vec::new(),
        }
    }
    /// Creates a node and attaches a set of child nodes.
    ///
    /// Any type convertible into `Tree<D>` may be provided.
    pub fn with_leaves(
        mut self,
        leaves: impl IntoIterator<Item = impl Into<OwnedTree<D>>>,
    ) -> Self {
        self.leaves = leaves.into_iter().map(Into::into).collect();
        self
    }
    /// Appends a child node.
    ///
    /// Returns the node itself to support method chaining.
    pub fn push(&mut self, leaf: impl Into<OwnedTree<D>>) -> &mut Self {
        self.leaves.push(leaf.into());
        self
    }
}

impl<D: AsRef<str>> From<D> for OwnedTree<D> {
    fn from(value: D) -> Self { OwnedTree::new(value) }
}

#[derive(Debug, Clone)]
pub struct StyledOwnedTree<D: AsRef<str>, I: IIndent> {
    content: D,
    leaves: Vec<StyledOwnedTree<D, I>>,
    indent: Option<I>,
}
impl<D: AsRef<str>, I: IIndent> ITreeContent for StyledOwnedTree<D, I> {
    fn content(&self) -> impl AsRef<str> { self.content.as_ref() }
}
impl<D: AsRef<str>, I: IIndent> IOwnedTree for StyledOwnedTree<D, I> {
    type Leaf = StyledOwnedTree<D, I>;
    type Leaves<'a>
        = core::slice::Iter<'a, Self::Leaf>
    where
        Self: 'a;

    fn leaves(&self) -> Self::Leaves<'_> { self.leaves.iter() }
}

impl<D: AsRef<str>, I: IIndent> IStyledOwnedTree for StyledOwnedTree<D, I> {
    type Indent = I;

    /// Returns the indentation style override for this node.
    ///
    /// If `None`, the renderer should inherit the indentation style
    /// from the parent or use its default configuration.
    fn indent(&self) -> &Option<I> { &self.indent }
}

impl<D: AsRef<str>, I: IIndent + Clone> StyledOwnedTree<D, I> {
    /// Creates a new node without a custom indentation style.
    pub fn new(content: D) -> Self {
        Self {
            content,
            leaves: Vec::new(),
            indent: None,
        }
    }

    /// Creates a new node with a specific indentation style.
    pub fn new_with_indent(content: D, indent: I) -> Self {
        Self {
            content,
            leaves: Vec::new(),
            indent: Some(indent),
        }
    }

    /// Attaches a collection of child nodes.
    ///
    /// Each element must be convertible into `ComplexTree<D, I>`.
    pub fn with_leaves(
        mut self,
        leaves: impl IntoIterator<Item = impl Into<StyledOwnedTree<D, I>>>,
    ) -> Self {
        self.leaves = leaves.into_iter().map(Into::into).collect();
        self
    }

    /// Assigns a custom indentation style to the node.
    ///
    /// This overrides the indentation used when rendering the subtree.
    pub fn with_indent(mut self, indent: I) -> Self {
        self.indent = Some(indent);
        self
    }

    /// Appends a child node.
    pub fn push(&mut self, leaf: impl Into<StyledOwnedTree<D, I>>) -> &mut Self {
        self.leaves.push(leaf.into());
        self
    }
}

/// Allows creating a node directly from content.
impl<D: AsRef<str>, I: IIndent + Clone> From<D> for StyledOwnedTree<D, I> {
    fn from(value: D) -> Self { StyledOwnedTree::new(value) }
}
