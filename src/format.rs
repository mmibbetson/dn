pub const DN_IDENTIFIER_FORMAT: &str = "%Y%m%dT%H%M%S";

const TEXT_FRONTMATTER_RULE: &str = r#"\n---------------------------\n"#; // there should be exactly 27 '-' characters
const TEXT_FRONTMATTER_TITLE: &str = r#"title:\s+(.+)\n|(?:\r\n)"#;
const TEXT_FRONTMATTER_DATE: &str = r#"date:\s+(.+)\n|(?:\r\n)"#; // will also unfortunately match the org and yaml dates
const TEXT_FRONTMATTER_KEYWORDS: &str = r#"tags:\s+((?:\S+\s+)*\S+)\n|(?:\r\n)"#; // will also unfortunately match the org filetags
const TEXT_FRONTMATTER_IDENTIFIER: &str = r#"identifier:\s+(\d{8}T\d{6})\n|(?:\r\n)"#; // will also unfortunately match the org ident

const YAML_FRONTMATTER_CONTAINER: &str = r#"\n---\n"#;
const YAML_FRONTMATTER_TITLE: &str = r#"title:\s+(".+")\n|(?:\r\n)"#;
const YAML_FRONTMATTER_DATE: &str = r#"date:\s+(.+)\n|(?:\r\n)"#; // will also unfortunately match the org and text dates
const YAML_FRONTMATTER_KEYWORDS: &str = r#"tags:\s+(\[(?:".+",\s).*".+"\])\n|(?:\r\n)"#;
const YAML_FRONTMATTER_IDENTIFIER: &str = r#"identifier:\s+("\d{8}T\d{6}")\n|(?:\r\n)"#;

const TOML_FRONTMATTER_CONTAINER: &str = r#"\n+++\n"#;
const TOML_FRONTMATTER_TITLE: &str = r#"title\s+=\s+(".+")\n|(?:\r\n)"#;
const TOML_FRONTMATTER_DATE: &str = r#"date\s+=\s+(.+)\n|(?:\r\n)"#;
const TOML_FRONTMATTER_KEYWORDS: &str = r#"tags\s+=\s+(\[(?:".+",\s).*".+"\])\n|(?:\r\n)"#;
const TOML_FRONTMATTER_IDENTIFIER: &str = r#"identifier\s=\s+("\d{8}T\d{6}")\n|(?:\r\n)"#;

const ORG_FRONTMATTER_TITLE: &str = r#"#\+title:\s+(.+)\n|(?:\r\n)"#;
const ORG_FRONTMATTER_DATE: &str = r#"#\+date:\s+(.+)\n|(?:\r\n)"#; // matches any time format
const ORG_FRONTMATTER_KEYWORDS: &str = r#"#\+filetags:\s+((?::\S+)+:)\n|(?:\r\n)"#;
const ORG_FRONTMATTER_IDENTIFIER: &str = r#"#\+identifier:\s+(\d{8}T\d{6})\n|(?:\r\n)"#;

/// TODO
pub fn get_frontmatter_section(content: &str) -> Option<String> {
    content
        .split_once("\n\n")
        .or_else(|| content.split_once("\r\n\r\n"))
        .and_then(|(p, _)| {
            if is_valid_frontmatter_format(p) {
                Some(p.to_string())
            } else {
                None
            }
        })
}

