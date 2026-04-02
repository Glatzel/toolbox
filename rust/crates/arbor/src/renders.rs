extern crate alloc;

use alloc::collections::vec_deque::VecDeque;
use alloc::rc::Rc;
use alloc::string::String;
use core::fmt::{self, Display};

use crate::protocol::{
    IIndent, ILazyTree, IOwnedTree, IStyledOwnedTree, ITreeContent, Layer, Line,
};

pub struct OwnedRender<'a, I, T> {
    /// Root tree node to render.
    pub tree: &'a T,

    /// Indentation style used for all nodes.
    pub indent: I,

    /// Optional wrapping width used when the `textwrap` feature is enabled.
    #[cfg(feature = "textwrap")]
    pub width: usize,
}

impl<'a, I, T> Display for OwnedRender<'a, I, T>
where
    I: IIndent,
    T: IOwnedTree<Leaf = T>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut queue = VecDeque::new();

        render_content(
            f,
            self.tree,
            Layer::Root,
            "",
            &self.indent,
            #[cfg(feature = "textwrap")]
            self.width,
        )?;

        enqueue(&mut queue, self.tree, Rc::new(String::new()), &self.indent);

        while let Some((leaf, layer, prefix, _)) = queue.pop_front() {
            render_content(
                f,
                leaf,
                layer,
                &prefix,
                &self.indent,
                #[cfg(feature = "textwrap")]
                self.width,
            )?;

            let mut iter = leaf.leaves().peekable();
            if iter.peek().is_some() {
                let mut leave_prefix = (*prefix).clone();
                leave_prefix.push_str(self.indent.get_indent(layer, Line::Other));
                enqueue(&mut queue, leaf, Rc::new(leave_prefix), &self.indent);
            }
        }
        Ok(())
    }
}

pub struct StyledOwnedRender<'a, T> {
    /// Root tree node to render.
    pub tree: &'a T,

    /// Optional wrapping width used when the `textwrap` feature is enabled.
    #[cfg(feature = "textwrap")]
    pub width: usize,
}

impl<'a, I, T> Display for StyledOwnedRender<'a, T>
where
    I: IIndent,
    T: IStyledOwnedTree<Indent = I, Leaf = T>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut queue = VecDeque::new();

        let default_indent = I::default();
        let indent = self.tree.indent().as_ref().unwrap_or(&default_indent);

        render_content(
            f,
            self.tree,
            Layer::Root,
            "",
            indent,
            #[cfg(feature = "textwrap")]
            self.width,
        )?;

        enqueue(&mut queue, self.tree, Rc::new(String::new()), indent);

        while let Some((leaf, layer, prefix, indent)) = queue.pop_front() {
            render_content(
                f,
                leaf,
                layer,
                &prefix,
                indent,
                #[cfg(feature = "textwrap")]
                self.width,
            )?;
            let mut iter = leaf.leaves().peekable();
            if iter.peek().is_some() {
                let mut leave_spaces = (*prefix).clone();
                let leaf_indent = leaf.indent().as_ref().unwrap_or(indent);
                leave_spaces.push_str(indent.get_indent(layer, Line::Other));
                enqueue(&mut queue, leaf, Rc::new(leave_spaces), leaf_indent);
            }
        }

        Ok(())
    }
}
pub struct LazyRender<I, T> {
    pub tree: T,
    pub indent: I,

