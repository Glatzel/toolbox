extern crate alloc;
use alloc::vec::Vec;

use crate::protocol::{IIndent, IOwnedTree, IStyledOwnedTree, ITreeContent};

/// A simple owned tree node.
///
/// Each node stores textual content and a collection of child nodes.
/// The entire tree structure is owned in memory, and children are accessed
/// through iterators returning references.
///
/// The generic parameter `D` represents the node content type and must
/// implement [`AsRef<str>`], allowing flexibility such as `String`,
/// `&str`, or `Cow<str>`.
///
/// # Examples
///
/// ```
/// use arbor::indents::UnicodeIndent;
/// use arbor::renders::OwnedRender;
/// use arbor::trees::OwnedTree;
/// let tree = OwnedTree::new("foo").with_leaves(["bar", "baz"]);
/// let render = OwnedRender {
///     tree: &tree,
///     indent: UnicodeIndent,
///     width: 0,
/// };
/// println!("{}", render);
/// ```
#[derive(Debug, Clone)]
pub struct OwnedTree<D: AsRef<str>> {
    content: D,
    leaves: Vec<OwnedTree<D>>,
}

impl<D: AsRef<str>> ITreeContent for OwnedTree<D> {
    /// Returns the textual content of this node.
    fn content(&self) -> impl AsRef<str> { self.content.as_ref() }
}

impl<D: AsRef<str>> IOwnedTree for OwnedTree<D> {
    /// Child node type.
    type Leaf = OwnedTree<D>;

    /// Iterator over child nodes.
    type Leaves<'a>
        = core::slice::Iter<'a, Self::Leaf>
    where
        Self: 'a;

    /// Returns an iterator over the node's children.
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

    /// Creates a node and attaches a collection of child nodes.
    ///
    /// Each element must be convertible into `OwnedTree<D>`.
    /// This method follows a builder-style pattern and returns
    /// the modified node.
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

/// Allows constructing a node directly from content.
///
/// # Example
///
/// ```
/// use arbor::trees::OwnedTree;
/// let node: OwnedTree<&str> = "hello".into();
/// ```
impl<D: AsRef<str>> From<D> for OwnedTree<D> {
    fn from(value: D) -> Self { OwnedTree::new(value) }
}

/// A tree node that supports optional indentation styling.
///
/// This structure extends [`OwnedTree`] by allowing each node to specify
/// an optional indentation style used during rendering.
///
/// If `indent` is `None`, the renderer should inherit indentation behavior
/// from the parent node or fall back to its default configuration.
///
/// # examples
///
/// ```
/// use arbor::indents::UnicodeIndent;
/// use arbor::renders::StyledOwnedRender;
/// use arbor::trees::StyledOwnedTree;
/// let tree = StyledOwnedTree::new_with_indent("foo", UnicodeIndent).with_leaves(["bar", "baz"]);
/// let render = StyledOwnedRender {
///     tree: &tree,
///     width: 0,
/// };
/// println!("{}", render);
/// ```
#[derive(Debug, Clone)]
pub struct StyledOwnedTree<D: AsRef<str>, I: IIndent> {
    content: D,
    leaves: Vec<StyledOwnedTree<D, I>>,
    indent: Option<I>,
}

impl<D: AsRef<str>, I: IIndent> ITreeContent for StyledOwnedTree<D, I> {
    /// Returns the textual content of this node.
    fn content(&self) -> impl AsRef<str> { self.content.as_ref() }
}

impl<D: AsRef<str>, I: IIndent> IOwnedTree for StyledOwnedTree<D, I> {
    /// Child node type.
    type Leaf = StyledOwnedTree<D, I>;

    /// Iterator over child nodes.
    type Leaves<'a>
        = core::slice::Iter<'a, Self::Leaf>
    where
        Self: 'a;

    /// Returns an iterator over the node's children.
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
    /// Each element must be convertible into `StyledOwnedTree<D, I>`.
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
