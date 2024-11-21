// SPDX-FileCopyrightText: 2024 Matthew Mark Ibbetson
// SPDX-FileContributor: Matthew Mark Ibbetson
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! This module should not exist in its current state.
//!
//! Due to have its components incorporated into the `content.rs` and `frontmatter.rs`
//! modules soon.

use once_cell::sync::Lazy;
use regex::Regex;

use crate::config::FrontmatterFormat;

/// Format string for use with `chrono`'s `format` function.
/// Represents the structure of a dn `Identifier`.
pub const DN_IDENTIFIER_FORMAT: &str = "%Y%m%dT%H%M%S";

/// Multiline-pattern regex to match the closing horizontal rule of `Text` front matter,
/// which should be exactly 27 '-' characters.
static REGEX_FRONTMATTER_TEXT_SUFFIX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)^\s*---------------------------\s*$").expect("Invalid text rule regex pattern")
});

/// Multiline-pattern regex to match the title line of `Text` front matter.
/// Contains a single capture group to extract the `Title` value.
///
/// ## Warning
///
/// May also improperly match some `Json` and `Yaml` titles if present.
static REGEX_FRONTMATTER_TEXT_TITLE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)^\s*title\s*:\s*(.+)\s*$").expect("Invalid text title regex pattern")
});

/// Multiline-pattern regex to match the tags line of `Text` front matter.
/// Contains a single capture group to extract the `Keywords` value.
///
/// ## Warning
///
/// May also improperly match some `Json` keywords if present.
static REGEX_FRONTMATTER_TEXT_KEYWORDS: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)^\s*tags\s*:\s*((?:\S+\s+)*\S+)\s*$")
        .expect("Invalid text keywords regex pattern")
});

/// Multiline-pattern regex to match the identifier line of `Text` front matter.
/// Contains a single capture group to extract the `Identifier` value.
static REGEX_FRONTMATTER_TEXT_IDENTIFIER: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)^\s*identifier\s*:\s*(\d{8}T\d{6})\s*$")
        .expect("Invalid text identifier regex pattern")
});

/// Multiline-pattern regex to match the opening and closing markers of `Yaml` front matter.
static REGEX_FRONTMATTER_YAML_CONTAINER: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^\s*---\s*$").expect("Invalid YAML container regex pattern"));

/// Multiline-pattern regex to match the title line of `Yaml` front matter.
/// Contains a single capture group to extract the `Title` value.
static REGEX_FRONTMATTER_YAML_TITLE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)^\s*title\s*:\s+(.+)\s*$").expect("Invalid YAML title regex pattern")
});

/// Multiline-pattern regex to match the tags line of `Yaml` front matter.
/// Contains a single capture group to extract the `Keywords` value.
static REGEX_FRONTMATTER_YAML_KEYWORDS: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?m)^\s*tags\s*:\s+\[\s*((?:.+,\s+).*".+")\s*\]\s*$"#)
        .expect("Invalid YAML keywords regex pattern")
});

/// Multiline-pattern regex to match the identifier line of `Yaml` front matter.
/// Contains a single capture group to extract the `Identifier` value.
static REGEX_FRONTMATTER_YAML_IDENTIFIER: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?m)^\s*identifier\s*:\s+("\d{8}T\d{6}")\s*$"#)
        .expect("Invalid YAML identifier regex pattern")
});

/// Multiline-pattern regex to match the opening and closing markers of `Toml` frontmatter.
static REGEX_FRONTMATTER_TOML_CONTAINER: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^\s*\+\+\+\s*$").expect("Invalid TOML container regex pattern"));

/// Multiline-pattern regex to match the title line of `Toml` front matter.
/// Contains a single capture group to extract the `Title` value.
static REGEX_FRONTMATTER_TOML_TITLE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?m)^\s*title\s*=\s*(".+")\s*$"#).expect("Invalid TOML title regex pattern")
});

