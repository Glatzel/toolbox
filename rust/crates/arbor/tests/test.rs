use arbor::Render;
use arbor::presets::indent::UnicodeIndent;
use arbor::presets::tree::Tree;
use arbor::protocol::WrapMode;

#[test]
fn render_tree_root() {
    let tree = Tree::new("foo");
    let render = Render {
        tree: &tree,
        indent: &UnicodeIndent,
        wrap_mode: WrapMode::SingleLine,
    };
    println!("{}", render);
    insta::assert_snapshot!(format!("{}", render));
}
#[test]
fn render_tree_with_leaves() {
    let tree = Tree::new("foo")
        .with_leaves([Tree::new("bar").with_leaves([Tree::new("foobar").with_leaves(["baz"])])]);
    let render = Render {
        tree: &tree,
        indent: &UnicodeIndent,
        wrap_mode: WrapMode::SingleLine,
    };
    println!("{}", render);
    insta::assert_snapshot!(format!("{}", render));
}
#[test]
fn render_tree_with_multiple_leaves() {
    let tree = Tree::new("foo").with_leaves(["bar", "baz"]);
    let render = Render {
        tree: &tree,
        indent: &UnicodeIndent,
        wrap_mode: WrapMode::SingleLine,
    };
    println!("{}", render);
    insta::assert_snapshot!(format!("{}", render));
}
#[test]
fn render_tree_with_multiple_lines() {
    let tree = Tree::new("foo\nfoo").with_leaves(["bar\nbar", "baz"]);
    let render = Render {
        tree: &tree,
        indent: &UnicodeIndent,
        wrap_mode: WrapMode::SingleLine,
    };
    println!("{}", render);
    insta::assert_snapshot!(format!("{}", render));
}
#[test]
fn render_tree_with_single_lines() {
    let tree = Tree::new("foo\nfoo").with_leaves(["bar\nbar\nbar", "baz"]);
    let render = Render {
        tree: &tree,
        indent: &UnicodeIndent,
        wrap_mode: WrapMode::MultiLine,
    };
    println!("{}", render);
    insta::assert_snapshot!(format!("{}", render));
}
#[test]
fn render_tree_with_complex() {
    let tree = Tree::new("node 1").with_leaves([
        Tree::new("node 1.1"),
        Tree::new("node 1.2"),
        Tree::new("node 1.3").with_leaves([
            Tree::new("node 1.3.1").with_leaves(["node 1.3.1.1"]),
            Tree::new("node 1.3.2"),
            Tree::new("node 1.3.3").with_leaves(["node\n1.3.3.1", "node 1.3.3.2"]),
        ]),
        Tree::new("node 1.4").with_leaves([
            Tree::new("node 1.4.1"),
            Tree::new("node 1.4.2"),
            Tree::new("node 1.4.3").with_leaves(["node 1.4.3.1", "node 1.4.3.2"]),
        ]),
    ]);
    let render = Render {
        tree: &tree,
        indent: &UnicodeIndent,
        wrap_mode: WrapMode::MultiLine,
    };
    println!("{}", render);
    insta::assert_snapshot!(format!("{}", render));
}
