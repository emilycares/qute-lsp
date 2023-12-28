use tree_sitter::{Language, Node, Parser, Point, Query, QueryCursor, Tree};

#[derive(Debug, PartialEq)]
pub enum TreesitterError {
    UnableToParse,
    NotCorrectDocumentSyntax,
    NoNodeFound,
    NoIdFoundOnElement,
}

pub fn could_extract(content: &str, row: usize, column: usize) -> Result<bool, TreesitterError> {
    let language = tree_sitter_html::language();
    let tree = get_tree(content, language)?;
    let node = get_node_at_point(&tree, tree_sitter::Point { row, column })?;
    let id = get_id_of_node(language, node, content)?;

    Ok(true)
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
     (quoted_attribute_value (attribute_value)) @_csd
    )
  (#eq? @_arrname \"id\")
  )";
    let query = match Query::new(language, query) {
        Ok(query) => query,
        Err(_) => return Err(TreesitterError::UnableToParse),
    };
    let captures = cursor.captures(&query, node, content.as_bytes());
    let matches: Vec<&str> = captures
        .into_iter()
        .flat_map(|(c, _)| c.captures)
        .filter(|c| c.node.kind() == "quoted_attribute_value")
        .map(|c| c.node.utf8_text(content.as_bytes()))
        .inspect(|c| {
            dbg!(c);
        })
        .filter(|c| c.is_ok())
        .map(|c| c.unwrap())
        .collect();

    if matches.len() == 0 {
        return Err(TreesitterError::NoIdFoundOnElement);
    }
    let id = matches
        .first()
        .expect("There must be a entry a check was made before");

    Ok(id.replace('"', "").replace('\'', "")
        .to_string())
}

fn get_node_at_point(tree: &Tree, point: Point) -> Result<Node, TreesitterError> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let Some(_node_index) = cursor.goto_first_child_for_point(point) else {
        return Err(TreesitterError::NoNodeFound);
    };

    Ok(cursor.node())
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
    use super::{could_extract, get_id_of_node, get_node_at_point, get_tree};
    use pretty_assertions::assert_eq;
    const DOCUMENT: &str =
        "<div id=\"inner\">Hey i am the inner</div><div id=\"other\">Hey i am the inner</div>";

    #[test]
    fn could_extract_basic() {
        let out = could_extract(DOCUMENT, 0, 7);
        assert_eq!(out, Ok(true));
    }
    #[test]
    fn get_id_of_node_basic() {
        let language = tree_sitter_html::language();
        let tree = get_tree(DOCUMENT, language).unwrap();
        let node = get_node_at_point(&tree, tree_sitter::Point { column: 7, row: 0 }).unwrap();
        let id = get_id_of_node(language, node, DOCUMENT);
        assert_eq!(id, Ok("inner".to_string()));
    }
    #[test]
    fn get_tree_basic() {
        let language = tree_sitter_html::language();
        assert!(get_tree(DOCUMENT, language).is_ok());
    }
}
