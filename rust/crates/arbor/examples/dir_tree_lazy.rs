use std::path::PathBuf;

use arbor::indents::UnicodeIndent;
use arbor::protocol::{ILazyTree, ITreeContent};
use arbor::renders::LazyRender;

struct LazyDirTree {
    pub path: std::path::PathBuf,
}
impl ITreeContent for LazyDirTree {
    fn content(&self) -> impl AsRef<str> { self.path.to_string_lossy() }
}
impl ILazyTree for LazyDirTree {
    type Leaf = LazyDirTree;
    type Leaves = std::vec::IntoIter<LazyDirTree>;
    fn leaves(&self) -> Self::Leaves {
        std::fs::read_dir(&self.path)
            .map(|entries| {
                let mut children: Vec<LazyDirTree> = entries
                    .filter_map(|entry| entry.ok())
                    .map(|entry| LazyDirTree { path: entry.path() })
                    .collect();
                children.sort_by_key(|c| c.path.clone());
                children
            })
            .unwrap_or_default()
            .into_iter()
    }
}

fn wrapper() {
    let tree = LazyDirTree {
        path: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
    };
    let render = LazyRender {
        tree: tree,
        indent: UnicodeIndent,
        width: 0,
    };
    let rendered = render.to_string();
    println!("{}", rendered);
}

fn main() { wrapper(); }
#[test]
fn test() { wrapper(); }
