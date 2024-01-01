use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::get_templates_folder_from_template_uri;

#[derive(Debug, PartialEq)]
pub struct Fragment {
    id: String,
}
#[derive(Debug, PartialEq)]
pub struct Document {
    path: String,
    id: String,
    fragments: Vec<Fragment>,
}
pub fn scan_templates(some_template_path: &str, content: &str) -> Vec<Fragment> {
    let Some(template_folder) = get_templates_folder_from_template_uri(&some_template_path) else {
        eprintln!("Unable to retrieve template folder");
        return vec![];
    };
    let path = Path::new(&template_folder);
    let files = find_files(path);
    dbg!(files.ok());
    vec![]
}
fn find_files(path: impl AsRef<Path>) -> std::io::Result<Vec<PathBuf>> {
    let mut buf = vec![];
    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let meta = entry.metadata()?;

        if meta.is_dir() {
            let mut subdir = find_files(entry.path())?;
            buf.append(&mut subdir);
        }

        if meta.is_file() {
            buf.push(entry.path());
        }
    }

    Ok(buf)
}

pub fn scan_fragments(content: String) -> Vec<Fragment> {
    content
        .lines()
        .map(|l| l.trim())
        .filter(|l| l.contains("{#fragment"))
        .filter_map(|l| l.split_once("{#fragment"))
        .map(|c| c.1)
        .filter_map(|l| l.split_once("id="))
        .map(|c| c.1)
        .map(|c| c.replace('}', ""))
        .inspect(|c| {
            dbg!(c);
        })
        .map(|id| Fragment { id: id.to_string() })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::parser::complete::{scan_fragments, Fragment};
    use pretty_assertions::assert_eq;

    #[test]
    fn scan_fragments_basic() {
        let conent = "<h1>Items</h1>
<ol>
    {#for item in items}
    {#fragment id=item}   
    <li>{item.name}</li>  
    {/fragment}
    {/for}
</ol>
";

        let out = scan_fragments(conent.to_string());
        assert_eq!(
            out,
            vec![Fragment {
                id: "item".to_string()
            }]
        )
    }
}
