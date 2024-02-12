use dashmap::DashMap;
use tower_lsp::lsp_types::CompletionItem;
use tree_sitter::{Node, Parser};

use super::route::Route;

pub fn completion(
    route_map: &DashMap<String, Route>,
    line: String,
    char_pos: usize,
) -> Vec<CompletionItem> {
    let content = line.to_owned();
    let mut parser = Parser::new();
    let language = tree_sitter_html::language();
    parser
        .set_language(language)
        .expect("Error loading html grammar");
    let Some(tree) = parser.parse(&content, None) else {
        return vec![];
    };
    let mut cursor = tree.walk();
    for _ in 0..50 {
        cursor.goto_first_child_for_point(tree_sitter::Point {
            row: 0,
            column: char_pos,
        });
    }
    let string_node = cursor.node();
    if string_node.kind() != "attribute_value" {
        return handle_muilty_line(route_map, &content, char_pos);
    }
    if let Some(value) = get_completion_items(string_node, content, route_map) {
        return value;
    }
    vec![]
}

fn handle_muilty_line<'a, 'b>(route_map: &DashMap<String, Route>,line: &'a str, char_pos: usize) -> Vec<CompletionItem> {
    let mut parser = Parser::new();
    let language = tree_sitter_html::language();
    parser
        .set_language(language)
        .expect("Error loading html grammar");
    let content = format!("<button {}></buton>", line);
    let Some(tree) = parser.parse(&content, None) else {
        return vec![];
    };
    let mut cursor = tree.walk();
    for _ in 0..50 {
        cursor.goto_first_child_for_point(tree_sitter::Point {
            row: 0,
            column: char_pos + 8,
        });
    }
    let string_node = cursor.node();
    if string_node.kind() != "attribute_value" {
        cursor.reset(tree.root_node());
        return vec![];
    }
    if let Some(value) = get_completion_items(string_node, content, route_map) {
        return value;
    }
    vec![]
}

fn get_completion_items(string_node: Node<'_>, content: String, route_map: &DashMap<String, Route>) -> Option<Vec<CompletionItem>> {
    let param_name = get_param_name(string_node, &content);
    if !can_complet_path_for_param_name(param_name) {
        return Some(vec![]);
    }
    return Some(route_map
        .iter()
        .map(|r| {
            CompletionItem::new_simple(
                r.key().to_string(), /* + optional_close*/
                r.value().to_string()
            )
        })
        .collect::<Vec<_>>());
}

fn can_complet_path_for_param_name(param_name: Option<String>) -> bool {
    let Some(param_name) = param_name else {
        return false;
    };
    match param_name.as_str() {
        "hx-get" => true,
        _ => false,
    }
}

fn get_param_name<'a, 'b>(node: Node, content: &'a str) -> Option<String> {
    let Some(node) = node.prev_sibling() else {
        return None;
    };
    let Some(node) = node.parent() else {
        return None;
    };
    let Some(node) = node.prev_sibling() else {
        return None;
    };
    let Some(node) = node.prev_sibling() else {
        return None;
    };
    if let Ok(n) = node.utf8_text(content.as_bytes()) {
        return Some(n.to_string());
    };
    None
}

#[cfg(test)]
mod tests {
    use dashmap::DashMap;
    use pretty_assertions::assert_eq;
    use tower_lsp::lsp_types::CompletionItem;

    use crate::parser::{route::Route, route_completion::completion};

    #[test]
    fn completion_basic() {
        let dm = DashMap::new();
        dm.insert("/start".to_string(), Route::default());
        let out = completion(&dm, "<button hx-get=\"/sel\" hx-trigger=\"click\" hx-target=\"#selectStyle\" hx-swap=\"outerHTML\"></button>".to_string(), 18);
        assert_eq!(
            out,
            vec![CompletionItem {
                label: "/start".to_string(),
                detail: Some("GET: \n".to_string()),
                ..CompletionItem::default()
            }]
        )
    }
    #[test]
    fn completion_basic_not() {
        let dm = DashMap::new();
        dm.insert("".to_string(), Route::default());
        let out = completion(&dm, "<button hx-get=\"/sel\" hx-trigger=\"click\" hx-target=\"#selectStyle\" hx-swap=\"outerHTML\"></button>".to_string(), 63);
        assert_eq!(out, vec![])
    }
    #[test]
    fn completion_multi_line() {
        let dm = DashMap::new();
        dm.insert("/start".to_string(), Route::default());
        let out = completion(&dm, "hx-get=\"/sel\"".to_string(), 9);
        assert_eq!(
            out,
            vec![CompletionItem {
                label: "/start".to_string(),
                detail: Some("GET: \n".to_string()),
                ..CompletionItem::default()
            }]
        )
    }
}
