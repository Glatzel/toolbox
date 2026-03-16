use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::{env, fs};

use goblin::pe::PE;

fn find_dll(name: &str, base: &Path) -> Option<PathBuf> {
    let candidate = base.join(name);
    if candidate.exists() {
        return Some(candidate);
    }

    if let Ok(path_env) = env::var("PATH") {
        for p in env::split_paths(&path_env) {
            let candidate = p.join(name);
            if candidate.exists() {
                return Some(candidate);
            }
        }
    }

    None
}

fn walk(path: &Path, indent: usize, visited: &mut HashSet<String>) {
    let buf = match fs::read(path) {
        Ok(b) => b,
        Err(_) => return,
    };

    let pe = match PE::parse(&buf) {
        Ok(p) => p,
        Err(_) => return,
    };

    let base_dir = path.parent().unwrap_or(Path::new("."));

    for import in pe.imports {
        let dll = import.dll.to_ascii_lowercase();

        if visited.contains(&dll) {
            continue;
        }

        visited.insert(dll.clone());

        for _ in 0..indent {
            print!("  ");
        }

        println!("{}", dll);

        if let Some(dep_path) = find_dll(&dll, base_dir) {
            walk(&dep_path, indent + 1, visited);
        }
    }
}

fn main() {
    let exe = env::args().nth(1).expect("usage: pe-tree <exe>");

    let mut visited = HashSet::new();
    walk(Path::new(&exe), 0, &mut visited);
}
