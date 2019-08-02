extern crate cc;

use std::path::PathBuf;

fn main() {
    let tree_sitter_hello: PathBuf =
        std::fs::canonicalize::<PathBuf>(["..", "tree-sitter-hello", "src"].iter().collect())
            .unwrap();

    cc::Build::new()
        .include(&tree_sitter_hello)
        .file(tree_sitter_hello.join("parser.c"))
        .compile("tree-sitter-hello");
}
