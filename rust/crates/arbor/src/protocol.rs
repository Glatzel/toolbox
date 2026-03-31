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
pub trait IIndent: Default + Clone {
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

/// Defines the basic structure required for a renderable tree.
///
/// A type implementing `ITree` represents a node with textual content
/// and zero or more child nodes. The trait is recursive, meaning that
/// each child (`Leave`) must also implement `ITree`.
///
/// This abstraction allows generic rendering of hierarchical data
/// structures such as:
///
/// - diagnostic reports
/// - error chains
/// - syntax trees
/// - hierarchical logs
pub trait ITree {
    /// The child node type.
    ///
    /// Each child must also implement [`ITree`], enabling recursive
    /// traversal of the tree.
    type Leaf: ITree;

    /// Returns the textual content associated with the node.
    ///
    /// This content is rendered after the indentation prefix.
    fn content(&self) -> impl AsRef<str>;
    /// Returns the child nodes of this element.
    ///
    /// An empty slice indicates that the node is a leaf.
    fn leaves(&self) -> impl Iterator<Item = &Self::Leaf>;
}

/// Extension of [`ITree`] that allows nodes to specify a custom
/// indentation style.
///
/// If an indentation style is provided, it overrides the renderer's
/// default indentation behavior for this node and its subtree.
pub trait IComplexTree: ITree {
    /// The indentation style used by the node.
    type Indent: IIndent;
    /// Returns the indentation style for this node.
    ///
    /// If `None` is returned, the renderer should fall back to the
    /// inherited or default indentation style.
    fn indent(&self) -> &Option<Self::Indent>;
}