    #[cfg(feature = "textwrap")]
    pub width: usize,
}
impl<I, T> Display for LazyRender<I, T>
where
    I: IIndent + Clone,
    T: ILazyTree<Leaf = T>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut queue = VecDeque::new();

        render_content(
            f,
            &self.tree,
            Layer::Root,
            "",
            &self.indent,
            #[cfg(feature = "textwrap")]
            self.width,
        )?;

        enqueue_lazy(
            &mut queue,
            self.tree.leaves(),
            Rc::new(String::new()),
            &self.indent,
        );

        while let Some((leaf, layer, prefix, indent)) = queue.pop_front() {
            render_content(
                f,
                &leaf,
                layer,
                &prefix,
                indent,
                #[cfg(feature = "textwrap")]
                self.width,
            )?;

            let mut iter = leaf.leaves().peekable();
            if iter.peek().is_some() {
                let mut leave_prefix = (*prefix).clone();
                leave_prefix.push_str(self.indent.get_indent(layer, Line::Other));
                enqueue_lazy(&mut queue, leaf.leaves(), Rc::new(leave_prefix), &self.indent);
            }
        }

        Ok(())
    }
}
fn enqueue<'a, I, T>(
    queue: &mut VecDeque<(&'a T, Layer, Rc<String>, &'a I)>,
    tree: &'a T,
    prefix: Rc<String>,
    indent: &'a I,
) where
    I: IIndent,
    T: IOwnedTree<Leaf = T>,
{
    let leaves: alloc::vec::Vec<_> = tree.leaves().collect();
    let last = leaves.len().saturating_sub(1);
    for (i, leaf) in leaves.iter().rev().enumerate() {
        let layer = match i {
            0 => Layer::Bottom,
            i if i == last => Layer::Top,
            _ => Layer::Middle,
        };
        queue.push_front((leaf, layer, prefix.clone(), indent));
    }
}
fn enqueue_lazy<'a, I, T>(
    queue: &mut VecDeque<(T, Layer, Rc<String>, &'a I)>,
    leaves: impl DoubleEndedIterator<Item = T>,
    prefix: Rc<String>,
    indent: &'a I,
) where
    I: IIndent,
    T: ILazyTree,
{
    let leaves: alloc::vec::Vec<T> = leaves.collect();
    let last = leaves.len().saturating_sub(1);

    for (i, leaf) in leaves.into_iter().rev().enumerate() {
        let layer = match i {
            0 => Layer::Bottom,
            i if i == last => Layer::Top,
            _ => Layer::Middle,
        };

        queue.push_front((leaf, layer, prefix.clone(), indent));
    }
}
/// Renders the textual content of a node without line wrapping.
///
/// The node content may contain multiple lines. Each line is rendered with
/// the appropriate prefix and indentation marker depending on whether it
/// is the first line or a continuation line.
///
/// The `Layer` determines which branch marker is emitted by the indentation
/// style implementation.
fn render_content_no_wrap<I, T>(
    f: &mut fmt::Formatter<'_>,
    node: &T,
    layer: Layer,
    prefix: &str,
    indent: &I,
) -> fmt::Result
where
    I: IIndent,
    T: ITreeContent,
{
    for (line_index, text) in node.content().as_ref().lines().enumerate() {
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

/// Renders the textual content of a node with optional line wrapping.
///
/// When the `textwrap` feature is disabled, this function delegates to
/// [`render_content_no_wrap`].
///
/// When enabled, the content is wrapped to the configured width while
/// preserving the indentation prefixes used for the first and subsequent
/// lines.
fn render_content<I, T>(
    f: &mut fmt::Formatter<'_>,
    node: &T,
    layer: Layer,
    prefix: &str,
    indent: &I,
    #[cfg(feature = "textwrap")] width: usize,
) -> fmt::Result
where
    I: IIndent,
    T: ITreeContent,
{
    #[cfg(not(feature = "textwrap"))]
    render_content_no_wrap(f, node, layer, prefix, indent)?;

    #[cfg(feature = "textwrap")]
    if width == 0 {
        render_content_no_wrap(f, node, layer, prefix, indent)?;
    } else {
        let initial_indent = alloc::format!("{}{}", prefix, indent.get_indent(layer, Line::First));

        let subsequent_indent =
            alloc::format!("{}{}", prefix, indent.get_indent(layer, Line::Other));

        let wrap_option = textwrap::Options::new(width)
            .initial_indent(&initial_indent)
            .subsequent_indent(&subsequent_indent);

        writeln!(
            f,
            "{}",
            textwrap::fill(node.content().as_ref(), &wrap_option)
        )?;
    }

    Ok(())
}
