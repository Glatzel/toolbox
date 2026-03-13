/// Represents the position of a layer in a hierarchical layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Layer {
    /// The root layer .
    Root,
    /// The top layer.
    Top,
    /// A middle layer.
    Middle,
    /// The bottom layer.
    Bottom,
}

/// Represents the position of an item within a layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Line {
    /// The first item in the layer.
    First,
    /// Any subsequent item in the layer.
    Other,
}
pub trait IIndent: Default + Clone {
    /// Returns a tuple of `(prefix, continuation)` strings for the given layer
    /// and element.
    fn get_indent(&self, layer: Layer, line: Line) -> &str;
}
/// Trait defining rendering behavior for diagnostic types.
pub trait ITree {
    type Leave: ITree;
    fn content(&self) -> &str;
    fn leaves(&self) -> &[Self::Leave];
}

/// Trait defining rendering behavior for diagnostic types.

pub trait IComplexTree: ITree {
    type Indent: IIndent;
    fn indent(&self) -> &Option<Self::Indent>;
}
