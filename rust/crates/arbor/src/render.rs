use alloc::collections::vec_deque::VecDeque;
use alloc::rc::Rc;
use alloc::string::String;
use core::fmt::{self, Display};

use crate::protocol::{IComplexTree, IIndent, ITree, Layer, Line};
extern crate alloc;
pub struct Render<'a, I, T> {
    pub tree: &'a T,
    pub indent: I,
    #[cfg(feature = "textwrap")]
    pub width: usize,
}
impl<'a, I, T> Display for Render<'a, I, T>
where
    I: IIndent,
    T: ITree<Leave = T>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut queue: VecDeque<(&T, Layer, Rc<String>)> = VecDeque::new();
        self.render_content(f, self.tree, Layer::Root, "")?;
        self.enqueue(&mut queue, self.tree, Rc::new(String::new()));
        while let Some((leaf, layer, s)) = queue.pop_front() {
            self.render_content(f, leaf, layer, &s)?;
            if !leaf.leaves().is_empty() {
                let mut leave_spaces = (*s).clone();
                leave_spaces.push_str(self.indent.get_indent(layer, Line::Other));
                self.enqueue(&mut queue, leaf, Rc::new(leave_spaces));
            }
        }
        Ok(())
    }
}
impl<'a, I, T> Render<'a, I, T>
where
    I: IIndent,
    T: ITree<Leave = T>,
{
    fn render_content(
        &self,
        f: &mut fmt::Formatter<'_>,
        node: &impl ITree,
        layer: Layer,
        prefix: &str,
    ) -> fmt::Result {
        #[cfg(not(feature = "textwrap"))]
        self.render_content_no_wrap(f, content, layer, prefix)?;
        #[cfg(feature = "textwrap")]
        if self.width == 0 {
            self.render_content_no_wrap(f, node, layer, prefix)?;
        } else {
            let initial_indent =
                alloc::format!("{}{}", prefix, self.indent.get_indent(layer, Line::First));
            let subsequent_indent =
                alloc::format!("{}{}", prefix, self.indent.get_indent(layer, Line::Other));
            let wrap_option = textwrap::Options::new(self.width)
                .initial_indent(&initial_indent)
                .subsequent_indent(&subsequent_indent);
            writeln!(f, "{}", textwrap::fill(node.content(), &wrap_option))?;
        }

        Ok(())
    }
    fn render_content_no_wrap(
        &self,
        f: &mut fmt::Formatter<'_>,
        node: &impl ITree,
        layer: Layer,
        prefix: &str,
    ) -> fmt::Result {
        let lines = node.content().lines();
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
        Ok(())
    }
    fn enqueue(
        &self,
        queue: &mut VecDeque<(&'a T, Layer, Rc<String>)>,
        tree: &'a T,
        spaces: Rc<String>,
    ) where
        I: IIndent,
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
}

pub struct ComplexRender<'a, T> {
    pub tree: &'a T,
    #[cfg(feature = "textwrap")]
    pub width: usize,
}
impl<'a, I, T> Display for ComplexRender<'a, T>
where
    I: IIndent,
    T: IComplexTree<Indent = I, Leave = T>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut queue: VecDeque<(&T, Layer, Rc<String>, I)> = VecDeque::new();
        {
            let indent = self.tree.indent().clone().unwrap_or_default();
            self.render_content(f, self.tree, Layer::Root, "", &indent)?;
            self.enqueue(&mut queue, self.tree, Rc::new(String::new()), indent);
        }
        while let Some((leaf, layer, s, indent)) = queue.pop_front() {
            self.render_content(f, leaf, layer, &s, &indent)?;
            if !leaf.leaves().is_empty() {
                let mut leave_spaces = (*s).clone();
                let leaf_indent = leaf.indent().clone().unwrap_or(indent.clone());
                leave_spaces.push_str(indent.get_indent(layer, Line::Other));
                self.enqueue(&mut queue, leaf, Rc::new(leave_spaces), leaf_indent);
            }
        }
        Ok(())
    }
}
impl<'a, I, T> ComplexRender<'a, T>
where
    I: IIndent,
    T: IComplexTree<Indent = I, Leave = T>,
{
    fn render_content(
        &self,
        f: &mut fmt::Formatter<'_>,
        node: &impl IComplexTree,
        layer: Layer,
        prefix: &str,
        indent: &I,
    ) -> fmt::Result {
        #[cfg(not(feature = "textwrap"))]
        self.render_content_no_wrap(f, content, layer, prefix)?;
        #[cfg(feature = "textwrap")]
        if self.width == 0 {
            self.render_content_no_wrap(f, node, layer, prefix, indent)?;
        } else {
            let initial_indent =
                alloc::format!("{}{}", prefix, indent.get_indent(layer, Line::First));
            let subsequent_indent =
                alloc::format!("{}{}", prefix, indent.get_indent(layer, Line::Other));
            let wrap_option = textwrap::Options::new(self.width)
                .initial_indent(&initial_indent)
                .subsequent_indent(&subsequent_indent);
            writeln!(f, "{}", textwrap::fill(node.content(), &wrap_option))?;
        }

        Ok(())
    }
    fn render_content_no_wrap(
        &self,
        f: &mut fmt::Formatter<'_>,
        node: &impl IComplexTree,
        layer: Layer,
        prefix: &str,
        indent: &I,
    ) -> fmt::Result {
        let lines = node.content().lines();
        for (line_index, text) in lines.enumerate() {
            f.write_str(prefix)?;
            f.write_str(indent.get_indent(
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
        Ok(())
    }
    fn enqueue(
        &self,
        queue: &mut VecDeque<(&'a T, Layer, Rc<String>, I)>,
        tree: &'a T,
        spaces: Rc<String>,
        indent: I,
    ) where
        I: IIndent,
        T: IComplexTree<Indent = I, Leave = T>,
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
            queue.push_front((leaf, layer, spaces.clone(), indent.clone()));
        }
    }
}
