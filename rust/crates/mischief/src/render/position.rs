/// Represents the position of a node in a hierarchical layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Layer {
    /// The bottom layer (start of the tree branch).
    Bottom,
    /// A middle layer (continuation of a branch).
    Middle,
    /// The top layer (end of a branch).
    Top,
}

/// Represents the position of an item within a layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Item {
    /// The first item in the layer.
    First,
    /// Any subsequent item in the layer.
    Other,
}
