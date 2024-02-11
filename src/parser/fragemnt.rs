use dashmap::DashMap;
use tower_lsp::lsp_types::CompletionItem;

use crate::{file_utils::find_files, get_templates_folder};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, PartialEq)]
pub struct Fragment {
    pub id: String,
    pub source: String,
}
#[derive(Debug, PartialEq)]
pub struct Document {
    path: String,
    id: String,
    fragments: Vec<Fragment>,
}
pub fn scan_templates() -> Vec<Fragment> {
    let template_folder = get_templates_folder();
    let path = Path::new(&template_folder);
    if let Ok(files) = find_files(path) {
        return files
            .into_iter()
            .filter_map(|p| {
                if let Ok(con) = fs::read_to_string(p.clone()) {
                    if let Some(filename) = p.to_str() {
                        return Some((p.clone(), scan_fragments(con, filename.to_string())));
                    }
                }
                None
            })
            .flat_map(|(path, fragments)| {
                let prefix = get_fragment_prefix(path) + "$";
                return fragments
                    .iter()
                    .map(|fragment| Fragment {
                        id: prefix.clone() + &fragment.id,
                        source: fragment.source.clone(),
                    })
                    .collect::<Vec<_>>();
            })
            .collect();
    }

    vec![]
}

/// folder/file$frag
fn get_fragment_prefix(p: PathBuf) -> String {
    let mut p = p.clone();
    let mut out = vec![];
    if let Some(filename) = get_name(&p) {
        out.push(filename);
    }
    if let Some(parent) = p.parent() {
        p = parent.to_path_buf();
    }
    for _ in 0..5 {
        if !p.is_dir() {
            break;
        }
        if let Some(folder_name) = get_name(&p) {
            if folder_name == "templates" {
                break;
            }
            out.push("/".to_string());
            out.push(folder_name);
        }

        if let Some(parent) = p.parent() {
            p = parent.to_path_buf();
        }
    }
    out.into_iter().rev().collect()
}

fn get_name(p: &Path) -> Option<String> {
    if let Some(filename) = p.file_name() {
        if let Some(filename) = filename.to_str() {
            if let Some(ext) = p.extension() {
                return Some(filename[0..filename.len() - ext.len() - 1].to_string());
            }
            return Some(filename.to_string());
        }
    }
    None
}

pub fn scan_fragments(content: String, source: String) -> Vec<Fragment> {
    content
        .lines()
        .map(|l| l.trim())
        .filter(|l| l.contains("{#fragment"))
        .filter_map(|l| l.split_once("{#fragment"))
        .map(|c| c.1)
        .filter_map(|l| l.split_once("id="))
        .map(|c| c.1)
        .map(|c| c.replace('}', ""))
        .map(|id| Fragment {
            id: id.to_string(),
            source: source.clone(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::parser::fragemnt::{scan_fragments, Fragment};
    use pretty_assertions::assert_eq;

    #[test]
    fn scan_fragments_basic() {
        let content = "<h1>Items</h1>
<ol>
    {#for item in items}
    {#fragment id=item}   
    <li>{item.name}</li>  
    {/fragment}
    {/for}
</ol>
";

        let out = scan_fragments(content.to_string(), String::new());
        assert_eq!(
            out,
            vec![Fragment {
                id: "item".to_string(),
                source: String::new()
            }]
        )
    }
}

pub fn completion(
    fragment_map: &DashMap<String, Fragment>,
    line: String,
    char_pos: usize,
) -> Vec<CompletionItem> {
    let last_words: &str = &line[0..char_pos];
    if last_words.contains("{#include") {
        let optional_close = get_optional_clonse_statement(line);
        return fragment_map
            .iter()
            .map(|r| {
                CompletionItem::new_simple(
                    r.key().to_string() + optional_close,
                    "Fragment from project".to_string(),
                )
            })
            .collect::<Vec<_>>();
    }

    vec![]
}

fn get_optional_clonse_statement(line: String) -> &'static str {
    match line.contains("/}") {
        true => "",
        false => " /}",
    }
}
