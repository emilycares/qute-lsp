use crate::parser::html_utils::html_inline;

use super::route::Route;
use dashmap::DashMap;
use tower_lsp::lsp_types::{GotoDefinitionResponse, Position};
//use tree_sitter::{Parser, TreeCursor};

pub fn get_definition(
    route_map: &DashMap<String, Route>,
    line: &str,
    position: &Position,
) -> Option<GotoDefinitionResponse> {
    let Some((tree, line)) = html_inline(line) else {
        return None;
    };
    let mut cursor = tree.walk();
    for _i in 0..10 {
        cursor.goto_first_child_for_point(tree_sitter::Point {
            row: 0,
            column: (position.character as usize) + 8,
        });
    }
    if &cursor.node().kind() != &"attribute_value" {
        // When this is not a attribute_value then we cannot provide as look
        return None;
    }
    let Ok(url) = &cursor.node().utf8_text(line.as_bytes()) else {
        // When we are not able to get the path we are not able to find out with path
        return None;
    };
    get_related_route(route_map, url)
        .filter(|r| r.implementation.is_some())
        .map(|route| GotoDefinitionResponse::Scalar(route.implementation.clone().unwrap()))
}

fn get_related_route<'a>(route_map: &'a DashMap<String, Route>, url: &'a str) -> Option<Route> {
    let url = without_vars(url);
    return route_map
        .into_iter()
        .find(|e| without_vars(e.key()) == url)
        .map(|e| e.value().to_owned());
}

/// In qute the route path could be partialy filled with variables. We remove these dynamic parts
/// to match against urls.
/// In quarkus route urls we can have variables we remove these so we can match them aginst the
/// qute typed one.
fn without_vars(url: &str) -> String {
    let mut open = false;
    let mut out = String::new();
    for c in url.chars() {
        match c {
            '{' => open = true,
            '}' => open = false,
            _ => {
                if !open {
                    out.push(c)
                }
            }
        }
    }
    return out;
}

#[cfg(test)]
mod tests {
    use dashmap::DashMap;
    use tower_lsp::lsp_types::{GotoDefinitionResponse, Position, Url};

    use crate::parser::{route::Route, route_definiton::without_vars};

    use super::get_definition;

    #[test]
    fn route_definition_basic() {
        let map = DashMap::new();
        map.insert(
            "/{first_param}/select/{second_param}".to_owned(),
            Route {
                implementation: Some(tower_lsp::lsp_types::Location {
                    uri: Url::parse("http://localhost/src/test.java").unwrap(),
                    range: tower_lsp::lsp_types::Range::default(),
                }),
                ..Default::default()
            },
        );
        let pos = Position::new(0, 12);
        let out = get_definition(&map, "			hx-get=\"/{id}/select/{toSelect}\"", &pos);

        assert_eq!(
            out,
            Some(GotoDefinitionResponse::Scalar(
                tower_lsp::lsp_types::Location {
                    uri: Url::parse("http://localhost/src/test.java").unwrap(),
                    range: tower_lsp::lsp_types::Range::default()
                }
            ))
        );
    }

    #[test]
    fn without_vars_base() {
        assert_eq!(without_vars("/{id}/select/{participant.uuid}"), "//select/");
    }
}
