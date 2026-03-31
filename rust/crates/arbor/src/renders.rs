extern crate alloc;

use alloc::collections::vec_deque::VecDeque;
use alloc::rc::Rc;
use alloc::string::String;
use core::fmt::{self, Display};

use crate::protocol::{IComplexTree, IIndent, ITree, Layer, Line};

/// Renders a tree structure using a fixed indentation style.
///
/// `Render` implements [`Display`] and produces a textual tree representation
/// of any type implementing [`ITree`]. The indentation style is provided by
/// the [`IIndent`] implementation supplied in `indent`.
///
/// The renderer performs a breadth-first traversal using an internal queue
/// and constructs prefix spacing dynamically while walking the tree.
///
/// The indentation style is applied uniformly across the entire tree.
///
/// # examples
/// ```
/// use arbor::indents::UnicodeIndent;
/// use arbor::renders::Render;
/// use arbor::trees::Tree;
/// let tree = Tree::new("foo").with_leaves(["bar", "baz"]);
/// let render = Render {
///     tree: &tree,
///     indent: UnicodeIndent,
///     width: 0,
/// };
/// println!("{}", render);
/// ```
pub struct Render<'a, I, T> {
    /// Root tree node to render.
    pub tree: &'a T,

    /// Indentation style used for all nodes.
    pub indent: I,

    /// Optional wrapping width used when the `textwrap` feature is enabled.
    #[cfg(feature = "textwrap")]
    pub width: usize,
}

impl<'a, I, T> Display for Render<'a, I, T>
where
    I: IIndent,
    T: ITree<Leaf = T>,
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

        while let Some((leaf, layer, s, _)) = queue.pop_front() {
            render_content(
                f,
                leaf,
                layer,
                &s,
                &self.indent,
                #[cfg(feature = "textwrap")]
                self.width,
            )?;

            let mut iter = leaf.leaves().peekable();
            if iter.peek().is_some() {
                let mut leave_spaces = (*s).clone();
                leave_spaces.push_str(self.indent.get_indent(layer, Line::Other));
                enqueue(&mut queue, leaf, Rc::new(leave_spaces), &self.indent);
            }
        }
        Ok(())
    }
}

/// Renders a tree that supports per-node indentation overrides.
///
/// `ComplexRender` behaves similarly to [`Render`] but supports trees that
/// implement [`IComplexTree`]. Each node may optionally provide its own
/// indentation style, allowing sections of the tree to render using different
/// visual formats.
///
/// When a node does not provide an indentation style, the renderer inherits
/// the indentation configuration from its parent.
///
/// # examples
/// ```
/// use arbor::indents::UnicodeIndent;
/// use arbor::renders::ComplexRender;
/// use arbor::trees::ComplexTree;
/// let tree = ComplexTree::new_with_indent("foo", UnicodeIndent).with_leaves(["bar", "baz"]);
/// let render = ComplexRender {
///     tree: &tree,
///     width: 0,
/// };
/// println!("{}", render);
/// ```
pub struct ComplexRender<'a, T> {
    /// Root tree node to render.
    pub tree: &'a T,

    /// Optional wrapping width used when the `textwrap` feature is enabled.
    #[cfg(feature = "textwrap")]
    pub width: usize,
}

impl<'a, I, T> Display for ComplexRender<'a, T>
where
    I: IIndent + Clone,
    T: IComplexTree<Indent = I, Leaf = T>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut queue = VecDeque::new();

        let indent = self.tree.indent().clone().unwrap_or_default();

        render_content(
            f,
            self.tree,
            Layer::Root,
            "",
            &indent,
            #[cfg(feature = "textwrap")]
            self.width,
        )?;

        enqueue(&mut queue, self.tree, Rc::new(String::new()), &indent);

        while let Some((leaf, layer, s, indent)) = queue.pop_front() {
            render_content(
                f,
                leaf,
                layer,
                &s,
                indent,
                #[cfg(feature = "textwrap")]
                self.width,
            )?;
            let mut iter = leaf.leaves().peekable();
            if iter.peek().is_some() {
                let mut leave_spaces = (*s).clone();
                let leaf_indent = leaf.indent().as_ref().unwrap_or(indent);
                leave_spaces.push_str(indent.get_indent(layer, Line::Other));
                enqueue(&mut queue, leaf, Rc::new(leave_spaces), leaf_indent);
            }
        }

        Ok(())
    }
}

/// Pushes child nodes into the rendering queue.
///
/// This function determines the [`Layer`] classification for each child node
/// based on its position within the parent's children:
///
/// - `Top` — the first child
/// - `Middle` — any intermediate child
/// - `Bottom` — the final child
///
/// The queue stores the node reference, layer classification, accumulated
/// indentation prefix, and the indentation configuration used for rendering.
fn enqueue<'a, I, T>(
    queue: &mut VecDeque<(&'a T, Layer, Rc<String>, &'a I)>,
    tree: &'a T,
    spaces: Rc<String>,
    indent: &'a I,
) where
    I: IIndent,
    T: ITree<Leaf = T>,
{
    let leaves: alloc::vec::Vec<_> = tree.leaves().collect();
    let last = leaves.len().saturating_sub(1);
    for (i, leaf) in leaves.iter().rev().enumerate() {
        let layer = match i {
            0 => Layer::Bottom,
            i if i == last => Layer::Top,
            _ => Layer::Middle,
        };
        queue.push_front((leaf, layer, spaces.clone(), indent));
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
    T: ITree,
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
fn render_content<I>(
    f: &mut fmt::Formatter<'_>,
    node: &impl ITree,
    layer: Layer,
    prefix: &str,
    indent: &I,
    #[cfg(feature = "textwrap")] width: usize,
) -> fmt::Result
where
    I: IIndent,
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
