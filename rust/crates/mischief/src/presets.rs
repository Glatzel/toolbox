use arbor::protocol::{IIndent, Layer, Line};

#[derive(Debug, Clone, Default)]
pub struct FancyIndent;
impl IIndent for FancyIndent {
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
