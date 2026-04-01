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
pub trait ITreeContent {
    fn content(&self) -> impl AsRef<str>;
}

pub trait IOwnedTree: ITreeContent {
    type Leaf: IOwnedTree;
    type Leaves<'a>: DoubleEndedIterator<Item = &'a Self::Leaf>
    where
        Self: 'a;
    fn leaves(&self) -> Self::Leaves<'_>;
}

pub trait IStyledOwnedTree: IOwnedTree {
    type Indent: IIndent;
    fn indent(&self) -> &Option<Self::Indent>;
}

pub trait ILazyTree: ITreeContent {
    type Leaf: ILazyTree;
    type Leaves: DoubleEndedIterator<Item = Self::Leaf>;
    fn leaves(&self) -> Self::Leaves;
}
