use std::{collections::HashMap, path::Path, str::FromStr};

use tower_lsp::lsp_types::{Position, TextEdit, Url};
use tree_sitter::{Language, Node, Parser, Point, Query, QueryCursor, Tree};

#[derive(Debug, PartialEq)]
pub enum TreesitterError {
    UnableToParse,
    NotCorrectDocumentSyntax,
    NoNodeFound,
    NoIdFoundOnElement,
    UnableToGetContent,
}
#[derive(Debug, PartialEq)]
pub enum ExtractionKind {
    AddFragement,
    ExtractAsFile,
    ExtractAsFragment,
}

impl ToString for ExtractionKind {
    fn to_string(&self) -> String {
        match self {
            ExtractionKind::AddFragement => "AddFragement",
            ExtractionKind::ExtractAsFile => "ExtractAsFile",
            ExtractionKind::ExtractAsFragment => "ExtractAsFragment",
        }
        .to_string()
    }
}

impl FromStr for ExtractionKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AddFragement" => Ok(Self::AddFragement),
            "ExtractAsFile" => Ok(Self::ExtractAsFile),
            "ExtractAsFragment" => Ok(Self::ExtractAsFragment),
            _ => Err(()),
        }
    }
}

pub fn check_extract(content: &str, point: Point) -> Vec<ExtractionKind> {
    let mut out = vec![];
    let language = tree_sitter_html::language();
    match get_tree(content, language) {
        Ok(tree) => match get_node_at_point(&tree, point) {
            Ok(node) => match get_element_node(node) {
                Ok(node) => {
                    out.push(ExtractionKind::AddFragement);
                    match get_id_of_node(language, node, content) {
                        Ok(_) => {
                            out.push(ExtractionKind::ExtractAsFile);
                            out.push(ExtractionKind::ExtractAsFragment);
                        }
                        Err(e) => eprintln!("Unable to get id of node {:?}", e),
                    }
                }
                Err(e) => eprintln!("Unable to get element node {:?}", e),
            },
            Err(e) => eprintln!("Unable to get node at point {:?}", e),
        },
        Err(e) => eprintln!("Unable to get tree {:?}", e),
    }

    out
}

pub fn add_fragment(
    uri: Url,
    point: Point,
    content: &str,
) -> Result<HashMap<Url, Vec<TextEdit>>, TreesitterError> {
    // parse
    let language = tree_sitter_html::language();
    let tree = get_tree(content, language)?;
    let node = get_node_at_point(&tree, point)?;
    let node = get_element_node(node)?;

    // get edit range
    let inner = node.range();
    let end = to_lsp_position(inner.end_point);
    let start = to_lsp_position(inner.start_point);

    Ok(HashMap::from([(
        uri,
        vec![
            TextEdit::new(
                tower_lsp::lsp_types::Range::new(end, end),
                "\n{/fragment}".to_string(),
            ),
            TextEdit::new(
                tower_lsp::lsp_types::Range::new(start, start),
                "{#fragment id=NEW_FRAGMENT }\n".to_string(),
            ),
        ],
    )]))
}

pub fn extract_as_file(
    url: Url,
    point: Point,
    content: &str,
) -> Result<HashMap<Url, Vec<TextEdit>>, TreesitterError> {
    let base_extract = base_extract(content, point, url.to_string())?;
    Ok(HashMap::from([
        (
            url.clone(),
            vec![TextEdit::new(
                tower_lsp::lsp_types::Range::new(base_extract.start, base_extract.end),
                format!("{{#include {} /}}", base_extract.id),
            )],
        ),
        (
            base_extract.new_url,
            vec![TextEdit::new(
                tower_lsp::lsp_types::Range::default(),
                base_extract.new_content,
            )],
        ),
    ]))
}

pub fn extract_as_fragment(
    url: Url,
    point: Point,
    content: &str,
) -> Result<HashMap<Url, Vec<TextEdit>>, TreesitterError> {
    let base_extract = base_extract(content, point, url.to_string())?;
    Ok(HashMap::from([
        (
            url.clone(),
            vec![TextEdit::new(
                tower_lsp::lsp_types::Range::new(base_extract.start, base_extract.end),
                format!("{{#include {}${} /}}", base_extract.id, base_extract.id),
            )],
        ),
        (
            base_extract.new_url,
            vec![TextEdit::new(
                tower_lsp::lsp_types::Range::default(),
                format!(
                    "{{#fragment id={}}}\n{}\n{{/fragment}}",
                    base_extract.id, base_extract.new_content
                ),
            )],
        ),
    ]))
}

struct BaseExtract {
    start: Position,
    end: Position,
    id: String,
    new_url: tower_lsp::lsp_types::Url,
    new_content: String,
}

fn base_extract(content: &str, point: Point, url: String) -> Result<BaseExtract, TreesitterError> {
    // parse
    let language = tree_sitter_html::language();
    let tree = get_tree(content, language)?;
    let node = get_node_at_point(&tree, point)?;
    let node = get_element_node(node)?;

    // get edit range
    let inner = node.range();
    let end = to_lsp_position(inner.end_point);
    let start = to_lsp_position(inner.start_point);

    // get content for generation
    let id = get_id_of_node(language, node, content)?;
    let Ok(new_content) = node.utf8_text(content.as_bytes()) else {
        return Err(TreesitterError::UnableToGetContent);
    };
    let Some(new_url) = get_url_with_id(url, &id) else {
        return Err(TreesitterError::UnableToGetContent);
    };

    Ok(BaseExtract {
        start,
        end,
        id,
        new_url,
        new_content: new_content.to_string(),
    })
}

