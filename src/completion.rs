use dashmap::DashMap;
use tower_lsp::lsp_types::CompletionItem;

use crate::parser::fragemnt::Fragment;

pub enum Keyword {
    Comment,
    Variable,
    DoubleVariable,
    ForLoop,
    Each,
    Let,
    If,
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

impl Keyword {
    pub fn complete(&self) {
        match &self {
            Keyword::Comment => "{! !}",
            Keyword::Variable => "{ }",
            Keyword::DoubleVariable => "{{ }}",
            Keyword::ForLoop => "{#for item in items} {/for}",
            Keyword::Each => "{#each items} {/each}",
            Keyword::Let => "{#let key=value} {/let}",
            Keyword::If => "{#if condition} {/if}",
            Keyword::When => 
                "{#when items.size}
  {#is 1} 
    There is exactly one item!
  {#is > 10} 
    There are more than 10 items!
  {#else} 
    There are 2 -10 items!
{/when}"
            
            Keyword::With => todo!(),
            Keyword::Include => todo!(),
            Keyword::Fragment => todo!(),
            Keyword::Cached => todo!(),
        }
    }
}

pub fn completion(
    fragment_map: &DashMap<String, Fragment>,
    line: String,
    char_pos: usize,
) -> Vec<CompletionItem> {
    let mut chars: &str;
    match line.len() {
        0 => return vec![],
        1 => &line[char_pos - 1..char_pos],
        2 => &line[char_pos - 2..char_pos],
        3 | _ => &line[char_pos - 3..char_pos],
    };
    chars = chars.trim();

    //match chars {
    ////"{" =>
    //}
}
