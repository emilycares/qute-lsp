use dashmap::DashMap;
use tower_lsp::lsp_types::CompletionItem;
use tree_sitter::Node;

use crate::parser::html_utils::html_inline;

use super::route::Route;

pub fn completion(
    route_map: &DashMap<String, Route>,
    line: &str,
    char_pos: usize,
) -> Vec<CompletionItem> {
    let Some((tree, line)) = html_inline(line) else {
        return vec![];
    };
    let mut cursor = tree.walk();
    for _i in 0..10 {
        cursor.goto_first_child_for_point(tree_sitter::Point {
            row: 0,
            column: char_pos + 8,
        });
    }
    if &cursor.node().kind() != &"attribute_value" {
        return vec![];
    }
    if let Some(value) = get_completion_items(cursor.node(), line, route_map) {
        return value;
    }
    vec![]
}

fn get_completion_items(
    string_node: Node<'_>,
    content: String,
    route_map: &DashMap<String, Route>,
) -> Option<Vec<CompletionItem>> {
    let param_name = get_param_name(string_node, &content);
    let already_written = match string_node.utf8_text(content.as_bytes()) {
        Ok(s) => s,
        Err(_) => "",
    };
    if !can_complete_path_for_param_name(param_name) {
        return Some(vec![]);
    }
    return Some(
        route_map
            .iter()
            .map(|r| {
                CompletionItem {
                    label: r.key().to_string(),
                    detail: Some(r.value().to_string()),
                    insert_text: Some(r.key().trim_start_matches(already_written).to_string()),
                   ..Default::default()
                }
            })
            .collect::<Vec<_>>(),
    );
}

fn can_complete_path_for_param_name(param_name: Option<String>) -> bool {
    let Some(param_name) = param_name else {
        return false;
    };
    match param_name.as_str() {
        // html
        "action" => true,
        // htmx
        "hx-get" => true,
        "hx-post" => true,
        "hx-put" => true,
        "hx-path" => true,
        "hx-delete" => true,
        _ => false,
    }
}

fn get_param_name(node: Node, content: &str) -> Option<String> {
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
        let out = completion(&dm, "<button hx-get=\"/s\" hx-trigger=\"click\" hx-target=\"#selectStyle\" hx-swap=\"outerHTML\"></button>", 18);
        assert_eq!(
            out,
            vec![CompletionItem {
                label: "/start".to_string(),
                detail: Some("GET: \n".to_string()),
                insert_text: Some("tart".to_string()),
                ..CompletionItem::default()
            }]
        )
    }
    #[test]
    fn completion_basic_not() {
        let dm = DashMap::new();
        dm.insert("".to_string(), Route::default());
        let out = completion(&dm, "<button hx-get=\"/sel\" hx-trigger=\"click\" hx-target=\"#selectStyle\" hx-swap=\"outerHTML\"></button>", 63);
        assert_eq!(out, vec![])
    }

    #[test]
    fn completion_multi_line() {
        let dm = DashMap::new();
        dm.insert("/start".to_string(), Route::default());
        let out = completion(&dm, "hx-get=\"/\"", 9);
        assert_eq!(
            out,
            vec![CompletionItem {
                label: "/start".to_string(),
                detail: Some("GET: \n".to_string()),
                insert_text: Some("start".to_string()),
                ..CompletionItem::default()
            }]
        )
    }
}
