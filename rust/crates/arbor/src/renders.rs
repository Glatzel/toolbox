extern crate alloc;

use alloc::collections::vec_deque::VecDeque;
use alloc::rc::Rc;
use alloc::string::String;
use core::fmt::{self, Display};

use crate::protocol::{
    IIndent, ILazyTree, IOwnedTree, IStyledOwnedTree, ITreeContent, Layer, Line,
};

/// Renderer for trees implementing [`IOwnedTree`].
///
/// This renderer traverses a tree whose nodes are referenced (`&T`) and whose
/// children are accessed via iterators yielding references. Rendering is done
/// in a breadth-first manner using an internal queue.
///
/// The indentation style is fixed for the entire tree and provided through
/// the `indent` field.
///
/// If the `textwrap` feature is enabled, lines can optionally be wrapped
/// to the specified width.
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

        enqueue(
            &mut queue,
            self.tree.leaves(),
            Rc::new(String::new()),
            &self.indent,
        );

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
                enqueue(
                    &mut queue,
                    leaf.leaves(),
                    Rc::new(leave_prefix),
                    &self.indent,
                );
            }
        }
        Ok(())
    }
}

/// Renderer for styled trees implementing [`IStyledOwnedTree`].
///
/// Unlike [`OwnedRender`], this renderer allows each node to override the
/// indentation style used for its subtree. If a node does not provide a custom
/// indentation style, the parent indentation or the default indentation
/// implementation is used.
///
/// This allows heterogeneous indentation styles within the same tree.
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

        enqueue(
            &mut queue,
            self.tree.leaves(),
            Rc::new(String::new()),
            indent,
        );

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
                enqueue(
                    &mut queue,
                    leaf.leaves(),
                    Rc::new(leave_spaces),
                    leaf_indent,
                );
            }
        }

        Ok(())
    }
}

/// Renderer for trees implementing [`ILazyTree`].
///
/// In this model, tree nodes own their children and the iterator returned by
/// `leaves()` yields values rather than references. This enables lazy or
/// streaming tree construction where nodes may be generated dynamically.
///
/// Because nodes are owned, this renderer stores them directly in the queue
/// during traversal.
pub struct LazyRender<I, T> {
    /// Root tree node to render.
    pub tree: T,

    /// Indentation style used for rendering.
    pub indent: I,

    /// Optional wrapping width used when the `textwrap` feature is enabled.
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

        enqueue(
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
                enqueue(
                    &mut queue,
                    leaf.leaves(),
                    Rc::new(leave_prefix),
                    &self.indent,
                );
            }
        }

        Ok(())
    }
}

/// Pushes a set of child nodes into the rendering queue.
///
/// The iterator of leaves is collected and then pushed in reverse order so
/// that the first child is processed first when popping from the front of
/// the queue.
///
/// Each child is annotated with a [`Layer`] describing whether it is the
/// top, middle, or bottom branch within the current sibling group.
fn enqueue<I, T, It>(
    queue: &mut VecDeque<(T, Layer, Rc<String>, I)>,
    leaves: It,
    prefix: Rc<String>,
    indent: I,
) where
    It: DoubleEndedIterator<Item = T>,
    I: Clone,
{
    let leaves: alloc::vec::Vec<T> = leaves.collect();
    let last = leaves.len().saturating_sub(1);

    for (i, leaf) in leaves.into_iter().rev().enumerate() {
        let layer = match i {
            0 => Layer::Bottom,
            i if i == last => Layer::Top,
            _ => Layer::Middle,
        };

        queue.push_front((leaf, layer, prefix.clone(), indent.clone()));
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
