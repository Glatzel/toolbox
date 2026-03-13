use crate::protocol::{IIndent, Layer, Line};
#[derive(Debug, Clone)]
pub struct UniversalIndent {
    pub root_first: &'static str,
    pub root_other: &'static str,
    pub top_first: &'static str,
    pub top_other: &'static str,
    pub mid_first: &'static str,
    pub mid_other: &'static str,
    pub bottom_first: &'static str,
    pub bottom_other: &'static str,
}
impl IIndent for UniversalIndent {
    fn get_indent(&self, layer: Layer, line: Line) -> &'static str {
        match (layer, line) {
            (Layer::Root, Line::First) => self.root_first,
            (Layer::Root, Line::Other) => self.root_other,
            (Layer::Top, Line::First) => self.top_first,
            (Layer::Top, Line::Other) => self.top_other,
            (Layer::Middle, Line::First) => self.mid_first,
            (Layer::Middle, Line::Other) => self.mid_other,
            (Layer::Bottom, Line::First) => self.bottom_first,
            (Layer::Bottom, Line::Other) => self.bottom_other,
        }
    }
}
impl Default for UniversalIndent {
    fn default() -> Self {
        Self {
            root_first: "",
            root_other: "",
            top_first: "|-- ",
            top_other: "|   ",
            mid_first: "`-- ",
            mid_other: "|   ",
            bottom_first: "    ",
            bottom_other: "    ",
        }
    }
}
#[derive(Debug, Clone, Default)]
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
#[derive(Debug, Clone, Default)]
pub struct SpaceIndent;
impl IIndent for SpaceIndent {
    fn get_indent(&self, layer: Layer, line: Line) -> &'static str {
        match (layer, line) {
            (Layer::Root, _) => "",
            _ => "    ",
        }
    }
}
#[derive(Debug, Clone, Default)]
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
#[derive(Debug, Clone, Default)]
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
