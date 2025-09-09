use crate::render::position::{Element, Layer};

extern crate alloc;

pub trait IIndent {
    fn get(&self, node: &Layer, element: &Element) -> (&'static str, &'static str);
}
pub struct Indent;

impl IIndent for Indent {
    fn get(&self, node: &Layer, element: &Element) -> (&'static str, &'static str) {
        match (node, element) {
            (Layer::Bottom, Element::First) =>  ("x ", "│ "),
            (Layer::Bottom, Element::Other) =>  ("│ ", "│ "),
            (Layer::Middle, Element::First) => ("├─▶ ", "│   "),
            (Layer::Middle, Element::Other) =>  ("│   ", "│   "),
            (Layer::Top, Element::First) => ("╰─▶ ", "    "),
            (Layer::Top, Element::Other) =>  ("    ", "    "),
        }
    }
}
