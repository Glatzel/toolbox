use crate::render::layer::Layer;

extern crate alloc;

pub trait IIndent {
    fn get(&self, node: &Layer) -> (&'static str, &'static str);
}
pub struct Indent;

impl IIndent for Indent {
    fn get(&self, node: &Layer) -> (&'static str, &'static str) {
        match node {
            Layer::Bottom => ("x ", "│ "),
            Layer::Middle => ("├─▶ ", "│   "),
            Layer::Top => ("╰─▶ ", "    "),
        }
    }
}
