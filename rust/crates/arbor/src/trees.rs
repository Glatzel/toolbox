extern crate alloc;
use alloc::vec::Vec;

use crate::protocol::{IComplexTree, IIndent, ITree};

/// A simple generic tree structure.
///
/// `Tree` is a minimal implementation of [`ITree`] that stores textual
/// content and a list of child nodes.
///
/// It is intended for:
///
/// - testing tree renderers
/// - simple hierarchical data
/// - building diagnostic trees
///
/// The node content is stored as any type implementing `AsRef<str>`,
/// allowing flexible ownership models such as:
///
/// - `&'static str`
/// - `String`
/// - `Cow<'_, str>`
///
/// # examples
/// ```
/// use arbor::indents::UnicodeIndent;
/// use arbor::renders::Render;
/// use arbor::trees::Tree;
/// let tree = Tree::new("foo").with_leaves(["bar", "baz"]);
/// let render = Render {
///     tree: &tree,
///     indent: UnicodeIndent,
///     width: 0,
/// };
/// println!("{}", render);
/// ```
#[derive(Debug, Clone)]
pub struct Tree<D: AsRef<str>> {
    content: D,
    leaves: Vec<Tree<D>>,
}

impl<D: AsRef<str>> ITree for Tree<D> {
    type Leaf = Tree<D>;

    fn content(&self) -> impl AsRef<str> { self.content.as_ref() }

    fn leaves(&self) -> impl Iterator<Item = &Self::Leaf> { self.leaves.iter() }
}
impl<D: AsRef<str>> Tree<D> {
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
    pub fn with_leaves(mut self, leaves: impl IntoIterator<Item = impl Into<Tree<D>>>) -> Self {
        self.leaves = leaves.into_iter().map(Into::into).collect();
        self
    }
    /// Appends a child node.
    ///
    /// Returns the node itself to support method chaining.
    pub fn push(&mut self, leaf: impl Into<Tree<D>>) -> &mut Self {
        self.leaves.push(leaf.into());
        self
    }
}

/// Allows creating a tree node directly from content.
///
/// # Example
///
/// ```
/// use arbor::trees::Tree;
/// let node: Tree<&str> = "hello".into();
/// ```
impl<D: AsRef<str>> From<D> for Tree<D> {
    fn from(value: D) -> Self { Tree::new(value) }
}

/// A tree node that supports custom indentation styles.
///
/// `ComplexTree` extends [`Tree`] by allowing nodes to override the
/// indentation style used when rendering the subtree.
///
/// This is useful when different sections of a diagnostic tree should
/// use different visual formats.
///
/// # examples
/// ```
/// use arbor::indents::UnicodeIndent;
/// use arbor::renders::ComplexRender;
/// use arbor::trees::ComplexTree;
/// let tree = ComplexTree::new_with_indent("foo", UnicodeIndent).with_leaves(["bar", "baz"]);
/// let render = ComplexRender {
///     tree: &tree,
///     width: 0,
/// };
/// println!("{}", render);
/// ```
#[derive(Debug, Clone)]
pub struct ComplexTree<D: AsRef<str>, I: IIndent> {
    content: D,
    leaves: Vec<ComplexTree<D, I>>,
    indent: Option<I>,
}

impl<D: AsRef<str>, I: IIndent> ITree for ComplexTree<D, I> {
    type Leaf = ComplexTree<D, I>;

    fn content(&self) -> impl AsRef<str> { self.content.as_ref() }

    fn leaves(&self) -> impl Iterator<Item = &Self::Leaf> { self.leaves.iter() }
}

impl<D: AsRef<str>, I: IIndent> IComplexTree for ComplexTree<D, I> {
    type Indent = I;

    /// Returns the indentation style override for this node.
    ///
    /// If `None`, the renderer should inherit the indentation style
    /// from the parent or use its default configuration.
    fn indent(&self) -> &Option<I> { &self.indent }
}

impl<D: AsRef<str>, I: IIndent + Clone> ComplexTree<D, I> {
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
        leaves: impl IntoIterator<Item = impl Into<ComplexTree<D, I>>>,
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
    pub fn push(&mut self, leaf: impl Into<ComplexTree<D, I>>) -> &mut Self {
        self.leaves.push(leaf.into());
        self
    }
}

/// Allows creating a node directly from content.
impl<D: AsRef<str>, I: IIndent + Clone> From<D> for ComplexTree<D, I> {
    fn from(value: D) -> Self { ComplexTree::new(value) }
}
