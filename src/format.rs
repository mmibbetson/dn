/// Format string for use with `chrono`'s `format` function.
/// Represents the structure of a dn `Identifier`.
pub const DN_IDENTIFIER_FORMAT: &str = "%Y%m%dT%H%M%S";

/// Multiline pattern to match the closing horizontal rule of `Text` front matter,
/// which should be exactly 27 '-' characters.
const PATTERN_TEXT_FRONTMATTER_RULE: &str = r#"(?m)^---------------------------$"#;

/// Multiline pattern to match the title line of `Text` front matter.
/// Contains a single capture group to extract the `Title` value.
///
/// ## Warning
///
/// May also improperly match some `Org` and `Yaml` titles if present.
const PATTERN_TEXT_FRONTMATTER_TITLE: &str = r#"(?m)^title\s*:\s*(.+)$"#;

/// Multiline pattern to match the date line of `Text` front matter.
/// Contains a single capture group to extract the `DateTime` value.
///
/// ## Warning
///
/// May also improperly match some `Org` and `Yaml` dates if present.
const PATTERN_TEXT_FRONTMATTER_DATETIME: &str = r#"(?m)^date\s*:\s*(.+)$"#;

/// Multiline pattern to match the tags line of `Text` front matter.
/// Contains a single capture group to extract the `Keywords` value.
///
/// ## Warning
///
/// May also improperly match some `Org` keywords if present.
const PATTERN_TEXT_FRONTMATTER_KEYWORDS: &str = r#"(?m)^tags\s*:\s*((?:\S+\s+)*\S+)$"#;

/// Multiline pattern to match the identifier line of `Text` front matter.
/// Contains a single capture group to extract the `Identifier` value.
const PATTERN_TEXT_FRONTMATTER_IDENTIFIER: &str = r#"(?m)^identifier\s*:\s*(\d{8}T\d{6})$"#;

/// Multiline pattern to match the opening and closing markers of `Yaml` front matter.
const PATTERN_YAML_FRONTMATTER_CONTAINER: &str = r#"(?m)^---$"#;

/// Multiline pattern to match the title line of `Yaml` front matter.
/// Contains a single capture group to extract the `Title` value.
const PATTERN_YAML_FRONTMATTER_TITLE: &str = r#"(?m)^title\s*:\s+(".+")$"#;

/// Multiline pattern to match the date line of `Yaml` front matter.
/// Contains a single capture group to extract the `DateTime` value.
///
/// ## Warning
///
/// May also improperly match some `Text` and `Org` dates if present.
const PATTERN_YAML_FRONTMATTER_DATE: &str = r#"(?m)^date\s*:\s+(.+)$"#;

/// Multiline pattern to match the tags line of `Yaml` front matter.
/// Contains a single capture group to extract the `Keywords` value.
const PATTERN_YAML_FRONTMATTER_KEYWORDS: &str = r#"(?m)^tags\s*:\s+(\[(?:".+",\s+).*".+"\])$"#;

/// Multiline pattern to match the identifier line of `Yaml` front matter.
/// Contains a single capture group to extract the `Identifier` value.
const PATTERN_YAML_FRONTMATTER_IDENTIFIER: &str = r#"(?m)^identifier\s*:\s+("\d{8}T\d{6}")$"#;

/// Multiline pattern to match the opening and closing markers of `Toml` frontmatter.
const PATTERN_TOML_FRONTMATTER_CONTAINER: &str = r#"(?m)^\+\+\+$"#;

/// Multiline pattern to match the title line of `Toml` front matter.
/// Contains a single capture group to extract the `Title` value.
const PATTERN_TOML_FRONTMATTER_TITLE: &str = r#"(?m)^title\s*=\s*(".+")$"#;

/// Multiline pattern to match the date line of `Toml` front matter.
/// Contains a single capture group to extract the `DateTime` value.
const PATTERN_TOML_FRONTMATTER_DATE: &str = r#"(?m)^date\s*=\s*(.+)$"#;

/// Multiline pattern to match the tags line of `Toml` front matter.
/// Contains a single capture group to extract the `Keywords` value.
const PATTERN_TOML_FRONTMATTER_KEYWORDS: &str = r#"(?m)^tags\s*=\s*(\[(?:".+",\s+).*".+"\])$"#;

/// Multiline pattern to match the identifier line of `Toml` front matter.
/// Contains a single capture group to extract the `Identifier` value.
const PATTERN_TOML_FRONTMATTER_IDENTIFIER: &str = r#"(?m)^identifier\s*=\s*("\d{8}T\d{6}")$"#;

