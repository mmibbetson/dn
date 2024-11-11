/// TODO
pub const DN_IDENTIFIER_FORMAT: &str = "%Y%m%dT%H%M%S";

/// TODO
const PATTERN_TEXT_FRONTMATTER_RULE: &str = r#"(?m)^---------------------------$"#; // there should be exactly 27 '-' characters
/// TODO
const PATTERN_TEXT_FRONTMATTER_TITLE: &str = r#"(?m)^title\s*:\s*(.+)$"#;
/// TODO
const PATTERN_TEXT_FRONTMATTER_DATE: &str = r#"(?m)^date\s*:\s*(.+)$"#; // will also unfortunately match the org and yaml dates
/// TODO
const PATTERN_TEXT_FRONTMATTER_KEYWORDS: &str = r#"(?m)^tags\s*:\s*((?:\S+\s+)*\S+)$"#; // will also unfortunately match the org filetags
/// TODO
const PATTERN_TEXT_FRONTMATTER_IDENTIFIER: &str = r#"(?m)^identifier\s*:\s*(\d{8}T\d{6})$"#; // will also unfortunately match the org ident

/// TODO
const PATTERN_YAML_FRONTMATTER_CONTAINER: &str = r#"(?m)^---$"#;
/// TODO
const PATTERN_YAML_FRONTMATTER_TITLE: &str = r#"(?m)^title\s*:\s+(".+")$"#;
/// TODO
const PATTERN_YAML_FRONTMATTER_DATE: &str = r#"(?m)^date\s*:\s+(.+)$"#; // will also unfortunately match the org and text dates
/// TODO
const PATTERN_YAML_FRONTMATTER_KEYWORDS: &str = r#"(?m)^tags\s*:\s+(\[(?:".+",\s+).*".+"\])$"#;
/// TODO
const PATTERN_YAML_FRONTMATTER_IDENTIFIER: &str = r#"(?m)^identifier\s*:\s+("\d{8}T\d{6}")$"#;

/// TODO
const PATTERN_TOML_FRONTMATTER_CONTAINER: &str = r#"(?m)^\+\+\+$"#;
/// TODO
const PATTERN_TOML_FRONTMATTER_TITLE: &str = r#"(?m)^title\s*=\s*(".+")$"#;
/// TODO
const PATTERN_TOML_FRONTMATTER_DATE: &str = r#"(?m)^date\s*=\s*(.+)$"#;
/// TODO
const PATTERN_TOML_FRONTMATTER_KEYWORDS: &str = r#"(?m)^tags\s*=\s*(\[(?:".+",\s+).*".+"\])$"#;
/// TODO
const PATTERN_TOML_FRONTMATTER_IDENTIFIER: &str = r#"(?m)^identifier\s*=\s*("\d{8}T\d{6}")$"#;

/// TODO
const ORG_SEGMENT_PREFIX: &str = r#"#+"#;
/// TODO
const PATTERN_ORG_FRONTMATTER_TITLE: &str = r#"(?m)^#\+title\s*:\s+(.+)$"#;
/// TODO
const PATTERN_ORG_FRONTMATTER_DATE: &str = r#"(?m)^#\+date\s*:\s+(.+)$"#; // matches any time format
/// TODO
const PATTERN_ORG_FRONTMATTER_KEYWORDS: &str = r#"(?m)^#\+filetags\s*:\s+((?::\S+)+:)$"#;
/// TODO
const PATTERN_ORG_FRONTMATTER_IDENTIFIER: &str = r#"(?m)^#\+identifier\s*:\s+(\d{8}T\d{6})$"#;

/// TODO
pub fn separate_existing_content(content: &str) -> (Option<String>, Option<String>) {
    let split = content
        .split_once("\n\n")
        .or_else(|| content.split_once("\r\n\r\n"));

    let frontmatter = split.and_then(|(p, _)| {
        if is_valid_frontmatter_format(p) {
            Some(p.to_string())
        } else {
            None
        }
    });

    let content = split.map(|(_, s)| s.to_string());

    (frontmatter, content)
}

/// TODO
fn is_valid_frontmatter_format(content: &str) -> bool {
    let lines = content.lines().collect::<Vec<_>>();
    let first = lines.first();
    let last = lines.last();

    let is_text = last == Some(&PATTERN_TEXT_FRONTMATTER_RULE);
    let is_yaml = first == Some(&PATTERN_YAML_FRONTMATTER_CONTAINER)
        && last == Some(&PATTERN_YAML_FRONTMATTER_CONTAINER);
    let is_toml = first == Some(&PATTERN_TOML_FRONTMATTER_CONTAINER)
        && last == Some(&PATTERN_TOML_FRONTMATTER_CONTAINER);
    let is_org = !lines.is_empty() && lines.iter().all(|l| l.starts_with(&ORG_SEGMENT_PREFIX));

    is_text || is_yaml || is_toml || is_org
}

// TODO: Fix failing yaml/toml tests
#[cfg(test)]
mod tests {
    use regex::Regex;

    use super::*;

    mod text_frontmatter {
        use regex::Regex;

        use super::*;

        #[test]
        fn test_rule() {
            let re = Regex::new(PATTERN_TEXT_FRONTMATTER_RULE).unwrap();
            assert!(re.is_match("---------------------------\n")); // 27 dashes
            assert!(re.is_match("---------------------------")); // 27 dashes
            assert!(!re.is_match("--------------------------\n")); // 26 dashes
            assert!(!re.is_match("----------------------------\n")); // 28 dashes
        }

        #[test]
        fn test_title() {
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
        fn test_date() {
            // Arrange
            let re = Regex::new(PATTERN_TEXT_FRONTMATTER_DATE).unwrap();
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
        fn test_keywords() {
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
        fn test_identifier() {
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
            let re = Regex::new(PATTERN_ORG_FRONTMATTER_DATE).unwrap();
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
