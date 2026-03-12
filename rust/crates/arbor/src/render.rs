use alloc::collections::vec_deque::VecDeque;
use alloc::string::String;
use alloc::sync::Arc;
use core::fmt::{self, Display};

use crate::protocol::{IIndent, ITree, Layer, Line};
extern crate alloc;

pub struct Render<'a, I, T> {
    pub tree: &'a T,
    pub indent: &'a I,
    #[cfg(feature = "textwrap")]
    pub width: usize,
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
        #[cfg(not(feature = "textwrap"))]
        {
            let lines = content.lines();
            for (line_index, text) in lines.enumerate() {
                f.write_str(prefix)?;
                f.write_str(self.indent.get_indent(
                    layer,
                    if line_index == 0 {
                        Line::First
                    } else {
                        Line::Other
                    },
                ))?;
                f.write_str(text)?;
                writeln!(f)?;
            }
        }
        #[cfg(feature = "textwrap")]
        if self.width == 0 {
            let lines = content.lines();
            for (line_index, text) in lines.enumerate() {
                f.write_str(prefix)?;
                f.write_str(self.indent.get_indent(
                    layer,
                    if line_index == 0 {
                        Line::First
                    } else {
                        Line::Other
                    },
                ))?;
                f.write_str(text)?;
                writeln!(f)?;
            }
        } else {
            let wrap_option = textwrap::Options::new(self.width)
                .initial_indent(self.indent.get_indent(layer, Line::First))
                .subsequent_indent(self.indent.get_indent(layer, Line::Other));
            writeln!(f, "{}", textwrap::fill(content, &wrap_option))?;
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