/// Multiline pattern to match the title line of `Org` front matter.
/// Contains a single capture group to extract the `Title` value.
const PATTERN_ORG_FRONTMATTER_TITLE: &str = r#"(?m)^#\+title\s*:\s+(.+)$"#;

/// Multiline pattern to match the date line of `Org` front matter.
/// Contains a single capture group to extract the `DateTime` value.
const PATTERN_ORG_FRONTMATTER_DATETIME: &str = r#"(?m)^#\+date\s*:\s+(.+)$"#;

/// Multiline pattern to match the filetags line of `Org` front matter.
/// Contains a single capture group to extract the `Keywords` value.
const PATTERN_ORG_FRONTMATTER_KEYWORDS: &str = r#"(?m)^#\+filetags\s*:\s+((?::\S+)+:)$"#;

/// Multiline pattern to match the identifier line of `Org` front matter.
/// Contains a single capture group to extract the `Identifier` value.
const PATTERN_ORG_FRONTMATTER_IDENTIFIER: &str = r#"(?m)^#\+identifier\s*:\s+(\d{8}T\d{6})$"#;

/// Attempts to partition a `&str` into a front matter prefix and a content suffix.
/// Does so by separating at the first blank line and checking for valid frontmatter
/// format. In the event that this fails, it will return its argument in the second
/// position of the return tuple, cast as an `Option<String>`.
pub fn separate_existing_content(content: &str) -> (Option<String>, Option<String>) {
    let split = content
        .split_once("\n\n")
        .or_else(|| content.split_once("\r\n\r\n"));

    match split {
        Some((prefix, suffix)) => {
            let frontmatter = if is_valid_frontmatter_format(prefix) {
                Some(prefix.to_string())
            } else {
                None
            };

            let content = Some(suffix.to_string());

            (frontmatter, content)
        }
        None => (None, Some(content.to_string())),
    }
}

