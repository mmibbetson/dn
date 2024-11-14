// SPDX-FileCopyrightText: 2024 Matthew Mark Ibbetson
//
// SPDX-License-Identifier: GPL-3.0-or-later

use once_cell::sync::Lazy;
use regex::Regex;

/// Format string for use with `chrono`'s `format` function.
/// Represents the structure of a dn `Identifier`.
pub const DN_IDENTIFIER_FORMAT: &str = "%Y%m%dT%H%M%S";

/// Multiline-pattern regex to match the closing horizontal rule of `Text` front matter,
/// which should be exactly 27 '-' characters.
static REGEX_FRONTMATTER_TEXT_RULE: Lazy<Regex> = Lazy::new(|| {
    Regex::new("(?m)^---------------------------$").expect("Invalid text rule regex pattern")
});

/// Multiline-pattern regex to match the title line of `Text` front matter.
/// Contains a single capture group to extract the `Title` value.
///
/// ## Warning
///
/// May also improperly match some `Org` and `Yaml` titles if present.
static REGEX_FRONTMATTER_TEXT_TITLE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^title\s*:\s*(.+)$").expect("Invalid text title regex pattern"));

/// Multiline-pattern regex to match the date line of `Text` front matter.
/// Contains a single capture group to extract the `DateTime` value.
///
/// ## Warning
///
/// May also improperly match some `Org` and `Yaml` dates if present.
static REGEX_FRONTMATTER_TEXT_DATETIME: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^date\s*:\s*(.+)$").expect("Invalid text date regex pattern"));

/// Multiline-pattern regex to match the tags line of `Text` front matter.
/// Contains a single capture group to extract the `Keywords` value.
///
/// ## Warning
///
/// May also improperly match some `Org` keywords if present.
static REGEX_FRONTMATTER_TEXT_KEYWORDS: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)^tags\s*:\s*((?:\S+\s+)*\S+)$").expect("Invalid text keywords regex pattern")
});

/// Multiline-pattern regex to match the identifier line of `Text` front matter.
/// Contains a single capture group to extract the `Identifier` value.
static REGEX_FRONTMATTER_TEXT_IDENTIFIER: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)^identifier\s*:\s*(\d{8}T\d{6})$")
        .expect("Invalid text identifier regex pattern")
});

/// Multiline-pattern regex to match the opening and closing markers of `Yaml` front matter.
static REGEX_FRONTMATTER_YAML_CONTAINER: Lazy<Regex> =
    Lazy::new(|| Regex::new("(?m)^---$").expect("Invalid YAML container regex pattern"));

/// Multiline-pattern regex to match the title line of `Yaml` front matter.
/// Contains a single capture group to extract the `Title` value.
static REGEX_FRONTMATTER_YAML_TITLE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?m)^title\s*:\s+(".+")$"#).expect("Invalid YAML title regex pattern")
});

/// Multiline-pattern regex to match the date line of `Yaml` front matter.
/// Contains a single capture group to extract the `DateTime` value.
///
/// ## Warning
///
/// May also improperly match some `Text` and `Org` dates if present.
static REGEX_FRONTMATTER_YAML_DATETIME: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^date\s*:\s+(.+)$").expect("Invalid YAML date regex pattern"));

/// Multiline-pattern regex to match the tags line of `Yaml` front matter.
/// Contains a single capture group to extract the `Keywords` value.
static REGEX_FRONTMATTER_YAML_KEYWORDS: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?m)^tags\s*:\s+(\[(?:".+",\s+).*".+"\])$"#)
        .expect("Invalid YAML keywords regex pattern")
});

/// Multiline-pattern regex to match the identifier line of `Yaml` front matter.
/// Contains a single capture group to extract the `Identifier` value.
static REGEX_FRONTMATTER_YAML_IDENTIFIER: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?m)^identifier\s*:\s+("\d{8}T\d{6}")$"#)
        .expect("Invalid YAML identifier regex pattern")
});

/// Multiline-pattern regex to match the opening and closing markers of `Toml` frontmatter.
static REGEX_FRONTMATTER_TOML_CONTAINER: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^\+\+\+$").expect("Invalid TOML container regex pattern"));

/// Multiline-pattern regex to match the title line of `Toml` front matter.
/// Contains a single capture group to extract the `Title` value.
static REGEX_FRONTMATTER_TOML_TITLE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?m)^title\s*=\s*(".+")$"#).expect("Invalid TOML title regex pattern")
});

/// Multiline-pattern regex to match the date line of `Toml` front matter.
/// Contains a single capture group to extract the `DateTime` value.
static REGEX_FRONTMATTER_TOML_DATETIME: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^date\s*=\s*(.+)$").expect("Invalid TOML date regex pattern"));

/// Multiline-pattern regex to match the tags line of `Toml` front matter.
/// Contains a single capture group to extract the `Keywords` value.
static REGEX_FRONTMATTER_TOML_KEYWORDS: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?m)^tags\s*=\s*(\[(?:".+",\s+).*".+"\])$"#)
        .expect("Invalid TOML keywords regex pattern")
});

