use crate::protocol::{IIndent, Layer, Line};

pub struct UnicodeIndent;
impl IIndent for UnicodeIndent {
    fn get_indent(&self, layer: Layer, line: Line) -> &'static str {
        match (layer, line) {
            (Layer::Root, Line::First) => "",
            (Layer::Root, Line::Other) => "",
            (Layer::Middle | Layer::Top, Line::First) => "├─▶ ",
            (Layer::Bottom, Line::First) => "╰─▶ ",
            (Layer::Bottom, Line::Other) => "    ",
            (_, Line::Other) => "│   ",
        }
    }
}

pub struct SpaceIndent;
impl IIndent for SpaceIndent {
    fn get_indent(&self, layer: Layer, line: Line) -> &'static str {
        match (layer, line) {
            (Layer::Root, _) => "",
            _ => "    ",
        }
    }
}
pub struct AsciiIndent;
impl IIndent for AsciiIndent {
    fn get_indent(&self, layer: Layer, line: Line) -> &'static str {
        match (layer, line) {
            (Layer::Root, _) => "",
            (Layer::Middle | Layer::Top, Line::First) => "|-- ",
            (Layer::Bottom, Line::First) => "`-- ",
            (Layer::Bottom, Line::Other) => "    ",
            (_, Line::Other) => "|   ",
        }
    }
}
#[cfg(debug_assertions)]
pub struct DebugIndent;

#[cfg(debug_assertions)]
impl IIndent for DebugIndent {
    fn get_indent(&self, layer: Layer, line: Line) -> &'static str {
        match (layer, line) {
            (Layer::Root, Line::First) => "(Root:First)",
            (Layer::Root, Line::Other) => "(Root:Other)",
            (Layer::Top, Line::First) => "(Top:First)",
            (Layer::Top, Line::Other) => "(Top:Other)",
            (Layer::Middle, Line::First) => "(Middle:First)",
            (Layer::Middle, Line::Other) => "(Middle:Other)",
            (Layer::Bottom, Line::First) => "(Bottom:First)",
            (Layer::Bottom, Line::Other) => "(Bottom:Other)",
        }
    }
}
