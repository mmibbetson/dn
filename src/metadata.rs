use chrono::{DateTime, Local};

use crate::{config::FileConfig, format::DN_IDENTIFIER_FORMAT};

pub struct FileMetadata {
    identifier: String,
    signature: Option<String>,
    title: Option<String>,
    title_raw: Option<String>,
    keywords: Option<Vec<String>>,
    extension: String,
    datetime: DateTime<Local>,
}

pub fn get_metadata(
    instance_time: &DateTime<Local>,
    identifier_arg: &Option<String>,
    signature_arg: &Option<String>,
    title_arg: &Option<String>,
    keywords_arg: &Option<String>,
    extension_arg: &Option<String>,
    config: &FileConfig,
) -> FileMetadata {
    let identifier = derive_identifier(instance_time, identifier_arg);

    let signature = signature_arg
        .as_ref()
        .and_then(|sig| parse_signature(&sig, &config.illegal_characters));

    let title_raw = title_arg.as_ref().map(String::from);
    let title = title_arg
        .as_ref()
        .and_then(|ttl| parse_title(&ttl, &config.illegal_characters));

    let keywords = keywords_arg
        .as_ref()
        .and_then(|key| parse_keywords(&key, &config.illegal_characters));

    let extension = extension_arg
        .as_ref()
        .and_then(|ext| parse_extension(&ext, &config.illegal_characters))
        .unwrap_or(config.default_extension.clone());

    let datetime = *instance_time;

    FileMetadata {
        identifier,
        signature,
        title,
        title_raw,
        keywords,
        extension,
        datetime,
    }
}

fn derive_identifier(instance_time: &DateTime<Local>, identifier_arg: &Option<String>) -> String {
    match identifier_arg {
        Some(id) => id.to_string(),
        None => instance_time.format(DN_IDENTIFIER_FORMAT).to_string(),
    }
}

fn parse_signature(signature_arg: &str, illegal_characters: &[char]) -> Option<String> {
    let out = signature_arg
        .to_lowercase()
        .chars()
        .filter(|c| !SEGMENT_SEPARATORS.contains(&c))
        .filter(|c| !illegal_characters.contains(&c))
        .collect::<String>();

    // NOTE: (!out.is_empty()).then_some(out) is equivalent; pretty cool.
    match out.is_empty() {
        true => None,
        false => Some(out),
    }
}

fn parse_title(title_arg: &str, illegal_characters: &[char]) -> Option<String> {
    let out = title_arg
        .to_lowercase()
        .split(['-', ' '])
        .map(|w| sanitise(w, illegal_characters))
        .filter(|w| !w.is_empty())
        .collect::<Vec<_>>()
        .join("-");

    match out.is_empty() {
        true => None,
        false => Some(out),
    }
}

fn parse_keywords(keywords_arg: &str, illegal_characters: &[char]) -> Option<Vec<String>> {
    let out = keywords_arg
        .to_lowercase()
        .split(['_', ' '])
        .map(|w| sanitise(w, illegal_characters))
        .filter(|w| !w.is_empty())
        .collect::<Vec<_>>();

    match out.is_empty() {
        true => None,
        false => Some(out),
    }
}

fn parse_extension(extension_arg: &str, illegal_characters: &[char]) -> Option<String> {
    let out = extension_arg
        .to_lowercase()
        .split(['.', ' '])
        .map(|w| sanitise(w, illegal_characters))
        .filter(|w| !w.is_empty())
        .collect::<Vec<_>>()
        .join(".");

    match out.is_empty() {
        true => None,
        false => Some(out),
    }
}

fn sanitise(dirty: &str, illegal_characters: &[char]) -> String {
    dirty
        .chars()
        .filter(|&c| !SEGMENT_SEPARATORS.contains(&c))
        .filter(|&c| !illegal_characters.contains(&c))
        .collect::<String>()
}