/// Multiline-pattern regex to match the identifier line of `Toml` front matter.
/// Contains a single capture group to extract the `Identifier` value.
static REGEX_FRONTMATTER_TOML_IDENTIFIER: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?m)^identifier\s*=\s*("\d{8}T\d{6}")$"#)
        .expect("Invalid TOML identifier regex pattern")
});

/// Multiline-pattern regex to match the title line of `Org` front matter.
/// Contains a single capture group to extract the `Title` value.
static REGEX_FRONTMATTER_ORG_TITLE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)^#\+title\s*:\s+(.+)$").expect("Invalid Org title regex pattern")
});

/// Multiline-pattern regex to match the date line of `Org` front matter.
/// Contains a single capture group to extract the `DateTime` value.
static REGEX_FRONTMATTER_ORG_DATETIME: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^#\+date\s*:\s+(.+)$").expect("Invalid Org date regex pattern"));

/// Multiline-pattern regex to match the filetags line of `Org` front matter.
/// Contains a single capture group to extract the `Keywords` value.
static REGEX_FRONTMATTER_ORG_KEYWORDS: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)^#\+filetags\s*:\s+((?::\S+)+:)$").expect("Invalid Org keywords regex pattern")
});

/// Multiline-pattern regex to match the identifier line of `Org` front matter.
/// Contains a single capture group to extract the `Identifier` value.
static REGEX_FRONTMATTER_ORG_IDENTIFIER: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)^#\+identifier\s*:\s+(\d{8}T\d{6})$")
        .expect("Invalid Org identifier regex pattern")
});

/// Attempts to partition a `&str` into a front matter prefix and a content suffix.
/// Does so by separating at the first blank line and checking for valid frontmatter
/// format. In the event that this fails, it will return its argument in the second
/// position of the return tuple, cast as an `Option<String>`.
pub fn separate_existing_content(input_content: &str) -> (Option<String>, Option<String>) {
    if input_content.is_empty() {
        (None, None)
    } else {
        input_content
            .split_once("\n\n")
            .or_else(|| input_content.split_once("\r\n\r\n"))
            .map_or_else(
                || (None, Some(input_content.to_owned())),
                |(prefix, suffix)| {
                    let (filename, content) = if is_valid_frontmatter_format(prefix) {
                        (
                            Some(prefix.to_owned()),
                            (!suffix.is_empty()).then(|| suffix.to_owned()),
                        )
                    } else {
                        (None, Some(input_content.to_owned()))
                    };

                    (filename, content)
                },
            )
    }
}

/// Checks that a `&str` conforms to one of the valid dn front matter formats -
/// `Text`, `Yaml`, `Toml`, or `Org`
fn is_valid_frontmatter_format(content: &str) -> bool {
    const ORG_SEGMENT_PREFIX: &str = "#+";

    let lines = content.lines().collect::<Vec<_>>();
    let first = lines.first();
    let last = lines.last();

    let is_text = last.map_or(false, |l| REGEX_FRONTMATTER_TEXT_RULE.is_match(l));
    let is_yaml = first.map_or(false, |l| REGEX_FRONTMATTER_YAML_CONTAINER.is_match(l))
        && last.map_or(false, |l| REGEX_FRONTMATTER_YAML_CONTAINER.is_match(l));
    let is_toml = first.map_or(false, |l| REGEX_FRONTMATTER_TOML_CONTAINER.is_match(l))
        && last.map_or(false, |l| REGEX_FRONTMATTER_TOML_CONTAINER.is_match(l));
    let is_org = !lines.is_empty() && lines.iter().all(|l| l.starts_with(ORG_SEGMENT_PREFIX));

    is_text || is_yaml || is_toml || is_org
}

#[cfg(test)]
mod tests {
    use super::*;

    mod content_partitioning {
        use crate::format::separate_existing_content;

