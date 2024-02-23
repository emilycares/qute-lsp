use tower_lsp::lsp_types::{CompletionItem, InsertTextFormat};

struct Completable<'a> {
    label: &'a str,
    detail: &'a str,
}

impl Completable<'_> {
    pub fn to_lsp(&self, content: &str) -> CompletionItem {
        CompletionItem {
            label: self.label.to_string(),
            label_details: None,
            kind: None,
            detail: Some(self.detail.to_string()),
            documentation: None,
            deprecated: None,
            preselect: None,
            sort_text: None,
            filter_text: None,
            insert_text: Some(content.to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            insert_text_mode: None,
            text_edit: None,
            additional_text_edits: None,
            command: None,
            commit_characters: None,
            data: None,
            tags: None,
        }
    }
}

impl From<&Completable<'_>> for CompletionItem {
    fn from(value: &Completable) -> Self {
        CompletionItem {
            label: value.label.to_string(),
            label_details: None,
            kind: None,
            detail: Some(value.detail.to_string()),
            documentation: None,
            deprecated: None,
            preselect: None,
            sort_text: None,
            filter_text: None,
            insert_text: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            insert_text_mode: None,
            text_edit: None,
            additional_text_edits: None,
            command: None,
            commit_characters: None,
            data: None,
            tags: None,
        }
    }
}

pub enum Keyword {
    Comment,
    Variable,
    DoubleVariable,
    ForLoop,
    Each,
    Let,
    If,
    Else,
    When,
    Is,
    IsIn,
    Switch,
    Case,
    With,
    Include,
    Fragment,
    Cached,
}
#[macro_export]
macro_rules! keyword {
    ($x:expr) => {
        (
            Completable {
                label: $x.complete(),
                detail: $x.detail(),
            },
            $x.complete(),
        )
    };
}
const KEYWODS: [(Completable, &str); 17] = [
    keyword!(Keyword::Comment),
    keyword!(Keyword::Variable),
    keyword!(Keyword::DoubleVariable),
    keyword!(Keyword::ForLoop),
    keyword!(Keyword::Each),
    keyword!(Keyword::Let),
    keyword!(Keyword::If),
    keyword!(Keyword::Else),
    keyword!(Keyword::When),
    keyword!(Keyword::Is),
    keyword!(Keyword::IsIn),
    keyword!(Keyword::Switch),
    keyword!(Keyword::Case),
    keyword!(Keyword::With),
    keyword!(Keyword::Include),
    keyword!(Keyword::Fragment),
    keyword!(Keyword::Cached),
];

impl Keyword {
    pub const fn complete(&self) -> &'static str {
        match self {
            Keyword::Comment => "{! $0 !}",
            Keyword::Variable => "{ $0 }",
            Keyword::DoubleVariable => "{{ $0 }}",
            Keyword::ForLoop => "{#for $1 in $2}\n$0\n{/for}",
            Keyword::Each => "{#each items}\n$0\n{/each}",
            Keyword::Let => "{#let key=value}\n$0\n{/let}",
            Keyword::If => "{#if condition}\n$0\n{/if}",
            Keyword::Else => "{#else}$0",
            Keyword::When => "{#when $1}\n$0\n{/when}",
            Keyword::Is => "{#is $1}$0",
            Keyword::IsIn => "{#is in $1}$0",
            Keyword::Switch => "{#switch $1}\n$0\n{/switch}",
            Keyword::Case => "{#case $1}$0",
            Keyword::With => "{#with $1}\n$0\n{/with}",
            Keyword::Include => "{#include $1 /}$0",
            Keyword::Fragment => {
                "{#fragment id=$1} 
${0:<h1>Fragment works</h1>}
{/fragment}"
            }
            Keyword::Cached => "{#cached}$0{/cached}",
        }
    }
    pub const fn detail(&self) -> &'static str {
        match self {
            Keyword::Comment => "The content of a comment is completely ignored when rendering the output.",
            Keyword::Variable => "Render value.",
            Keyword::DoubleVariable => "Render value.",
            Keyword::ForLoop => "Go throw array",
            Keyword::Each => "Go throw list. The value will be assigned to \"it\"",
            Keyword::Let => "Define a variable in scope",
            Keyword::If => "Condition",
            Keyword::Else => "Else",
            Keyword::When => "A java switch like scructure",
            Keyword::Is => "Condition inside of when",
            Keyword::IsIn => "In condition inside of when",
            Keyword::Switch => "A java switch like scructure",
            Keyword::Case => "A case statement inside of switch",
            Keyword::With => "This section can be used to set the current context object",
            Keyword::Include => "Include from other templates",
            Keyword::Fragment => "A fragment represents a part of the template that can be treated as a separate template, i.e. rendered separately.",
            Keyword::Cached => "Cashable section",
        }
    }
}

