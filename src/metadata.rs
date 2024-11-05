use chrono::{DateTime, Local, NaiveDateTime, TimeZone};

use crate::{config::FileConfig, format::DN_IDENTIFIER_FORMAT};

const SEGMENT_SEPARATORS: [char; 4] = ['=', '-', '_', '.'];

#[derive(Debug, Default, Clone)]
pub struct FileMetadata {
    pub identifier: String,
    pub signature: Option<String>,
    pub title: Option<String>,
    pub title_raw: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub extension: String,
    pub datetime: DateTime<Local>,
}

#[derive(Debug, Default)]
pub struct FileMetadataBuilder {
    identifier: Option<String>,
    signature: Option<String>,
    title: Option<String>,
    keywords: Option<String>,
    extension: Option<String>,
    datetime: DateTime<Local>,
}

impl FileMetadata {
    pub fn builder() -> FileMetadataBuilder {
        FileMetadataBuilder::default()
    }
}

impl FileMetadataBuilder {
    pub fn with_identifier(mut self, value: &Option<String>) -> Self {
        self.identifier = value.clone();
        self
    }

    pub fn with_signature(mut self, value: &Option<String>) -> Self {
        self.signature = value.clone();
        self
    }

    pub fn with_title(mut self, value: &Option<String>) -> Self {
        self.title = value.clone();
        self
    }

    pub fn with_keywords(mut self, value: &Option<String>) -> Self {
        self.keywords = value.clone();
        self
    }

    pub fn with_extension(mut self, value: &Option<String>) -> Self {
        self.extension = value.clone();
        self
    }

    pub fn build(&self, config: &FileConfig) -> FileMetadata {
        let datetime = derive_datetime(&self.identifier);

        let identifier = derive_identifier(&self.datetime, &self.identifier);

        let signature = self
            .signature
            .as_ref()
            .and_then(|s| parse_signature(&s, &config.illegal_characters));

        let title = self
            .title
            .as_ref()
            .and_then(|t| parse_title(&t, &config.illegal_characters));
        let title_raw = self.title.as_ref().map(String::from);

        let keywords = self
            .keywords
            .as_ref()
            .and_then(|k| parse_keywords(&k, &config.illegal_characters));

        let extension = self
            .extension
            .as_ref()
            .and_then(|e| parse_extension(&e, &config.illegal_characters))
            .unwrap_or(config.default_extension.clone());

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

fn derive_datetime(identifier: &Option<String>) -> DateTime<Local> {
    match identifier {
        Some(id) => match NaiveDateTime::parse_from_str(id, DN_IDENTIFIER_FORMAT) {
            Ok(naive) => TimeZone::from_local_datetime(&Local, &naive)
                .single()
                .unwrap_or_else(Local::now),
            Err(_) => Local::now(),
        },
        None => Local::now(),
    }
}

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
        // WARN: Unwrap may panic.
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
            "Expected: {:?}\nReceived: {:?}",
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
            "Expected: {:?}\nReceived: {:?}",
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
            "Input: {:?}\nExpected: {:?}\nReceived: {:?}",
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
            "Input: {:?}\nExpected: {:?}\nReceived: {:?}",
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
            "Input: {:?}\nExpected: {:?}\nReceived: {:?}",
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
            "Input: {:?}\nExpected: {:?}\nReceived: {:?}",
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
            "Input: {:?}\nExpected keywords: {:?}\nReceived: {:?}",
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
            "Input: {:?}\nExpected extension: {:?}\nReceived: {:?}",
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
            "Input: {:?}\nExpected extension: {:?}\nReceived: {:?}",
            input, expected, result
        );
    }

    #[test]
    fn new_metadata_full_integration() {
        // Arrange
        let config = setup_config();
        let args = FileMetadata::builder()
            .with_signature(&Some("test@signature".to_string()))
            .with_title(&Some("My Cool Title!".to_string()))
            .with_keywords(&Some("rust programming".to_string()))
            .with_extension(&Some("RS".to_string()));

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
        let result = args.build(&config);

        // Assert
        assert_eq!(
            result.identifier, expected.identifier,
            "Identifier mismatch:\nExpected: {:?}\nReceived: {:?}",
            expected.identifier, result.identifier
        );
        assert_eq!(
            result.signature, expected.signature,
            "Signature mismatch:\nExpected: {:?}\nReceived: {:?}",
            expected.signature, result.signature
        );
        assert_eq!(
            result.title, expected.title,
            "Title mismatch:\nExpected: {:?}\nReceived: {:?}",
            expected.title, result.title
        );
        assert_eq!(
            result.title_raw, expected.title_raw,
            "Title raw mismatch:\nExpected: {:?}\nReceived: {:?}",
            expected.title_raw, result.title_raw
        );
        assert_eq!(
            result.keywords, expected.keywords,
            "Keywords mismatch:\nExpected: {:?}\nReceived: {:?}",
            expected.keywords, result.keywords
        );
        assert_eq!(
            result.extension, expected.extension,
            "Extension mismatch:\nExpected: {:?}\nReceived: {:?}",
            expected.extension, result.extension
        );
        assert_eq!(
            result.datetime, expected.datetime,
            "Datetime mismatch:\nExpected: {:?}\nReceived: {:?}",
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
            "Input: {:?}\nExpected sanitized string: {:?}\nReceived: {:?}",
            input, expected, result
        );
    }
}
