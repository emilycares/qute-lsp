use std::str::FromStr;

use serde_json::Value;
use tower_lsp::lsp_types::{ExecuteCommandParams, Url};
use tree_sitter::Point;

pub fn parse(params: ExecuteCommandParams) -> (Point, Option<Url>) {
    let mut uri: String = String::new();
    let mut row: usize = 0;
    let mut column: usize = 0;
    for (i, arguments) in params.arguments.into_iter().enumerate() {
        match arguments {
            Value::String(string) => {
                if i == 0 {
                    uri = string;
                }
            }
            Value::Number(number) => {
                if i == 1 {
                    row = number
                        .as_u64()
                        .unwrap_or_default()
                        .try_into()
                        .unwrap_or_default();
                }
                if i == 2 {
                    column = number
                        .as_u64()
                        .unwrap_or_default()
                        .try_into()
                        .unwrap_or_default();
                }
            }
            Value::Null => (),
            Value::Bool(_) => (),
            Value::Array(_) => (),
            Value::Object(_) => (),
        }
    }
    let point = Point { row, column };
    let url = Url::from_str(&uri).ok();
    (point, url)
}
