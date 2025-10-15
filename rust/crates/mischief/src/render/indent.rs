use crate::render::position::{Item, Layer};

extern crate alloc;

/// Trait for computing indentation strings based on node layers and elements.
pub trait IIndent {
    /// Returns a tuple of `(prefix, continuation)` strings for the given node
    /// and element.
    fn get(&self, node: Layer, element: Item) -> (&'static str, &'static str);
}

/// Default implementation of `IIndent`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Indent;

impl IIndent for Indent {
    fn get(&self, node: Layer, element: Item) -> (&'static str, &'static str) {
        match (node, element) {
            (Layer::Bottom, Item::First) => ("x ", "│ "),
            (Layer::Bottom, Item::Other) => ("│ ", "│ "),
            (Layer::Middle, Item::First) => ("├─▶ ", "│   "),
            (Layer::Middle, Item::Other) => ("│   ", "│   "),
            (Layer::Top, Item::First) => ("╰─▶ ", "    "),
            (Layer::Top, Item::Other) => ("    ", "    "),
        }
    }
}
