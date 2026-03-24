//! `arbor` provides a lightweight and flexible framework for representing and
//! rendering hierarchical tree structures.
//!
//! The crate is designed for building structured textual output such as
//! diagnostics, logs, reports, or any data that benefits from a hierarchical
//! visualization. It separates the concerns of **tree structure**,
//! **indentation style**, and **rendering**, allowing each component to evolve
//! independently.
//!
//! # Design Goals
//!
//! - **Minimal core abstraction** — the tree model is intentionally simple and
//!   based on small traits.
//! - **Pluggable indentation styles** — visual layout is controlled through
//!   interchangeable indentation strategies.
//! - **Renderer independence** — trees are data structures and are not coupled
//!   to any specific rendering implementation.
//! - **Composable formatting** — different parts of a tree may optionally
//!   override indentation behavior.
//! - **Allocation-conscious** — rendering is designed to avoid unnecessary
//!   allocations while remaining ergonomic.
//!
//! # Core Concepts
//!
//! The crate is built around three core abstractions.
//!
//! ## Tree Structure
//!
//! [`protocol::ITree`] defines a minimal interface for hierarchical data
//! structures. A node exposes textual content and a list of child nodes. The
//! interface is intentionally small so that existing data structures can easily
//! implement it.
//!
//! [`protocol::IComplexTree`] extends this model by allowing nodes to override
//! the indentation style used for rendering. This enables sections of a tree to
//! use different visual styles without affecting the entire structure.
//!
//! ## Indentation Styles
//!
//! [`protocol::IIndent`] defines how branch prefixes and continuation markers
//! are generated during rendering. Implementations of this trait determine how
//! tree relationships are visually represented.
//!
//! Several built-in indentation strategies are provided, ranging from minimal
//! whitespace indentation to ASCII or Unicode tree drawing styles.
//!
//! The indentation system is independent of the tree representation and may be
//! reused across different renderers.
//!
//! ## Rendering
//!
//! Rendering is performed by the [`renders::Render`] and
//! [`renders::ComplexRender`] types, which implement [`core::fmt::Display`].
//! These renderers walk the tree structure and emit formatted output using the
//! configured indentation strategy.
//!
//! `Render` applies a single indentation style to the entire tree, while
//! `ComplexRender` supports indentation overrides defined by nodes implementing
//! [`protocol::IComplexTree`].
//!
//! The rendering pipeline is designed to support multi-line node content and
//! optional line wrapping.
//!
//! # Indentation Model
//!
//! Rendering uses two contextual dimensions to determine indentation output:
//!
//! - [`protocol::Layer`] — describes the vertical relationship of a node within
//!   its siblings (top, middle, bottom, or root).
//! - [`protocol::Line`] — distinguishes between the first line of a node and
//!   continuation lines when the content spans multiple lines.
//!
//! These two signals allow indentation implementations to construct accurate
//! tree visuals while preserving alignment for wrapped or multi-line content.
//!
//! # Extensibility
//!
//! `arbor` is designed to be extended in several ways:
//!
//! - Custom tree data structures can implement [`protocol::ITree`].
//! - Custom indentation systems can implement [`protocol::IIndent`].
//! - Alternative renderers can operate on the same trait abstractions.
//!
//! The crate intentionally avoids enforcing a specific tree type, enabling it
//! to integrate naturally into existing applications.
//!
//! # Feature Flags
//!
//! Some functionality is gated behind optional features.
//!
//! - `textwrap` — enables automatic line wrapping during rendering.
//!
//! When disabled, node content is rendered exactly as provided.
//!
//! # Intended Use Cases
//!
//! `arbor` is suitable for applications that require structured textual output,
//! including:
//!
//! - diagnostic and error reporting systems
//! - hierarchical logging
//! - command-line tools
//! - structured debugging output
//! - tree visualization utilities
//!
//! The crate aims to provide a clean and predictable foundation for building
//! these systems without imposing unnecessary complexity.
//!
//!  # examples
//！ ```
//！ use arbor::indents::UnicodeIndent;
//！ use arbor::renders::Render;
//！ use arbor::trees::Tree;
//！ let tree = Tree::new("foo").with_leaves(["bar", "baz"]);
//！ let render = Render {
//！     tree: &tree,
//！     indent: UnicodeIndent,
//！     width: 0,
//！ };
//！ println!("{}", render);
//！ ```
#![no_std]
pub mod indents;
pub mod lazy_renders;
pub mod protocol;
pub mod renders;
pub mod trees;
