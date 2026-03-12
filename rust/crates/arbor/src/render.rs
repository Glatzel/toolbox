use alloc::collections::vec_deque::VecDeque;
use alloc::string::String;
use alloc::sync::Arc;
use core::fmt::{self, Display};

use crate::protocol::{IIndent, ITree, Layer, Line};
extern crate alloc;

pub struct Render<'a, I, T> {
    pub tree: &'a T,
    pub indent: &'a I,
}
impl<'a, I, T> Display for Render<'a, I, T>
where
    T: ITree<Leave = T>,
    I: IIndent,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.indent.get_indent(Layer::Root, Line::First))?;
        writeln!(f, "{}", self.tree.content())?;
        let mut queue: VecDeque<(&T, Layer, Arc<String>)> = VecDeque::new();
        enqueue(&mut queue, self.tree, Arc::new(String::new()));
        while let Some((leaf, layer, s)) = queue.pop_front() {
            f.write_str(&s)?;
            f.write_str(self.indent.get_indent(layer, crate::protocol::Line::First))?;
            f.write_str(leaf.content())?;
            writeln!(f)?;
            if !leaf.leaves().is_empty() {
                let mut leave_spaces = (*s).clone();
                leave_spaces.push_str(self.indent.get_indent(layer, Line::Other));
                enqueue(&mut queue, leaf, Arc::new(leave_spaces));
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