/// Multiline-pattern regex to match the tags line of `Toml` front matter.
/// Contains a single capture group to extract the `Keywords` value.
static REGEX_FRONTMATTER_TOML_KEYWORDS: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?m)^\s*tags\s*=\s*\[\s*((?:".+",\s+).*".+",{0,1})\s*\]\s*$"#)
        .expect("Invalid TOML keywords regex pattern")
});

/// Multiline-pattern regex to match the identifier line of `Toml` front matter.
/// Contains a single capture group to extract the `Identifier` value.
static REGEX_FRONTMATTER_TOML_IDENTIFIER: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?m)^\s*identifier\s*=\s*("\d{8}T\d{6}")\s*$"#)
        .expect("Invalid TOML identifier regex pattern")
});

/// Multiline-pattern regex to match the opening marker of `Json` frontmatter.
static REGEX_FRONTMATTER_JSON_PREFIX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^\s*\{\s*$").expect("Invalid TOML container regex pattern"));

/// Multiline-pattern regex to match the opening marker of `Json` frontmatter.
static REGEX_FRONTMATTER_JSON_SUFFIX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^\s*\}\s*$").expect("Invalid TOML container regex pattern"));

/// Multiline-pattern regex to match the title line of `Json` front matter.
/// Contains a single capture group to extract the `Title` value.
static REGEX_FRONTMATTER_JSON_TITLE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)^\s*title:\s+(.+)\s*$").expect("Invalid Json title regex pattern")
});

/// Multiline-pattern regex to match the tags line of `Json` front matter.
/// Contains a single capture group to extract the `Keywords` value.
static REGEX_FRONTMATTER_JSON_KEYWORDS: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?m)^\s*tags:\s+\[\s*((?:".+",\s+).*".+")\s*\]\s*$"#)
        .expect("Invalid Json keywords regex pattern")
});

/// Multiline-pattern regex to match the identifier line of `Json` front matter.
/// Contains a single capture group to extract the `Identifier` value.
static REGEX_FRONTMATTER_JSON_IDENTIFIER: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?m)^\s*identifier:\s+(\"\d{8}T\d{6}\")\s*$"#)
        .expect("Invalid Json identifier regex pattern")
});

/// Attempts to acquire the first paragraph of an input string. If the paragraph is not followed by
/// an empty line (`\n\n` or `\r\n\r\n`) then it will return `None`.
///
/// # Example
///
/// ```
/// let input = "---\ntitle: Example\n---\nThis is the content.";
/// let frontmatter = get_first_paragraph(input);
/// assert_eq!(frontmatter, Some("---\ntitle: Example\n---".to_string()));
/// ```
pub fn get_first_paragraph(input_content: &str) -> Option<String> {
    if input_content.is_empty() {
        None
    } else {
        input_content
            .split_once("\n\n")
            .or_else(|| input_content.split_once("\r\n\r\n"))
            .and_then(|(p, _)| (!p.is_empty()).then(|| p.to_owned()))
    }
}

