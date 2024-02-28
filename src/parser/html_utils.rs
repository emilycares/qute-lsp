use tree_sitter::{Parser, Tree};

pub fn html_inline<'a>(line: &str) -> Option<(Box<Tree>, String)> {
    let line = line;
    let mut parser = Parser::new();
    let language = tree_sitter_html::language();
    parser
        .set_language(language)
        .expect("Error loading html grammar");
    let content = format!("<button {}></button>", line);
    let Some(tree) = parser.parse(&content, None) else {
        return None;
    };
    let tree = Box::new(tree);
    let string_node = tree.root_node();
    if string_node.kind() != "attribute_value" {
        let mut parser = Parser::new();
        let language = tree_sitter_html::language();
        parser
            .set_language(language)
            .expect("Error loading html grammar");
        let line = format!("<button {}></button>", line);
        let Some(tree) = parser.parse(&line, None) else {
            return None;
        };
        return Some((Box::new(tree), line));
    }
    return Some((Box::new(*tree), line.to_owned()));
}
