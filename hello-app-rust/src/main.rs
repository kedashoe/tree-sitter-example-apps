use tree_sitter::{Language, Parser};

extern "C" {
    fn tree_sitter_hello() -> Language;
}

fn main() {
    let mut parser = Parser::new();
    let language = unsafe { tree_sitter_hello() };
    parser.set_language(language).unwrap();

    let text = "hi mom";
    let tree = parser.parse(text, None);
    match tree {
        Some(t) => {
            let root = t.root_node();
            dbg!(root);
        }
        None => {
            println!("could not parse");
        }
    }
}