        #[test]
        fn separates_frontmatter_only() {
            // Arrange
            let input = "---\nidentifier: \"20241212T121212\"\n---\n\n";
            let expected = (
                Some("---\nidentifier: \"20241212T121212\"\n---".to_owned()),
                None,
            );

            // Act
            let result = separate_existing_content(input);

            // Assert
            assert_eq!(
                expected, result,
                "\nInput: {input:#?}\nExpected frontmatter and content: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn separates_content_only() {
            // Arrange
            let input = "# Example Markdown\n\nFirst paragraph here.\n";
            let expected = (
                None,
                Some("# Example Markdown\n\nFirst paragraph here.\n".to_owned()),
            );

            // Act
            let result = separate_existing_content(input);

            // Assert
            assert_eq!(
                expected, result,
                "\nInput: {input:#?}\nExpected frontmatter and content: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn separates_both_frontmatter_and_content() {
            // Arrange
            let input = "+++\ntitle =  \"This is a Test!\"\n+++\n\n# Example Markdown\n\nFirst paragraph here.\n";
            let expected = (
                Some("+++\ntitle =  \"This is a Test!\"\n+++".to_owned()),
                Some("# Example Markdown\n\nFirst paragraph here.\n".to_owned()),
            );

            // Act
            let result = separate_existing_content(input);

            // Assert
            assert_eq!(
                expected, result,
                "\nInput: {input:#?}\nExpected frontmatter and content: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn separates_empty_content_into_none() {
            // Arrange
            let input = "";
            let expected = (None, None);

            // Act
            let result = separate_existing_content(input);

            // Assert
            assert_eq!(
                expected, result,
                "\nInput: {input:#?}\nExpected frontmatter and content: {expected:#?}\nReceived: {result:#?}"
            );
        }
    }

    mod frontmatter_validity {
        use crate::format::is_valid_frontmatter_format;

        #[test]
        fn determines_valid_text_frontmatter() {
            // Arrange
            let input = "title:foo\nidentifier : 20241212T121212\n---------------------------";
            let expected = true;

            // Act
            let result = is_valid_frontmatter_format(input);

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
            let expected = true;

            // Act
            let result = is_valid_frontmatter_format(input);

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
            let expected = true;

            // Act
            let result = is_valid_frontmatter_format(input);

            // Assert
            assert_eq!(
                expected, result,
                "\nInput: {input:#?}\nExpected: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn determines_valid_org_frontmatter() {
            // Arrange
            let input = "#+title: Test Title!\n#+date: [2024-12-12@12:12:12+02:00]";
            let expected = true;

            // Act
            let result = is_valid_frontmatter_format(input);

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
            let expected = false;

            // Act
            let result = is_valid_frontmatter_format(input);

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
            assert!(REGEX_FRONTMATTER_TEXT_RULE.is_match("---------------------------\n")); // 27 dashes
            assert!(REGEX_FRONTMATTER_TEXT_RULE.is_match("---------------------------")); // 27 dashes
            assert!(!REGEX_FRONTMATTER_TEXT_RULE.is_match("--------------------------\n")); // 26 dashes
            assert!(!REGEX_FRONTMATTER_TEXT_RULE.is_match("----------------------------\n"));
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
        fn date_parses_correctly() {
            // Arrange
            let input = "date  :  2024-12-12 @ 12:12:12\n";
            let expected = "2024-12-12 @ 12:12:12";

            // Act
            let result = REGEX_FRONTMATTER_TEXT_DATETIME
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
        fn test_container() {
            assert!(REGEX_FRONTMATTER_YAML_CONTAINER.is_match("---"));
            assert!(!REGEX_FRONTMATTER_YAML_CONTAINER.is_match("--"));
            assert!(!REGEX_FRONTMATTER_YAML_CONTAINER.is_match("----"));
        }

        #[test]
        fn test_title() {
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
        fn test_date() {
            // Arrange
            let input = "date :  2024-12-12T12:12:12\n";
            let expected = "2024-12-12T12:12:12";

            // Act
            let result = REGEX_FRONTMATTER_YAML_DATETIME
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
        fn test_keywords() {
            // Arrange
            let input = "tags:   [\"foo\",    \"bar\", \"baz\",  \"boom\"]\n";
            let expected = "[\"foo\",    \"bar\", \"baz\",  \"boom\"]";

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
        fn test_identifier() {
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
        fn test_container() {
            assert!(REGEX_FRONTMATTER_TOML_CONTAINER.is_match("+++"));
            assert!(!REGEX_FRONTMATTER_TOML_CONTAINER.is_match("++"));
            assert!(!REGEX_FRONTMATTER_TOML_CONTAINER.is_match("++++"));
        }

        #[test]
        fn test_title() {
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
        fn test_date() {
            // Arrange
            let input = "date  = 2024-12-12_12:12:12\n";
            let expected = "2024-12-12_12:12:12";

            // Act
            let result = REGEX_FRONTMATTER_TOML_DATETIME
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
        fn test_keywords() {
            // Arrange
            let input = "tags =   [\"foo\",    \"bar\", \"baz\",  \"boom\"]\n";
            let expected = "[\"foo\",    \"bar\", \"baz\",  \"boom\"]";

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
        fn test_identifier() {
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

    mod org_frontmatter {
        use super::*;

        #[test]
        fn test_title() {
            // Arrange
            let input = "#+title:   my-Test_ A N3w title\n";
            let expected = "my-Test_ A N3w title";

            // Act
            let result = REGEX_FRONTMATTER_ORG_TITLE
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
        fn test_date() {
            // Arrange
            let input = "#+date:  [2024-12-12 Thu 12:12]\n";
            let expected = "[2024-12-12 Thu 12:12]";

            // Act
            let result = REGEX_FRONTMATTER_ORG_DATETIME
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
        fn test_keywords() {
            // Arrange
            let input = "#+filetags:   :foo:bar:baz:boom:\n";
            let expected = ":foo:bar:baz:boom:";

            // Act
            let result = REGEX_FRONTMATTER_ORG_KEYWORDS
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
        fn test_identifier() {
            // Arrange
            let input = "#+identifier:   20241212T121212\n";
            let expected = "20241212T121212";

            // Act
            let result = REGEX_FRONTMATTER_ORG_IDENTIFIER
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
