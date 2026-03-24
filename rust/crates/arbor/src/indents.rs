use crate::protocol::{IIndent, Layer, Line};

/// A fully configurable indentation style.
///
/// `UniversalIndent` exposes all `(Layer, Line)` combinations as
/// user-configurable fields, allowing complete control over how
/// tree prefixes are rendered.
///
/// This is useful when:
///
/// - creating custom tree renderers
/// - implementing non-standard indentation formats
/// - matching the output style of external tools
///
/// Each field corresponds to a `(Layer, Line)` pair returned by
/// [`IIndent::get_indent`].
#[derive(Debug, Clone)]
pub struct UniversalIndent {
    /// Prefix used for the first line of the root node.
    pub root_first: &'static str,
    /// Prefix used for continuation lines of the root node.
    pub root_other: &'static str,
    /// Prefix used for the first line of a top layer node.
    pub top_first: &'static str,
    /// Prefix used for continuation lines of a top layer node.
    pub top_other: &'static str,
    /// Prefix used for the first line of a middle layer node.
    pub mid_first: &'static str,
    /// Prefix used for continuation lines of a middle layer node.
    pub mid_other: &'static str,
    /// Prefix used for the first line of a bottom layer node.
    pub bottom_first: &'static str,
    /// Prefix used for continuation lines of a bottom layer node.
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
    /// Creates a default ASCII-style indentation configuration.
    ///
    /// The default style produces output similar to:
    ///
    /// ```text
    /// root
    /// |-- child
    /// |   |-- grandchild
    /// `-- child
    /// ```
    fn default() -> Self {
        Self {
            root_first: "",
            root_other: "",
            top_first: "|-- ",
            top_other: "|   ",
            mid_first: "|-- ",
            mid_other: "|   ",
            bottom_first: "`-- ",
            bottom_other: "    ",
        }
    }
}

/// A Unicode tree indentation style.
///
/// This renderer uses box-drawing characters to produce a
/// visually clearer tree structure.
///
/// Example output:
///
/// ```text
/// root
/// ├── child
/// │   ├── grandchild
/// ╰── child
/// ```
#[derive(Debug, Clone, Default)]
pub struct UnicodeIndent;

impl IIndent for UnicodeIndent {
    fn get_indent(&self, layer: Layer, line: Line) -> &'static str {
        match (layer, line) {
            (Layer::Root, _) => "",
            (Layer::Middle | Layer::Top, Line::First) => "├── ",
            (Layer::Bottom, Line::First) => "╰── ",
            (Layer::Bottom, Line::Other) => "    ",
            (_, Line::Other) => "│   ",
        }
    }
}

/// A minimal indentation style using only spaces.
///
/// This style does not render tree connectors and simply
/// indents nodes according to their depth.
///
/// Example output:
///
/// ```text
/// root
///     child
///     child
/// ```
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

/// A classic ASCII tree indentation style.
///
/// This style is compatible with terminals that do not support
/// Unicode box-drawing characters.
///
/// Example output:
///
/// ```text
/// root
/// |-- child
/// |   |-- grandchild
/// `-- child
/// ```
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
/// A debugging indentation style.
///
/// Instead of drawing tree connectors, this style prints
/// explicit `(Layer, Line)` markers. This is useful when
/// developing or debugging render logic.
///
/// Example output:
///
/// ```text
/// [RF]root
/// [TF]child
/// [MF]child
/// [BF]child
/// ```
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
