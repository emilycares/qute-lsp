use tower_lsp::lsp_types::{CompletionItem, InsertTextFormat};

struct Completable<'a> {
    label: &'a str,
    detail: &'a str,
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
            Keyword::Let => "Defind a variable in scope",
            Keyword::If => "",
            Keyword::Else => "",
            Keyword::When => "",
            Keyword::Is => "",
            Keyword::IsIn => "",
            Keyword::Switch => "",
            Keyword::Case => "",
            Keyword::With => "",
            Keyword::Include => "",
            Keyword::Fragment => "",
            Keyword::Cached => "",
        }
    }
}

pub fn completion(line: String, char_pos: usize) -> Vec<CompletionItem> {
    let line_len = line.trim_end().len() - 1;
    let chars: &str = match line_len {
        0 => return vec![],
        1 => &line[char_pos - 1..char_pos],
        2 => &line[char_pos - 2..char_pos],
        3 | _ => &line[char_pos - 3..char_pos],
    }
    .trim();

    KEYWODS
        .iter()
        .filter(|(_, v)| v.contains(chars))
        .map(|(c, _)| c.into())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::completion;

    #[test]
    fn completion_crash() {
        let line = "{\r\n".to_string();
        completion(line, 1);
    }
}