pub fn completion(line: String, char_pos: usize) -> Vec<CompletionItem> {
    let chars = get_characters_before(line, char_pos);
    let chars = chars.trim();

    let chars = chars.trim_start_matches([
        ' ', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
        'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ]);

    KEYWODS
        .iter()
        .filter(|(_, v)| v.contains(chars))
        // remove prefix if already typed
        .map(|(c, _)| {
            if c.label.starts_with(chars) {
                return c.to_lsp(&c.label[chars.len()..]);
            }
            c.to_lsp(c.label)
        })
        .map(|c| c.into())
        .collect()
}

fn get_characters_before(line: String, char_pos: usize) -> String {
    let char_pos = char_pos.min(line.len());
    let start_pos = if char_pos >= 3 { char_pos - 3 } else { 0 };
    let length = char_pos - start_pos;
    line[start_pos..char_pos]
        .chars()
        .take(length)
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::completion;
    use pretty_assertions::assert_eq;

    #[test]
    fn completion_crash() {
        let line = "{\r\n".to_string();
        assert_eq!(completion(line, 1).len(), 17);

        let line = "{#".to_string();
        assert_eq!(completion(line, 2).len(), 14);

        let line = "{#f".to_string();
        assert_eq!(completion(line, 3).len(), 2);

        let line = "{@if".to_string();
        assert_eq!(completion(line, 4).len(), 0);

        let line = "{@whe".to_string();
        assert_eq!(completion(line, 4).len(), 0);
    }

    #[test]
    fn completion_trim_prefix() {
        let line = "{".to_string();
        let out = completion(line, 1);
        let out: Vec<Option<String>> = out.into_iter().map(|e| e.insert_text).collect();
        assert_eq!(
            out,
            vec![
                Some("! $0 !}".to_string()),
                Some(" $0 }".to_string()),
                Some("{ $0 }}".to_string()),
                Some("#for $1 in $2}\n$0\n{/for}".to_string()),
                Some("#each items}\n$0\n{/each}".to_string()),
                Some("#let key=value}\n$0\n{/let}".to_string()),
                Some("#if condition}\n$0\n{/if}".to_string()),
                Some("#else}$0".to_string()),
                Some("#when $1}\n$0\n{/when}".to_string()),
                Some("#is $1}$0".to_string()),
                Some("#is in $1}$0".to_string()),
                Some("#switch $1}\n$0\n{/switch}".to_string()),
                Some("#case $1}$0".to_string()),
                Some("#with $1}\n$0\n{/with}".to_string()),
                Some("#include $1 /}$0".to_string()),
                Some("#fragment id=$1} \n${0:<h1>Fragment works</h1>}\n{/fragment}".to_string()),
                Some("#cached}$0{/cached}".to_string())
            ]
        );
    }

    #[test]
    fn completion_trim_prefix_with_unwanted() {
        let line = "df{".to_string();
        let out = completion(line, 3);
        let out: Vec<Option<String>> = out.into_iter().map(|e| e.insert_text).collect();
        assert_eq!(
            out,
            vec![
                Some("! $0 !}".to_string()),
                Some(" $0 }".to_string()),
                Some("{ $0 }}".to_string()),
                Some("#for $1 in $2}\n$0\n{/for}".to_string()),
                Some("#each items}\n$0\n{/each}".to_string()),
                Some("#let key=value}\n$0\n{/let}".to_string()),
                Some("#if condition}\n$0\n{/if}".to_string()),
                Some("#else}$0".to_string()),
                Some("#when $1}\n$0\n{/when}".to_string()),
                Some("#is $1}$0".to_string()),
                Some("#is in $1}$0".to_string()),
                Some("#switch $1}\n$0\n{/switch}".to_string()),
                Some("#case $1}$0".to_string()),
                Some("#with $1}\n$0\n{/with}".to_string()),
                Some("#include $1 /}$0".to_string()),
                Some("#fragment id=$1} \n${0:<h1>Fragment works</h1>}\n{/fragment}".to_string()),
                Some("#cached}$0{/cached}".to_string())
            ]
        );
    }
}