/// Checks that a `&str` conforms to one of the valid dn front matter formats -
/// `Text`, `Yaml`, `Toml`, or `Org`
fn is_valid_frontmatter_format(content: &str) -> bool {
    const ORG_SEGMENT_PREFIX: &str = "#+";

    let lines = content.lines().collect::<Vec<_>>();
    let first = lines.first();
    let last = lines.last();

    let is_text = last == Some(&PATTERN_TEXT_FRONTMATTER_RULE);
    let is_yaml = first == Some(&PATTERN_YAML_FRONTMATTER_CONTAINER)
        && last == Some(&PATTERN_YAML_FRONTMATTER_CONTAINER);
    let is_toml = first == Some(&PATTERN_TOML_FRONTMATTER_CONTAINER)
        && last == Some(&PATTERN_TOML_FRONTMATTER_CONTAINER);
    let is_org = !lines.is_empty() && lines.iter().all(|l| l.starts_with(ORG_SEGMENT_PREFIX));

    is_text || is_yaml || is_toml || is_org
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use super::*;

    mod content_partitioning {
        #[test]
        fn separates_frontmatter_only() {
            // Arrange
            let input = todo!();
            let expected = todo!();

            // Act
            let result = todo!();

            // Assert
            assert_eq!(
                expected, result,
                "Input: {input:#?}\nExpected frontmatter and content: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn separates_content() {
            // Arrange
            let input = todo!();
            let expected = todo!();

            // Act
            let result = todo!();

            // Assert
            assert_eq!(
                expected, result,
                "Input: {input:#?}\nExpected frontmatter and content: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn separates_both_frontmatter_and_content() {
            // Arrange
            let input = todo!();
            let expected = todo!();

            // Act
            let result = todo!();

            // Assert
            assert_eq!(
                expected, result,
                "Input: {input:#?}\nExpected frontmatter and content: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn separates_neither_frontmatter_nor_content() {
            // Arrange
            let input = todo!();
            let expected = todo!();

            // Act
            let result = todo!();

            // Assert
            assert_eq!(
                expected, result,
                "Input: {input:#?}\nExpected frontmatter and content: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn determines_valid_text_frontmatter() {
            // Arrange
            let input = todo!();
            let expected = todo!();

            // Act
            let result = todo!();

            // Assert
            assert_eq!(
                expected, result,
                "Input: {input:#?}\nExpected: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn determines_valid_yaml_frontmatter() {
            // Arrange
            let input = todo!();
            let expected = todo!();

            // Act
            let result = todo!();

            // Assert
            assert_eq!(
                expected, result,
                "Input: {input:#?}\nExpected: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn determines_valid_toml_frontmatter() {
            // Arrange
            let input = todo!();
            let expected = todo!();

            // Act
            let result = todo!();

            // Assert
            assert_eq!(
                expected, result,
                "Input: {input:#?}\nExpected: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn determines_invalid_frontmatter() {
            // Arrange
            let input = todo!();
            let expected = todo!();

            // Act
            let result = todo!();

            // Assert
            assert_eq!(
                expected, result,
                "Input: {input:#?}\nExpected: {expected:#?}\nReceived: {result:#?}"
            );
        }
    }

    mod text_frontmatter {
        use regex::Regex;

        use super::*;

        #[test]
        fn rule_parses_correctly() {
            let re = Regex::new(PATTERN_TEXT_FRONTMATTER_RULE).unwrap();
            assert!(re.is_match("---------------------------\n")); // 27 dashes
            assert!(re.is_match("---------------------------")); // 27 dashes
            assert!(!re.is_match("--------------------------\n")); // 26 dashes
            assert!(!re.is_match("----------------------------\n")); // 28 dashes
        }

        #[test]
        fn title_parses_correctly() {
            // Arrange
            let re = Regex::new(PATTERN_TEXT_FRONTMATTER_TITLE).unwrap();
            let input = "title:my-Test : A N3w title\n";
            let expected = "my-Test : A N3w title";

            // Act
            let result = re.captures(input).unwrap().get(1).unwrap().as_str();

            // Assert
            assert_eq!(
                expected, result,
                "Input: {input:#?}\nExpected match: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn date_parses_correctly() {
            // Arrange
            let re = Regex::new(PATTERN_TEXT_FRONTMATTER_DATETIME).unwrap();
            let input = "date  :  2024-12-12 @ 12:12:12\n";
            let expected = "2024-12-12 @ 12:12:12";

            // Act
            let result = re.captures(input).unwrap().get(1).unwrap().as_str();

            // Assert
            assert_eq!(
                expected, result,
                "Input: {input:#?}\nExpected match: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn keywords_parse_correcly() {
            // Arrange
            let re = Regex::new(PATTERN_TEXT_FRONTMATTER_KEYWORDS).unwrap();
            let input = "tags:   foo    bar baz  boom\n";
            let expected = "foo    bar baz  boom";

            // Act
            let result = re.captures(input).unwrap().get(1).unwrap().as_str();

            // Assert
            assert_eq!(
                expected, result,
                "Input: {input:#?}\nExpected match: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn identifier_parses_correctly() {
            // Arrange
            let re = Regex::new(PATTERN_TEXT_FRONTMATTER_IDENTIFIER).unwrap();
            let input = "identifier: 20241212T121212\n";
            let expected = "20241212T121212";

            // Act
            let result = re.captures(input).unwrap().get(1).unwrap().as_str();

            // Assert
            assert_eq!(
                expected, result,
                "Input: {input:#?}\nExpected match: {expected:#?}\nReceived: {result:#?}"
            );
        }
    }

    mod yaml_frontmatter {
        use regex::Regex;

        use super::*;

        #[test]
        fn test_container() {
            let re = Regex::new(PATTERN_YAML_FRONTMATTER_CONTAINER).unwrap();
            assert!(re.is_match("---"));
            assert!(!re.is_match("--"));
            assert!(!re.is_match("----"));
        }

        #[test]
        fn test_title() {
            // Arrange
            let re = Regex::new(PATTERN_YAML_FRONTMATTER_TITLE).unwrap();
            let input = "title:   \"my-Test -  A N3w title\"\n";
            let expected = "\"my-Test -  A N3w title\"";

            // Act
            let result = re.captures(input).unwrap().get(1).unwrap().as_str();

            // Assert
            assert_eq!(
                expected, result,
                "Input: {input:#?}\nExpected match: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn test_date() {
            // Arrange
            let re = Regex::new(PATTERN_YAML_FRONTMATTER_DATE).unwrap();
            let input = "date :  2024-12-12T12:12:12\n";
            let expected = "2024-12-12T12:12:12";

            // Act
            let result = re.captures(input).unwrap().get(1).unwrap().as_str();

            // Assert
            assert_eq!(
                expected, result,
                "Input: {input:#?}\nExpected match: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn test_keywords() {
            // Arrange
            let re = Regex::new(PATTERN_YAML_FRONTMATTER_KEYWORDS).unwrap();
            let input = "tags:   [\"foo\",    \"bar\", \"baz\",  \"boom\"]\n";
            let expected = "[\"foo\",    \"bar\", \"baz\",  \"boom\"]";

            // Act
            let result = re.captures(input).unwrap().get(1).unwrap().as_str();

            // Assert
            assert_eq!(
                expected, result,
                "Input: {input:#?}\nExpected match: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn test_identifier() {
            // Arrange
            let re = Regex::new(PATTERN_YAML_FRONTMATTER_IDENTIFIER).unwrap();
            let input = "identifier: \"20241212T121212\"\n";
            let expected = "\"20241212T121212\"";

            // Act
            let result = re.captures(input).unwrap().get(1).unwrap().as_str();

            // Assert
            assert_eq!(
                expected, result,
                "Input: {input:#?}\nExpected match: {expected:#?}\nReceived: {result:#?}"
            );
        }
    }

    mod toml_frontmatter {
        use super::*;

        #[test]
        fn test_container() {
            let re = Regex::new(PATTERN_TOML_FRONTMATTER_CONTAINER).unwrap();
            assert!(re.is_match("+++"));
            assert!(!re.is_match("++"));
            assert!(!re.is_match("++++"));
        }

        #[test]
        fn test_title() {
            // Arrange
            let re = Regex::new(PATTERN_TOML_FRONTMATTER_TITLE).unwrap();
            let input = "title=\"my-Test : A N3w title\"\n";
            let expected = "\"my-Test : A N3w title\"";

            // Act
            let result = re.captures(input).unwrap().get(1).unwrap().as_str();

            // Assert
            assert_eq!(
                expected, result,
                "Input: {input:#?}\nExpected match: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn test_date() {
            // Arrange
            let re = Regex::new(PATTERN_TOML_FRONTMATTER_DATE).unwrap();
            let input = "date  = 2024-12-12_12:12:12\n";
            let expected = "2024-12-12_12:12:12";

            // Act
            let result = re.captures(input).unwrap().get(1).unwrap().as_str();

            // Assert
            assert_eq!(
                expected, result,
                "Input: {input:#?}\nExpected match: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn test_keywords() {
            // Arrange
            let re = Regex::new(PATTERN_TOML_FRONTMATTER_KEYWORDS).unwrap();
            let input = "tags =   [\"foo\",    \"bar\", \"baz\",  \"boom\"]\n";
            let expected = "[\"foo\",    \"bar\", \"baz\",  \"boom\"]";

            // Act
            let result = re.captures(input).unwrap().get(1).unwrap().as_str();

            // Assert
            assert_eq!(
                expected, result,
                "Input: {input:#?}\nExpected match: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn test_identifier() {
            // Arrange
            let re = Regex::new(PATTERN_TOML_FRONTMATTER_IDENTIFIER).unwrap();
            let input = "identifier =\"20241212T121212\"\n";
            let expected = "\"20241212T121212\"";

            // Act
            let result = re.captures(input).unwrap().get(1).unwrap().as_str();

            // Assert
            assert_eq!(
                expected, result,
                "Input: {input:#?}\nExpected match: {expected:#?}\nReceived: {result:#?}"
            );
        }
    }

    mod org_frontmatter {
        use super::*;

        #[test]
        fn test_title() {
            // Arrange
            let re = Regex::new(PATTERN_ORG_FRONTMATTER_TITLE).unwrap();
            let input = "#+title:   my-Test_ A N3w title\n";
            let expected = "my-Test_ A N3w title";

            // Act
            let result = re.captures(input).unwrap().get(1).unwrap().as_str();

            // Assert
            assert_eq!(
                expected, result,
                "Input: {input:#?}\nExpected match: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn test_date() {
            // Arrange
            let re = Regex::new(PATTERN_ORG_FRONTMATTER_DATETIME).unwrap();
            let input = "#+date:  [2024-12-12 Thu 12:12]\n";
            let expected = "[2024-12-12 Thu 12:12]";

            // Act
            let result = re.captures(input).unwrap().get(1).unwrap().as_str();

            // Assert
            assert_eq!(
                expected, result,
                "Input: {input:#?}\nExpected match: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn test_keywords() {
            // Arrange
            let re = Regex::new(PATTERN_ORG_FRONTMATTER_KEYWORDS).unwrap();
            let input = "#+filetags:   :foo:bar:baz:boom:\n";
            let expected = ":foo:bar:baz:boom:";

            // Act
            let result = re.captures(input).unwrap().get(1).unwrap().as_str();

            // Assert
            assert_eq!(
                expected, result,
                "Input: {input:#?}\nExpected match: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn test_identifier() {
            // Arrange
            let re = Regex::new(PATTERN_ORG_FRONTMATTER_IDENTIFIER).unwrap();
            let input = "#+identifier:   20241212T121212\n";
            let expected = "20241212T121212";

            // Act
            let result = re.captures(input).unwrap().get(1).unwrap().as_str();

            // Assert
            assert_eq!(
                expected, result,
                "Input: {input:#?}\nExpected match: {expected:#?}\nReceived: {result:#?}"
            );
        }
    }
}
