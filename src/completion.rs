use tower_lsp::lsp_types::{CompletionItem, InsertTextFormat};

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
        ($x, $x.complete())
    };
}
const KEYWODS: [(Keyword, &str); 17] = [
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
            Keyword::When => {
                "{#when $1}\n$0\n{/when}"
            }
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
}

pub fn completion(line: String, char_pos: usize) -> Vec<CompletionItem> {
    let mut chars: &str = "";
    match line.len() - 1 {
        0 => return vec![],
        1 => &line[char_pos - 1..char_pos],
        2 => &line[char_pos - 2..char_pos],
        3 | _ => &line[char_pos - 3..char_pos],
    };
    chars = chars.trim();

    dbg!(chars);

    KEYWODS
        .iter()
        .filter(|(_, v)| v.contains(chars))
        .map(|(_, v)| CompletionItem {
            label: v.to_string(),
            detail: Some("Common quarkus completion item".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..CompletionItem::default()
        })
        .collect()
}