const SEGMENT_SEPARATORS: [char; 4] = ['=', '-', '_', '.'];

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn setup_config() -> FileConfig {
        FileConfig {
            illegal_characters: vec!['#', '@', '!'],
            default_extension: String::from("md"),
            ..Default::default()
        }
    }

    fn setup_datetime() -> DateTime<Local> {
        Local.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap()
    }

    #[test]
    fn derive_identifier_with_no_identifier_arg() {
        // Arrange
        let time = setup_datetime();
        let expected = "20240101T120000";

        // Act
        let result = derive_identifier(&time, &None);

        // Assert
        assert_eq!(
            result, expected,
            "Expected identifier {:?} but got {:?}",
            expected, result
        );
    }

    #[test]
    fn derive_identifier_with_existing_identifier_arg() {
        // Arrange
        let time = setup_datetime();
        let identifier = Some("20241212T121212".to_string());
        let expected = "20241212T121212";

        // Act
        let result = derive_identifier(&time, &identifier);

        // Assert
        assert_eq!(
            result, expected,
            "Expected identifier {:?} but got {:?}",
            expected, result
        );
    }

    #[test]
    fn parse_signature_with_multiple_parts() {
        // Arrange
        let config = setup_config();
        let input = "Test=Signature!";
        let expected = Some("testsignature".to_string());

        // Act
        let result = parse_signature(input, &config.illegal_characters);

        // Assert
        assert_eq!(
            result, expected,
            "Input: {:?}\nExpected: {:?}\nGot: {:?}",
            input, expected, result
        );
    }

    #[test]
    fn parse_signature_with_illegal_chars() {
        // Arrange
        let config = setup_config();
        let input = "Test@Signature!-";
        let expected = Some("testsignature".to_string());

        // Act
        let result = parse_signature(input, &config.illegal_characters);

        // Assert
        assert_eq!(
            result, expected,
            "Input: {:?}\nExpected: {:?}\nGot: {:?}",
            input, expected, result
        );
    }

    #[test]
    fn parse_signature_with_only_illegal_chars() {
        // Arrange
        let config = setup_config();
        let input = "@!#";
        let expected = None;

        // Act
        let result = parse_signature(input, &config.illegal_characters);

        // Assert
        assert_eq!(
            result, expected,
            "Input: {:?}\nExpected: {:?}\nGot: {:?}",
            input, expected, result
        );
    }

    #[test]
    fn parse_title_with_multiple_parts() {
        // Arrange
        let config = setup_config();
        let input = "My-Cool Title";
        let expected = Some("my-cool-title".to_string());

        // Act
        let result = parse_title(input, &config.illegal_characters);

        // Assert
        assert_eq!(
            result, expected,
            "Input: {:?}\nExpected: {:?}\nGot: {:?}",
            input, expected, result
        );
    }

    #[test]
    fn parse_keywords_with_multiple_parts() {
        // Arrange
        let config = setup_config();
        let input = "_rust_programming test";
        let expected = Some(vec![
            "rust".to_string(),
            "programming".to_string(),
            "test".to_string(),
        ]);

        // Act
        let result = parse_keywords(input, &config.illegal_characters);

        // Assert
        assert_eq!(
            result, expected,
            "Input: {:?}\nExpected keywords: {:?}\nGot: {:?}",
            input, expected, result
        );
    }

    #[test]
    fn parse_extension_with_multiple_parts() {
        // Arrange
        let config = setup_config();
        let input = ".tar.gz";
        let expected = Some("tar.gz".to_string());

        // Act
        let result = parse_extension(input, &config.illegal_characters);

        // Assert
        assert_eq!(
            result, expected,
            "Input: {:?}\nExpected extension: {:?}\nGot: {:?}",
            input, expected, result
        );
    }

    #[test]
    fn parse_extension_with_uppercase() {
        // Arrange
        let config = setup_config();
        let input = "MD";
        let expected = Some("md".to_string());

        // Act
        let result = parse_extension(input, &config.illegal_characters);

        // Assert
        assert_eq!(
            result, expected,
            "Input: {:?}\nExpected extension: {:?}\nGot: {:?}",
            input, expected, result
        );
    }

    #[test]
    fn get_metadata_full_integration() {
        // Arrange
        let config = setup_config();
        let time = setup_datetime();
        let inputs = (
            &None,
            &Some("test@signature".to_string()),
            &Some("My Cool Title!".to_string()),
            &Some("rust programming".to_string()),
            &Some("RS".to_string()),
        );
        let expected = FileMetadata {
            identifier: "20240101T120000".to_string(),
            signature: Some("testsignature".to_string()),
            title: Some("my-cool-title".to_string()),
            title_raw: Some("My Cool Title!".to_string()),
            keywords: Some(vec!["rust".to_string(), "programming".to_string()]),
            extension: "rs".to_string(),
            datetime: time,
        };

        // Act
        let result = get_metadata(
            &time, inputs.0, inputs.1, inputs.2, inputs.3, inputs.4, &config,
        );

        // Assert
        assert_eq!(
            result.identifier, expected.identifier,
            "Identifier mismatch:\nExpected: {:?}\nGot: {:?}",
            expected.identifier, result.identifier
        );
        assert_eq!(
            result.signature, expected.signature,
            "Signature mismatch:\nExpected: {:?}\nGot: {:?}",
            expected.signature, result.signature
        );
        assert_eq!(
            result.title, expected.title,
            "Title mismatch:\nExpected: {:?}\nGot: {:?}",
            expected.title, result.title
        );
        assert_eq!(
            result.keywords, expected.keywords,
            "Keywords mismatch:\nExpected: {:?}\nGot: {:?}",
            expected.keywords, result.keywords
        );
        assert_eq!(
            result.extension, expected.extension,
            "Extension mismatch:\nExpected: {:?}\nGot: {:?}",
            expected.extension, result.extension
        );
        assert_eq!(
            result.datetime, expected.datetime,
            "Datetime mismatch:\nExpected: {:?}\nGot: {:?}",
            expected.datetime, result.datetime
        );
    }

    #[test]
    fn sanitise_with_illegal_chars() {
        // Arrange
        let config = setup_config();
        let input = "hello@world!";
        let expected = "helloworld".to_string();

        // Act
        let result = sanitise(input, &config.illegal_characters);

        // Assert
        assert_eq!(
            result, expected,
            "Input: {:?}\nExpected sanitized string: {:?}\nGot: {:?}",
            input, expected, result
        );
    }
}
