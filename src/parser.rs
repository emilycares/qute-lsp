#[derive(Debug, PartialEq)]
pub enum QuteInclude {
    Basic(String),
    Fragment(QuteIncludeFragement),
}
#[derive(Debug, PartialEq)]
pub struct QuteIncludeFragement {
    template: String,
    fragment: String,
}

/// This will return the template name for a include section
pub fn parse_include(line: String) -> Option<QuteInclude> {
    let Some((_, line)) = line.trim().split_once("{#include ") else {
        // Not an include line
        return None;
    };
    let Some((template, _)) = line.split_once(" ") else {
        // Incase of detail. With html params
        let name = line.replace('}', "");
        return Some(QuteInclude::Basic(name));
    };
    // Incase of fragment
    if let Some((template, fragment)) = line.split_once('$') {
        if let Some((fragment, _)) = fragment.split_once(' ') {
            return Some(QuteInclude::Fragment(QuteIncludeFragement {
                template: template.to_string(),
                fragment: fragment.to_string(),
            }));
        }
    }

    let name = template.to_string();
    return Some(QuteInclude::Basic(name));
}

#[cfg(test)]
mod tests {
    use crate::parser::{QuteInclude, QuteIncludeFragement};

    use super::parse_include;
    use pretty_assertions::assert_eq;

    #[test]
    fn basic() {
        assert_eq!(
            parse_include("{#include foo limit=10 /}".to_string()),
            Some(QuteInclude::Basic("foo".to_string()))
        );
    }
    #[test]
    fn basic_folder() {
        assert_eq!(
            parse_include("{#include snippets/tailwind /}".to_string()),
            Some(QuteInclude::Basic("snippets/tailwind".to_string()))
        );
    }

    #[test]
    fn fragment() {
        assert_eq!(
            parse_include("{#include select$user target=target /}".to_string()),
            Some(QuteInclude::Fragment(QuteIncludeFragement {
                template: "select".to_string(),
                fragment: "user".to_string()
            }))
        );
    }

    #[test]
    fn detail() {
        assert_eq!(
            parse_include("{#include detail}".to_string()),
            Some(QuteInclude::Basic("detail".to_string()))
        );
    }
}
