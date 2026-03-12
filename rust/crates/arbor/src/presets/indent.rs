use crate::protocol::{IIndent, Layer, Line};

pub struct UnicodeIndent;
impl IIndent for UnicodeIndent {
    fn get_indent(&self, layer: Layer, line: Line) -> &'static str {
        match (layer, line) {
            (Layer::Root, Line::First) => "",
            (Layer::Middle | Layer::Top, Line::First) => "├─▶ ",
            (Layer::Bottom, Line::First) => "╰─▶ ",
            (Layer::Bottom, Line::Other) => "    ",
            (_, Line::Other) => "│   ",
        }
    }

    fn space(&self) -> &'static str { "    " }
}

pub struct SpaceIndent;
impl IIndent for SpaceIndent {
    fn get_indent(&self, layer: Layer, line: Line) -> &'static str {
        match (layer, line) {
            (Layer::Root, _) => "",
            (Layer::Top, Line::First) => "  ",
            (Layer::Top, Line::Other) => "  ",
            (Layer::Middle, Line::First) => "    ",
            (Layer::Middle, Line::Other) => "    ",
            (Layer::Bottom, Line::First) => "    ",
            (Layer::Bottom, Line::Other) => "    ",
        }
    }

    fn space(&self) -> &'static str { "    " }
}