/// TODO
fn is_valid_frontmatter_format(content: &str) -> bool {
    let lines = content.lines().collect::<Vec<_>>();
    let first = lines.first();
    let last = lines.last();

    let is_text = last == Some(&TEXT_FRONTMATTER_RULE);
    let is_yaml =
        first == Some(&YAML_FRONTMATTER_CONTAINER) && last == Some(&YAML_FRONTMATTER_CONTAINER);
    let is_toml =
        first == Some(&TOML_FRONTMATTER_CONTAINER) && last == Some(&TOML_FRONTMATTER_CONTAINER);
    let is_org = !lines.is_empty() && lines.iter().all(|l| l.starts_with("#+"));

    is_text || is_yaml || is_toml || is_org
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use super::*;

    mod text_frontmatter {
        use regex::Regex;

        use super::*;

        #[test]
        fn test_rule() {
            let re = Regex::new(TEXT_FRONTMATTER_RULE).unwrap();
            assert!(re.is_match("\n---------------------------\n")); // 27 dashes
            assert!(!re.is_match("---------------------------")); // 27 dashes
            assert!(!re.is_match("\n--------------------------\n")); // 26 dashes
            assert!(!re.is_match("\n----------------------------\n")); // 28 dashes
        }

        #[test]
        fn test_title() {
            // Arrange
            let re = Regex::new(TEXT_FRONTMATTER_TITLE).unwrap();
            let input = "title:   my-Test : A N3w title\n";
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
            let re = Regex::new(TEXT_FRONTMATTER_DATE).unwrap();
            let input = "date:  2024-12-12 @ 12:12:12\n";
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
            let re = Regex::new(TEXT_FRONTMATTER_KEYWORDS).unwrap();
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
            let re = Regex::new(TEXT_FRONTMATTER_IDENTIFIER).unwrap();
            let input = "identifier:   20241212T121212\n";
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
            let re = Regex::new(YAML_FRONTMATTER_CONTAINER).unwrap();
            assert!(re.is_match("\n---\n"));
            assert!(!re.is_match("\n--\n"));
            assert!(!re.is_match("\n----\n"));
        }

        #[test]
        fn test_title() {
            // Arrange
            let input = todo!();
            let expected = todo!();

            // Act
            let result = todo!();

            // Assert
            assert_eq!(
                expected, result,
                "Input: {input:#?}\nExpected match: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn test_date() {
            // Arrange
            let input = todo!();
            let expected = todo!();

            // Act
            let result = todo!();

            // Assert
            assert_eq!(
                expected, result,
                "Input: {input:#?}\nExpected match: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn test_keywords() {
            // Arrange
            let re = Regex::new(YAML_FRONTMATTER_KEYWORDS).unwrap();
            let input = "tags:   [\"foo\", \"bar\"]\n";
            let expected = "[\"foo\", \"bar\"]";

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
            let input = todo!();
            let expected = todo!();

            // Act
            let result = todo!();

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
            let re = Regex::new(TOML_FRONTMATTER_CONTAINER).unwrap();
            assert!(re.is_match("\n+++\n"));
            assert!(!re.is_match("\n++\n"));
            assert!(!re.is_match("\n++++\n"));
        }

        #[test]
        fn test_title() {
            // Arrange
            let input = todo!();
            let expected = todo!();

            // Act
            let result = todo!();

            // Assert
            assert_eq!(
                expected, result,
                "Input: {input:#?}\nExpected match: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn test_date() {
            // Arrange
            let input = todo!();
            let expected = todo!();

            // Act
            let result = todo!();

            // Assert
            assert_eq!(
                expected, result,
                "Input: {input:#?}\nExpected match: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn test_keywords() {
            // Arrange
            let input = todo!();
            let expected = todo!();

            // Act
            let result = todo!();

            // Assert
            assert_eq!(
                expected, result,
                "Input: {input:#?}\nExpected match: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn test_identifier() {
            // Arrange
            let input = todo!();
            let expected = todo!();

            // Act
            let result = todo!();

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
            let input = todo!();
            let expected = todo!();

            // Act
            let result = todo!();

            // Assert
            assert_eq!(
                expected, result,
                "Input: {input:#?}\nExpected match: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn test_date() {
            // Arrange
            let input = todo!();
            let expected = todo!();

            // Act
            let result = todo!();

            // Assert
            assert_eq!(
                expected, result,
                "Input: {input:#?}\nExpected match: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn test_keywords() {
            // Arrange
            let input = todo!();
            let expected = todo!();

            // Act
            let result = todo!();

            // Assert
            assert_eq!(
                expected, result,
                "Input: {input:#?}\nExpected match: {expected:#?}\nReceived: {result:#?}"
            );
        }

        #[test]
        fn test_identifier() {
            // Arrange
            let input = todo!();
            let expected = todo!();

            // Act
            let result = todo!();

            // Assert
            assert_eq!(
                expected, result,
                "Input: {input:#?}\nExpected match: {expected:#?}\nReceived: {result:#?}"
            );
        }
    }
}
