extern crate alloc;

/// Represents the vertical position of a node relative to its parent
/// in a hierarchical tree layout.
///
/// This information is primarily used by renderers to decide which
/// indentation or connector style should be emitted.
///
/// Typical usage in ASCII tree renderers:
///
/// ```text
/// Root
/// ├─ Top
/// │  ├─ Middle
/// │  └─ Bottom
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Layer {
    /// The root layer of the tree.
    ///
    /// This is the topmost node and has no parent.
    Root,
    
    /// The first child layer under a parent.
    ///
    /// Often rendered with a connector such as `├`.
    Top,
    
    /// A middle child layer between the first and last children.
    ///
    /// Used when a parent contains multiple intermediate nodes.
    Middle,
    
    /// The final child layer under a parent.
    ///
    /// Often rendered with a connector such as `└`.
    Bottom,
}

/// Represents the relative position of a line within the current layer.
///
/// This helps renderers determine whether indentation should continue
/// across subsequent lines.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Line {
    /// The first rendered line of the element.
    ///
    /// This is typically where the branch connector appears.
    First,

    /// Any subsequent lines belonging to the same element.
    ///
    /// These lines usually align with the content indentation rather
    /// than repeating the connector symbol.
    Other,
}

/// Defines how indentation strings are produced for tree rendering.
///
/// Implementations of this trait provide the textual prefix inserted
/// before each rendered line of a node. This allows renderers to support
/// multiple indentation styles such as:
///
/// - ASCII tree (`|--`)
/// - Unicode tree (`├─`, `└─`)
/// - Debug or structured formats
///
/// The returned string is expected to represent the indentation prefix
/// for the given `(layer, line)` combination.
pub trait IIndent: Default {
    /// Returns the indentation prefix for a specific `(layer, line)` pair.
    ///
    /// # Parameters
    ///
    /// * `layer` - The vertical position of the node relative to its parent.
    /// * `line` - Whether this is the first line or a continuation line.
    ///
    /// # Returns
    ///
    /// A string slice representing the prefix that should be written
    /// before the content of the line.
    fn get_indent(&self, layer: Layer, line: Line) -> &str;
}

/// Provides textual content for a tree node.
///
/// This trait represents the minimal interface for a tree element.
/// Each node must expose a textual representation through [`content`].
///
/// The returned value implements [`AsRef<str>`], allowing flexibility in
/// whether the content is stored as `String`, `&str`, or another string-like type.
pub trait ITreeContent {
    /// Returns the textual content associated with this node.
    fn content(&self) -> impl AsRef<str>;
}

/// A tree whose structure is owned and accessed through references.
///
/// This trait models a hierarchical structure where each node owns its
/// children and exposes them through an iterator of references. It is
/// designed for static or pre-built tree structures.
///
/// Implementors must define:
/// - [`Leaf`]: the concrete child node type
/// - [`Leaves`]: an iterator yielding references to child nodes
///
/// The iterator must implement [`DoubleEndedIterator`] to allow traversal
/// from both directions.
pub trait IOwnedTree: ITreeContent {
    /// The type of child nodes.
    type Leaf: IOwnedTree;

    /// Iterator over references to child nodes.
    ///
    /// This iterator is generic over a lifetime tied to the tree instance.
    type Leaves<'a>: DoubleEndedIterator<Item = &'a Self::Leaf>
    where
        Self: 'a;

    /// Returns an iterator over the node's children.
    fn leaves(&self) -> Self::Leaves<'_>;
}

/// Extension of [`IOwnedTree`] that supports styled indentation.
///
/// This trait allows nodes to optionally specify indentation information,
/// which can be used when rendering the tree structure (for example in
/// pretty printers or CLI tree views).
pub trait IStyledOwnedTree: IOwnedTree {
    /// The indentation type used for rendering.
    type Indent: IIndent;

    /// Returns the indentation configuration for this node.
    ///
    /// `None` indicates that the node should inherit or use default
    /// indentation behavior.
    fn indent(&self) -> &Option<Self::Indent>;
}

/// A tree where children are produced lazily.
///
/// Unlike [`IOwnedTree`], this trait returns child nodes by value rather
/// than by reference. This enables implementations where the tree is
/// generated dynamically or streamed rather than stored entirely in memory.
///
/// The iterator must implement [`DoubleEndedIterator`] to support both
/// forward and reverse traversal.
pub trait ILazyTree: ITreeContent {
    /// The type of child nodes produced by the iterator.
    type Leaf: ILazyTree;

    /// Iterator yielding child nodes.
    type Leaves: DoubleEndedIterator<Item = Self::Leaf>;

    /// Produces the children of this node.
    ///
    /// Each call may generate new nodes rather than referencing existing ones.
    fn leaves(&self) -> Self::Leaves;
}