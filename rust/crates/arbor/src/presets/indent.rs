use crate::protocol::{IIndent, Layer, Line};

pub struct UnicodeIndent;
impl IIndent for UnicodeIndent {
    fn get_indent(&self, layer: Layer, line: Line) -> &'static str {
        match (layer, line) {
            (Layer::Root, _) => "",
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
            (Layer::Root, Line::First) => "[RF]",
            (Layer::Root, Line::Other) => "[RO]",
            (Layer::Top, Line::First) => "[TF]",
            (Layer::Top, Line::Other) => "[TO]",
            (Layer::Middle, Line::First) => "[MF]",
            (Layer::Middle, Line::Other) => "[MO]",
            (Layer::Bottom, Line::First) => "[BF]",
            (Layer::Bottom, Line::Other) => "[BO]",
        }
    }
}