/// Checks if a `&str` conforms to one of the valid front matter formats: `Text`, `Yaml`, `Toml`, or `Json`.
/// Valid formats are determined by matching the structure of the front matter text. If there is a match,
/// that format is returned as Some(format). Otherwise, returns None.
///
/// # Example
///
/// ```
/// let valid_yaml = "---\ntitle: Example\n---";
/// let invalid_yaml = "title: Example";
/// assert_eq!(FrontmatterFormat::Yaml, get_valid_frontmatter_format(valid_yaml));
/// assert_eq!(None, get_valid_frontmatter_format(invalid_yaml));
/// ```
fn get_frontmatter_format(text: &str) -> Option<FrontmatterFormat> {
    let lines = text.lines().collect::<Vec<_>>();
    let first = lines.first();
    let last = lines.last();

    let is_text = last.map_or(false, |l| REGEX_FRONTMATTER_TEXT_SUFFIX.is_match(l));
    let is_yaml = first.map_or(false, |l| REGEX_FRONTMATTER_YAML_CONTAINER.is_match(l))
        && last.map_or(false, |l| REGEX_FRONTMATTER_YAML_CONTAINER.is_match(l));
    let is_toml = first.map_or(false, |l| REGEX_FRONTMATTER_TOML_CONTAINER.is_match(l))
        && last.map_or(false, |l| REGEX_FRONTMATTER_TOML_CONTAINER.is_match(l));
    let is_json = first.map_or(false, |l| REGEX_FRONTMATTER_JSON_PREFIX.is_match(l))
        && last.map_or(false, |l| REGEX_FRONTMATTER_JSON_SUFFIX.is_match(l));

    match (is_text, is_yaml, is_toml, is_json) {
        (true, _, _, _) => Some(FrontmatterFormat::Text),
        (_, true, _, _) => Some(FrontmatterFormat::Yaml),
        (_, _, true, _) => Some(FrontmatterFormat::Toml),
        (_, _, _, true) => Some(FrontmatterFormat::Json),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod content_partitioning {
        use crate::format::get_first_paragraph;

        #[test]
        fn separates_normal_paragraph() {
            // Arrange
            let input = "First paragraph.\n\nSecond paragraph.";
            let expected = Some("First paragraph.".to_string());

            // Act
            let result = get_first_paragraph(input);

            // Assert
            assert_eq!(
                expected, result,
                "\nInput: {input:#?}\nExpected paragraph: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn separates_frontmatter_with_empty_line() {
            // Arrange
            let input = "---\ntitle: Example\n---\n\nActual content.";
            let expected = Some("---\ntitle: Example\n---".to_string());

            // Act
            let result = get_first_paragraph(input);

            // Assert
            assert_eq!(
                expected, result,
                "\nInput: {input:#?}\nExpected paragraph: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn separates_empty_paragraph_into_none() {
            // Arrange
            let input = "\r\n\r\n";
            let expected = None;

            // Act
            let result = get_first_paragraph(input);

            // Assert
            assert_eq!(
                expected, result,
                "\nInput: {input:#?}\nExpected paragraph: {expected:#?}\nReceived: {result:#?}"
            );
        }
    }

    mod frontmatter_validity {
        use crate::{config::FrontmatterFormat, format::get_frontmatter_format};

        #[test]
        fn determines_valid_text_frontmatter() {
            // Arrange
            let input = "title:foo\nidentifier : 20241212T121212\n---------------------------";
            let expected = Some(FrontmatterFormat::Text);

            // Act
            let result = get_frontmatter_format(input);

            // Assert
            assert_eq!(
                expected, result,
                "\nInput: {input:#?}\nExpected: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn determines_valid_yaml_frontmatter() {
            // Arrange
            let input = "---\ndate: 2024-12-12@12:12:12+02:00\n---";
            let expected = Some(FrontmatterFormat::Yaml);

            // Act
            let result = get_frontmatter_format(input);

            // Assert
            assert_eq!(
                expected, result,
                "\nInput: {input:#?}\nExpected: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn determines_valid_toml_frontmatter() {
            // Arrange
            let input = "+++\ndate: 2024-12-12@12:12:12+02:00\n+++";
            let expected = Some(FrontmatterFormat::Toml);

            // Act
            let result = get_frontmatter_format(input);

            // Assert
            assert_eq!(
                expected, result,
                "\nInput: {input:#?}\nExpected: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn determines_valid_json_frontmatter() {
            // Arrange
            let input = "{\ntitle: \"Test Title!\"\ndate: \"2024-12-12@12:12:12+02:00\"\n}";
            let expected = Some(FrontmatterFormat::Json);

            // Act
            let result = get_frontmatter_format(input);

            // Assert
            assert_eq!(
                expected, result,
                "\nInput: {input:#?}\nExpected: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn determines_invalid_frontmatter_format() {
            // Arrange
            let input = "+++\n#+title: Test Title!\nidentifier = \"TestId\"\n#+date: [2024-12-12@12:12:12+02:00]\n---";
            let expected = None;

            // Act
            let result = get_frontmatter_format(input);

            // Assert
            assert_eq!(
                expected, result,
                "\nInput: {input:#?}\nExpected: {expected:#?}\nReceived: {result:#?}"
            );
        }
    }

    mod text_frontmatter {
        use super::*;

        #[test]
        fn rule_parses_correctly() {
            assert!(REGEX_FRONTMATTER_TEXT_SUFFIX.is_match("---------------------------\n")); // 27 dashes
            assert!(REGEX_FRONTMATTER_TEXT_SUFFIX.is_match("---------------------------")); // 27 dashes
            assert!(!REGEX_FRONTMATTER_TEXT_SUFFIX.is_match("--------------------------\n")); // 26 dashes
            assert!(!REGEX_FRONTMATTER_TEXT_SUFFIX.is_match("----------------------------\n"));
            // 28 dashes
        }

        #[test]
        fn title_parses_correctly() {
            // Arrange
            let input = "title:my-Test : A N3w title\n";
            let expected = "my-Test : A N3w title";

            // Act
            let result = REGEX_FRONTMATTER_TEXT_TITLE
                .captures(input)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str();

            // Assert
            assert_eq!(
                expected, result,
                "\nInput: {input:#?}\nExpected match: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn keywords_parse_correcly() {
            // Arrange
            let input = "tags:   foo    bar baz  boom\n";
            let expected = "foo    bar baz  boom";

            // Act
            let result = REGEX_FRONTMATTER_TEXT_KEYWORDS
                .captures(input)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str();

            // Assert
            assert_eq!(
                expected, result,
                "\nInput: {input:#?}\nExpected match: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn identifier_parses_correctly() {
            // Arrange
            let input = "identifier: 20241212T121212\n";
            let expected = "20241212T121212";

            // Act
            let result = REGEX_FRONTMATTER_TEXT_IDENTIFIER
                .captures(input)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str();

            // Assert
            assert_eq!(
                expected, result,
                "\nInput: {input:#?}\nExpected match: {expected:#?}\nReceived: {result:#?}"
            );
        }
    }

    mod yaml_frontmatter {
        use super::*;

        #[test]
        fn container_parses_correctly() {
            assert!(REGEX_FRONTMATTER_YAML_CONTAINER.is_match("---"));
            assert!(!REGEX_FRONTMATTER_YAML_CONTAINER.is_match("--"));
            assert!(!REGEX_FRONTMATTER_YAML_CONTAINER.is_match("----"));
        }

        #[test]
        fn title_parses_correctly() {
            // Arrange
            let input = "title:   \"my-Test -  A N3w title\"\n";
            let expected = "\"my-Test -  A N3w title\"";

            // Act
            let result = REGEX_FRONTMATTER_YAML_TITLE
                .captures(input)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str();

            // Assert
            assert_eq!(
                expected, result,
                "\nInput: {input:#?}\nExpected match: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn keywords_parse_correcly() {
            // Arrange
            let input = "tags:   [ \"foo\",    \"bar\", \"baz\",  \"boom\" ]\n";
            let expected = "\"foo\",    \"bar\", \"baz\",  \"boom\"";

            // Act
            let result = REGEX_FRONTMATTER_YAML_KEYWORDS
                .captures(input)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str();

            // Assert
            assert_eq!(
                expected, result,
                "\nInput: {input:#?}\nExpected match: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn identifier_parses_correctly() {
            // Arrange
            let input = "identifier: \"20241212T121212\"\n";
            let expected = "\"20241212T121212\"";

            // Act
            let result = REGEX_FRONTMATTER_YAML_IDENTIFIER
                .captures(input)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str();

            // Assert
            assert_eq!(
                expected, result,
                "\nInput: {input:#?}\nExpected match: {expected:#?}\nReceived: {result:#?}"
            );
        }
    }

    mod toml_frontmatter {
        use super::*;

        #[test]
        fn container_parses_correctly() {
            assert!(REGEX_FRONTMATTER_TOML_CONTAINER.is_match("+++"));
            assert!(!REGEX_FRONTMATTER_TOML_CONTAINER.is_match("++"));
            assert!(!REGEX_FRONTMATTER_TOML_CONTAINER.is_match("++++"));
        }

        #[test]
        fn title_parses_correctly() {
            // Arrange
            let input = "title=\"my-Test : A N3w title\"\n";
            let expected = "\"my-Test : A N3w title\"";

            // Act
            let result = REGEX_FRONTMATTER_TOML_TITLE
                .captures(input)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str();

            // Assert
            assert_eq!(
                expected, result,
                "\nInput: {input:#?}\nExpected match: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn keywords_parse_correcly() {
            // Arrange
            let input = "tags =   [ \"bar\", \"baz\", \"boom\",    \"foo\" ]\n";
            let expected = "\"bar\", \"baz\", \"boom\",    \"foo\"";

            // Act
            let result = REGEX_FRONTMATTER_TOML_KEYWORDS
                .captures(input)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str();

            // Assert
            assert_eq!(
                expected, result,
                "\nInput: {input:#?}\nExpected match: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn identifier_parses_correctly() {
            // Arrange
            let input = "identifier =\"20241212T121212\"\n";
            let expected = "\"20241212T121212\"";

            // Act
            let result = REGEX_FRONTMATTER_TOML_IDENTIFIER
                .captures(input)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str();

            // Assert
            assert_eq!(
                expected, result,
                "\nInput: {input:#?}\nExpected match: {expected:#?}\nReceived: {result:#?}"
            );
        }
    }

    mod json_frontmatter {
        use super::*;

        #[test]
        fn prefix_parses_correctly() {
            assert!(REGEX_FRONTMATTER_JSON_PREFIX.is_match("{"));
            assert!(!REGEX_FRONTMATTER_JSON_PREFIX.is_match("}"));
            assert!(!REGEX_FRONTMATTER_JSON_SUFFIX.is_match("\n{{\n"));
        }

        #[test]
        fn suffix_parses_correctly() {
            assert!(REGEX_FRONTMATTER_JSON_SUFFIX.is_match("}\n"));
            assert!(!REGEX_FRONTMATTER_JSON_SUFFIX.is_match("{"));
            assert!(!REGEX_FRONTMATTER_JSON_SUFFIX.is_match("}}"));
        }

        #[test]
        fn title_parses_correctly() {
            // Arrange
            let input = "{\ntitle:   \"my-Test_ A N3w title\"\n}";
            let expected = "\"my-Test_ A N3w title\"";

            // Act
            let result = REGEX_FRONTMATTER_JSON_TITLE
                .captures(input)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str();

            // Assert
            assert_eq!(
                expected, result,
                "\nInput: {input:#?}\nExpected match: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn keywords_parse_correcly() {
            // Arrange
            let input = "{\ntags:   [ \"bar\",   \"baz\", \"boom\", \"foo\"]\n}";
            let expected = "\"bar\",   \"baz\", \"boom\", \"foo\"";

            // Act
            let result = REGEX_FRONTMATTER_JSON_KEYWORDS
                .captures(input)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str();

            // Assert
            assert_eq!(
                expected, result,
                "\nInput: {input:#?}\nExpected match: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn identifier_parses_correctly() {
            // Arrange
            let input = "{\nidentifier:   \"20241212T121212\"\n}";
            let expected = "\"20241212T121212\"";

            // Act
            let result = REGEX_FRONTMATTER_JSON_IDENTIFIER
                .captures(input)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str();

            // Assert
            assert_eq!(
                expected, result,
                "\nInput: {input:#?}\nExpected match: {expected:#?}\nReceived: {result:#?}"
            );
        }
    }
}
