use arbor::indents::{AsciiIndent, DebugIndent, SpaceIndent, UnicodeIndent, UniversalIndent};
use arbor::protocol::IIndent;
use arbor::renders::{OwnedRender, StyledOwnedRender};
use arbor::trees::{OwnedTree, StyledOwnedTree};
use rstest::rstest;

#[test]
fn render_tree_root() {
    let tree = OwnedTree::new("foo");
    let render = OwnedRender {
        tree: &tree,
        indent: UnicodeIndent,
        width: 0,
    };
    println!("{}", render);
    insta::assert_snapshot!(format!("{}", render));
}
#[test]
fn render_tree_with_leaves() {
    let tree = OwnedTree::new("foo").with_leaves([
        OwnedTree::new("bar").with_leaves([OwnedTree::new("foobar").with_leaves(["baz"])])
    ]);
    let render = OwnedRender {
        tree: &tree,
        indent: UnicodeIndent,
        width: 0,
    };
    println!("{}", render);
    insta::assert_snapshot!(format!("{}", render));
}
#[test]
fn render_tree_with_multiple_leaves() {
    let tree = OwnedTree::new("foo").with_leaves(["bar", "baz"]);
    let render = OwnedRender {
        tree: &tree,
        indent: UnicodeIndent,
        width: 0,
    };
    println!("{}", render);
    insta::assert_snapshot!(format!("{}", render));
}
#[test]
fn render_tree_with_fixed_line() {
    let tree = OwnedTree::new("textwrap1: an efficient and powerful library for wrapping text.")
        .with_leaves([
            "textwrap2: an efficient and powerful library for wrapping text.",
            "textwrap3: an efficient and powerful library for wrapping text.",
        ]);
    let render = OwnedRender {
        tree: &tree,
        indent: UnicodeIndent,
        width: 28,
    };
    println!("{}", render);
    insta::assert_snapshot!(format!("{}", render));
}

#[rstest]
#[case("normal", 0)]
#[case("fixed_width", 12)]
fn render_tree_with_multiple_lines(#[case] name: &str, #[case] mode: usize) {
    let tree = OwnedTree::new("foo\nfoo").with_leaves(["bar\nbar\nbar\nbar bar bar bar", "baz"]);
    let render = OwnedRender {
        tree: &tree,
        indent: UnicodeIndent,
        width: mode,
    };
    println!("{}", render);
    insta::assert_snapshot!(
        format!("render_tree_with_multiple_lines_{name}",),
        format!("{}", render)
    );
}
#[test]
fn render_tree_with_different_indent() {
    let indent1 = UniversalIndent::default();
    let indent2 = UniversalIndent {
        root_first: "".to_string(),
        root_other: "".to_string(),
        top_first: "~~> ".to_string(),
        top_other: "l   ".to_string(),
        mid_first: "~~> ".to_string(),
        mid_other: "l   ".to_string(),
        bottom_first: "==> ".to_string(),
        bottom_other: "    ".to_string(),
    };
    let tree = StyledOwnedTree::new_with_indent("node 1\nroot", indent1).with_leaves([
        StyledOwnedTree::new("node 1.1"),
        StyledOwnedTree::new("node 1.2"),
        StyledOwnedTree::new_with_indent("node 1.3", indent2).with_leaves([
            StyledOwnedTree::new("node 1.3.1").with_leaves(["node 1.3.1.1"]),
            StyledOwnedTree::new("node 1.3.2"),
            StyledOwnedTree::new("node 1.3.3").with_leaves(["node\n1.3.3.1", "node 1.3.3.2"]),
        ]),
        StyledOwnedTree::new("node 1.4").with_leaves([
            StyledOwnedTree::new("node 1.4.1"),
            StyledOwnedTree::new("node 1.4.2"),
            StyledOwnedTree::new("node 1.4.3").with_leaves(["node 1.4.3.1", "node 1.4.3.2"]),
        ]),
    ]);
    let render = StyledOwnedRender {
        tree: &tree,
        width: 0,
    };
    println!("{}", render);
    insta::assert_snapshot!(format!("{}", render));
}
#[rstest]
#[case("unicode0", UnicodeIndent, 0)]
#[case("ascii0", AsciiIndent, 0)]
#[case("space0", SpaceIndent, 0)]
#[case("debug0", DebugIndent, 0)]
#[case("unicode1", UnicodeIndent, 20)]
#[case("ascii1", AsciiIndent, 20)]
#[case("space1", SpaceIndent, 20)]
#[case("debug1", DebugIndent, 20)]
fn render_tree_with_complex(
    #[case] name: &str,
    #[case] indent: impl IIndent,
    #[case] width: usize,
) {
    let tree = OwnedTree::new("node 1\nroot").with_leaves([
        OwnedTree::new("node 1.1"),
        OwnedTree::new("node 1.2"),
        OwnedTree::new("node 1.3").with_leaves([
            OwnedTree::new("node 1.3.1").with_leaves(["node 1.3.1.1"]),
            OwnedTree::new("node 1.3.2"),
            OwnedTree::new("node 1.3.3").with_leaves(["node\n1.3.3.1", "node 1.3.3.2"]),
        ]),
        OwnedTree::new("node 1.4").with_leaves([
            OwnedTree::new("node 1.4.1"),
            OwnedTree::new("node 1.4.2"),
            OwnedTree::new("node 1.4.3").with_leaves(["node 1.4.3.1", "node 1.4.3.2"]),
        ]),
    ]);
    let render = OwnedRender {
        tree: &tree,
        indent: indent,
        width: width,
    };
    println!("{}", render);
    insta::assert_snapshot!(
        format!("render_tree_with_complex_{name}",),
        format!("{}", render)
    );
}
