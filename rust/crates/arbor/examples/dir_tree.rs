use arbor::indents::UnicodeIndent;
use arbor::renders::OwnedRender;
use arbor::trees::OwnedTree;

fn dir_tree(dir: &std::path::Path) -> OwnedTree<String> {
    let name = dir
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| dir.to_string_lossy().into_owned());

    let mut node = OwnedTree::new(name);

    if let Ok(entries) = std::fs::read_dir(dir) {
        let mut paths: Vec<_> = entries.filter_map(|e| e.ok()).map(|e| e.path()).collect();
        paths.sort();

        for path in paths {
            if path.is_dir() {
                node.push(dir_tree(&path));
            } else {
                node.push(OwnedTree::new(
                    path.file_name().unwrap().to_string_lossy().into_owned(),
                ));
            }
        }
    }

    node
}
#[test]
fn main() {
    let tree = dir_tree(std::path::Path::new(env!("CARGO_MANIFEST_DIR")));
    let render = OwnedRender {
        tree: &tree,
        indent: UnicodeIndent,
        width: 0,
    };
    println!("{}", render);
}
