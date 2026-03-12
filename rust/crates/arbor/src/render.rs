use alloc::collections::vec_deque::VecDeque;
use alloc::string::String;
use alloc::sync::Arc;
use core::fmt::{self, Display};

use crate::protocol::{IIndent, ITree, Layer, Line, WrapMode};
extern crate alloc;

pub struct Render<'a, I, T> {
    pub tree: &'a T,
    pub indent: &'a I,
    pub wrap_mode: WrapMode,
}
impl<'a, I, T> Display for Render<'a, I, T>
where
    T: ITree<Leave = T>,
    I: IIndent,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.render_content(f, self.tree.content(), Layer::Root, "")?;
        let mut queue: VecDeque<(&T, Layer, Arc<String>)> = VecDeque::new();
        enqueue(&mut queue, self.tree, Arc::new(String::new()));
        while let Some((leaf, layer, s)) = queue.pop_front() {
            self.render_content(f, leaf.content(), layer, &s)?;
            if !leaf.leaves().is_empty() {
                let mut leave_spaces = (*s).clone();
                leave_spaces.push_str(self.indent.get_indent(layer, Line::Other));
                enqueue(&mut queue, leaf, Arc::new(leave_spaces));
            }
        }
        Ok(())
    }
}
impl<'a, I, T> Render<'a, I, T>
where
    T: ITree<Leave = T>,
    I: IIndent,
{
    fn render_content(
        &self,
        f: &mut fmt::Formatter<'_>,
        content: &str,
        layer: Layer,
        prefix: &str,
    ) -> fmt::Result {
        let lines = content.lines();
        let line_count = lines.clone().count();

        for (line_index, text) in lines.enumerate() {
            let last = line_index == line_count - 1;

            match (self.wrap_mode, line_index) {
                (WrapMode::SingleLine, 0) => {
                    f.write_str(prefix)?;
                    f.write_str(self.indent.get_indent(layer, Line::First))?;
                    f.write_str(text)?;
                    if last {
                        writeln!(f)?;
                    }
                }
                (WrapMode::SingleLine, _) => {
                    f.write_str(prefix)?;
                    f.write_str(text)?;
                    if last {
                        writeln!(f)?;
                    }
                }
                (WrapMode::MultiLine, 0) => {
                    f.write_str(prefix)?;
                    f.write_str(self.indent.get_indent(layer, Line::First))?;
                    f.write_str(text)?;
                    writeln!(f)?;
                }
                (WrapMode::MultiLine, _) => {
                    f.write_str(prefix)?;
                    f.write_str(self.indent.get_indent(layer, Line::Other))?;
                    f.write_str(text)?;
                    writeln!(f)?;
                }
                #[cfg(feature = "textwrap")]
                (WrapMode::FixedWidth(width), 0) => {
                    let wrap_option = textwrap::Options::new(width)
                        .initial_indent(self.indent.get_indent(layer, Line::First))
                        .subsequent_indent(self.indent.get_indent(layer, Line::Other));
                    writeln!(f, "{}", textwrap::fill(text, &wrap_option))?;
                }
                #[cfg(feature = "textwrap")]
                (WrapMode::FixedWidth(width), _) => {
                    let wrap_option = textwrap::Options::new(width)
                        .initial_indent(self.indent.get_indent(layer, Line::Other))
                        .subsequent_indent(self.indent.get_indent(layer, Line::Other));
                    writeln!(f, "{}", textwrap::fill(text, &wrap_option))?;
                }
            }
        }

        Ok(())
    }
}
fn enqueue<'a, T>(
    queue: &mut VecDeque<(&'a T, Layer, Arc<String>)>,
    tree: &'a T,
    spaces: Arc<String>,
) where
    T: ITree<Leave = T>,
{
    let children_count_index = tree.leaves().len().saturating_sub(1);
    for (i, leaf) in tree.leaves().iter().rev().enumerate() {
        let layer = match i {
            0 => Layer::Bottom,
            i => {
                if i == children_count_index {
                    Layer::Top
                } else {
                    Layer::Middle
                }
            }
        };
        queue.push_front((leaf, layer, spaces.clone()));
    }
}