fn get_url_with_id(url: String, id: &str) -> Option<Url> {
    let Some(folder) = Path::new(&url).parent() else {
        return None;
    };
    let path = format!("{}/{}.html", folder.to_str().unwrap_or_default(), id);
    let Ok(path) = Url::from_str(&path) else {
        return None;
    };
    Some(path)
}

fn to_lsp_position(point: Point) -> tower_lsp::lsp_types::Position {
    tower_lsp::lsp_types::Position::new(
        point.row.try_into().unwrap_or_default(),
        point.column.try_into().unwrap_or_default(),
    )
}

fn get_id_of_node<'a>(
    language: Language,
    node: Node<'a>,
    content: &'a str,
) -> Result<String, TreesitterError> {
    let mut cursor = QueryCursor::new();
    let query = "(start_tag
  (attribute
     (attribute_name) @_arrname
     (quoted_attribute_value
       (attribute_value) @_value
       (#eq? @_arrname \"id\")
     ) 
    )
  )";
    let query = match Query::new(language, query) {
        Ok(query) => query,
        Err(_) => return Err(TreesitterError::UnableToParse),
    };
    let captures = cursor.captures(&query, node, content.as_bytes());
    let matches: Vec<&str> = captures
        .into_iter()
        .flat_map(|(c, _)| c.captures)
        .filter(|c| c.node.kind() == "attribute_value")
        .map(|c| c.node.utf8_text(content.as_bytes()))
        .filter_map(|c| c.ok())
        .collect();

    if matches.is_empty() {
        return Err(TreesitterError::NoIdFoundOnElement);
    }
    let id = matches
        .first()
        .expect("There must be a entry a check was made before");

    Ok(id.trim().to_string())
}

fn range_includes_point(range: tree_sitter::Range, point: Point) -> bool {
    let row = range.start_point.row..range.end_point.row;
    let row_eq = range.start_point.row == range.end_point.row;

    if row.contains(&point.row) || (row_eq && range.start_point.row == point.row) {
        let column = range.start_point.column..range.end_point.column;
        let column_eq = range.start_point.column == range.end_point.column;
        if column.contains(&point.column) || (column_eq && range.start_point.column == point.column)
        {
            return true;
        }
    }
    false
}

fn get_node_at_point(tree: &Tree, point: Point) -> Result<Node, TreesitterError> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    loop {
        let Some(_node_index) = cursor.goto_first_child_for_point(point) else {
            return Err(TreesitterError::NoNodeFound);
        };
        let node = cursor.node();

        // Do not loop forever
        if node.child_count() == 0 {
            break;
        }
    }

    Ok(cursor.node())
}

fn get_element_node(node: Node<'_>) -> Result<Node<'_>, TreesitterError> {
    let mut node = node;
    loop {
        if node.kind() == "element" {
            return Ok(node);
        }

        let Some(parent) = node.parent() else {
            return Err(TreesitterError::NoNodeFound);
        };
        node = parent;
    }
}

fn get_tree(content: &str, language: Language) -> Result<Tree, TreesitterError> {
    let mut parser = Parser::new();
    if parser.set_language(language).is_err() {
        return Err(TreesitterError::UnableToParse);
    };

    let Some(tree) = parser.parse(content, None) else {
        return Err(TreesitterError::NotCorrectDocumentSyntax);
    };

    Ok(tree)
}

#[cfg(test)]
mod tests {
    use crate::parser::document::{get_element_node, ExtractionKind};

    use super::{check_extract, get_id_of_node, get_node_at_point, get_tree, range_includes_point};
    use pretty_assertions::assert_eq;
    use tree_sitter::{Point, Range};
    const DOCUMENT: &str = "
<!doctype html>
<html lang=\"en\">
<head>
	<title>Real example</title>
</head>
<body>
<div id=\"did\">
	<div>
		<h1>Hey</h1>
	</div>
</div>
</body>";

    #[test]
    fn could_extract_basic() {
        let point = tree_sitter::Point { row: 7, column: 2 };
        let out = check_extract(DOCUMENT, point);
        assert_eq!(
            out,
            vec![
                ExtractionKind::AddFragement,
                ExtractionKind::ExtractAsFile,
                ExtractionKind::ExtractAsFragment
            ]
        );
    }
    #[test]
    fn could_extract_no_id() {
        let point = tree_sitter::Point { row: 8, column: 2 };
        let out = check_extract(DOCUMENT, point);
        assert_eq!(out, vec![ExtractionKind::AddFragement]);
    }
    #[test]
    fn get_id_of_node_basic() {
        let language = tree_sitter_html::language();
        let tree = get_tree(DOCUMENT, language).unwrap();
        let point = tree_sitter::Point { row: 7, column: 2 };
        let node = get_node_at_point(&tree, point).unwrap();
        let node = get_element_node(node).unwrap();
        let id = get_id_of_node(language, node, DOCUMENT);
        assert_eq!(id, Ok("did".to_string()));
    }
    #[test]
    fn get_tree_basic() {
        let language = tree_sitter_html::language();
        assert!(get_tree(DOCUMENT, language).is_ok());
    }

    #[test]
    fn range_includes_point_basic() {
        let point = Point { row: 0, column: 7 };
        let range = Range {
            start_byte: 0,
            end_byte: 16,
            start_point: Point { row: 0, column: 0 },
            end_point: Point { row: 0, column: 16 },
        };
        assert!(range_includes_point(range, point));
    }
}
