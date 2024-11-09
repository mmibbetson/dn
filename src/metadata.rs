//! TODO

use std::collections::HashSet;

use chrono::{DateTime, Local, NaiveDateTime, TimeZone};

use crate::{config::FileConfig, format::DN_IDENTIFIER_FORMAT};

/// The unique separator characters for each segment of a dn file name.
pub const SEGMENT_SEPARATORS: [char; 5] = ['@', '=', '-', '_', '.'];

/// Represents the deserialised metadata associated with a note that can be encoded
/// in its title and/or frontmatter.
#[derive(Debug, Default, Clone)]
pub struct FileMetadata {
    pub identifier: String,
    pub signature: Option<String>,
    pub title: Option<String>,
    pub title_raw: Option<String>,
    pub keywords: Option<HashSet<String>>,
    pub extension: String,
    pub datetime: DateTime<Local>,
}

/// A `mut self` builder that allows progressively updating an input state for a new `FileMetadata`.
#[derive(Debug, Default)]
struct FileMetadataBuilder {
    identifier: Option<String>,
    signature: Option<String>,
    title: Option<String>,
    keywords: Option<String>,
    added_keywords: Option<String>,
    removed_keywords: Option<String>,
    extension: Option<String>,
    datetime: DateTime<Local>,
}

impl FileMetadata {
    /// Creates a new builder initialised with default values.
    pub fn builder() -> FileMetadataBuilder {
        FileMetadataBuilder::default()
    }
}

impl FileMetadataBuilder {
    /// Optionally adds an identifier to the builder which will override the default.
    pub fn with_identifier(mut self, value: &Option<String>) -> Self {
        self.identifier = value.clone();
        self
    }

    /// Optionally adds a signature to the builder.
    pub fn with_signature(mut self, value: &Option<String>) -> Self {
        self.signature = value.clone();
        self
    }

    /// Optionally adds a title to the builder.
    pub fn with_title(mut self, value: &Option<String>) -> Self {
        self.title = value.clone();
        self
    }

    /// Optionally adds keywords to the builder.
    pub fn with_keywords(mut self, value: &Option<String>) -> Self {
        self.keywords = value.clone();
        self
    }

    /// Optionally adds additional keywords to be joined with existing keywords to the builder.
    pub fn with_added_keywords(mut self, value: &Option<String>) -> Self {
        self.added_keywords = value.clone();
        self
    }

    /// Optionally adds additional keywords to be removed from existing and added keywords to the
    /// builder.
    pub fn with_removed_keywords(mut self, value: &Option<String>) -> Self {
        self.removed_keywords = value.clone();
        self
    }

    /// Optionally adds a file extension to the builder which will override the default.
    pub fn with_extension(mut self, value: &Option<String>) -> Self {
        self.extension = value.clone();
        self
    }

    /// Builds the final `FileMetadata` state, falling back to the default builder values where
    /// values have not been otherwise provided.
    ///
    /// Parses and sanitises the various segment arguments into their correct format
    /// for use in dn before constructing the `FileMetadata`.
    pub fn build(&self, config: &FileConfig) -> FileMetadata {
        let datetime = derive_datetime(&self.identifier);

        let identifier = self
            .identifier
            .clone()
            .unwrap_or_else(|| self.datetime.format(DN_IDENTIFIER_FORMAT).to_string());

        let signature = self
            .signature
            .as_ref()
            .and_then(|s| parse_signature(&s, &config.illegal_characters));

        let title = self
            .title
            .as_ref()
            .and_then(|t| parse_title(&t, &config.illegal_characters));

        let title_raw = self.title.as_ref().map(String::from);

        let keywords = {
            let base_keywords = self
                .keywords
                .as_ref()
                .and_then(|k| parse_keywords(&k, &config.illegal_characters))
                .unwrap_or_default();

            let added_keywords = self
                .added_keywords
                .as_ref()
                .and_then(|k| parse_keywords(&k, &config.illegal_characters))
                .unwrap_or_default();

            let removed_keywords = self
                .removed_keywords
                .as_ref()
                .and_then(|k| parse_keywords(&k, &config.illegal_characters))
                .unwrap_or_default();

            match base_keywords.is_empty() && added_keywords.is_empty() {
                true => None,
                false => Some(
                    base_keywords
                        .into_iter()
                        .chain(added_keywords)
                        .filter(|k| !removed_keywords.contains(k))
                        .collect(),
                ),
            }
        };

        let extension = self
            .extension
            .as_ref()
            .and_then(|e| parse_extension(&e, &config.illegal_characters))
            .unwrap_or_else(|| config.default_extension.clone());

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

/// Derives a local datetime from an optional dn identifier. If the identifier is
/// not successfully parsed into a datetime, then falls back to `Local::now()`.
fn derive_datetime(identifier: &Option<String>) -> DateTime<Local> {
    match identifier {
        Some(identifier) => match NaiveDateTime::parse_from_str(identifier, DN_IDENTIFIER_FORMAT) {
            Ok(naive) => TimeZone::from_local_datetime(&Local, &naive)
                .single()
                .unwrap_or_else(Local::now),
            Err(_) => Local::now(),
        },
        None => Local::now(),
    }
}

/// Parses the signature argument to a valid dn signature.
fn parse_signature(signature_arg: &str, illegal_characters: &HashSet<char>) -> Option<String> {
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

/// Parses the title argument to a valid dn title.
fn parse_title(title_arg: &str, illegal_characters: &HashSet<char>) -> Option<String> {
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

/// Parses the keywords argument to a valid dn keywords vector.
fn parse_keywords(keywords_arg: &str, illegal_characters: &HashSet<char>) -> Option<Vec<String>> {
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

/// Parses the extension argument to a valid dn extension.
fn parse_extension(extension_arg: &str, illegal_characters: &HashSet<char>) -> Option<String> {
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

/// Removes segment separators and illegal characters from a dirty string.
fn sanitise(dirty: &str, illegal_characters: &HashSet<char>) -> String {
    dirty
        .chars()
        .filter(|&c| !SEGMENT_SEPARATORS.contains(&c))
        .filter(|&c| !illegal_characters.contains(&c))
        .collect::<String>()
}

///////////
// Tests //
///////////

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn setup_config() -> FileConfig {
        FileConfig {
            illegal_characters: HashSet::from(['#', '@', '!']),
            default_extension: String::from("md"),
            ..Default::default()
        }
    }

    fn setup_datetime() -> DateTime<Local> {
        // WARN: Unwrap may panic.
        Local.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap()
    }

    #[test]
    fn derive_datetime_with_identifier() {
        // Arrange
        let input = Some("20241212T121212".to_string());
        let expected = Local.with_ymd_and_hms(2024, 12, 12, 12, 12, 12).unwrap();

        // Act
        let result = derive_datetime(&input);

        // Assert
        assert_eq!(
            expected, result,
            "Input: {:#?}\nExpected datetime: {:#?}\nReceived: {:#?}",
            input, expected, result
        );
    }

    #[test]
    fn derive_datetime_without_identifier() {
        // Arrange
        let input = None;
        let before_call = Local::now();

        // Act
        let result = derive_datetime(&input);
        let after_call = Local::now();

        // Assert
        assert!(
            result >= before_call && result <= after_call,
            "Input: Local::now()\nExpected datetime between: {:#?} and {:#?}\nReceived: {:#?}",
            before_call,
            after_call,
            result
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
            expected, result,
            "Input: {:#?}\nExpected signature: {:#?}\nReceived: {:#?}",
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
            expected, result,
            "Input: {:#?}\nExpected signature: {:#?}\nReceived: {:#?}",
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
            expected, result,
            "Input: {:#?}\nExpected signature: {:#?}\nReceived: {:#?}",
            input, expected, result
        );
    }

    #[test]
    fn parse_title_with_multiple_parts() {
        // Arrange
        let config = setup_config();
        let input = "My-teST Title";
        let expected = Some("my-test-title".to_string());

        // Act
        let result = parse_title(input, &config.illegal_characters);

        // Assert
        assert_eq!(
            expected, result,
            "Input: {:#?}\nExpected title: {:#?}\nReceived: {:#?}",
            input, expected, result
        );
    }

    #[test]
    fn parse_keywords_with_multiple_parts() {
        // Arrange
        let config = setup_config();
        let input = "_dn_tags test";
        let expected = Some(vec![
            "dn".to_string(),
            "tags".to_string(),
            "test".to_string(),
        ]);

        // Act
        let result = parse_keywords(input, &config.illegal_characters);

        // Assert
        assert_eq!(
            expected, result,
            "Input: {:#?}\nExpected keywords: {:#?}\nReceived: {:#?}",
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
            expected, result,
            "Input: {:#?}\nExpected extension: {:#?}\nReceived: {:#?}",
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
            expected, result,
            "Input: {:#?}\nExpected extension: {:#?}\nReceived: {:#?}",
            input, expected, result
        );
    }

    #[test]
    fn new_metadata_full_integration() {
        // Arrange
        let config = setup_config();
        let args = FileMetadata::builder()
            .with_identifier(&Some("20240101T120000".to_string()))
            .with_signature(&Some("test@signature".to_string()))
            .with_title(&Some("My T3ST Title!".to_string()))
            .with_keywords(&Some("dn testing".to_string()))
            .with_added_keywords(&Some("dn_testing_changes".to_string()))
            .with_removed_keywords(&Some("dn".to_string()))
            .with_extension(&Some("DJ".to_string()));

        let expected = FileMetadata {
            identifier: "20240101T120000".to_string(),
            signature: Some("testsignature".to_string()),
            title: Some("my-t3st-title".to_string()),
            title_raw: Some("My T3ST Title!".to_string()),
            keywords: Some(HashSet::from([
                "testing".to_string(),
                "changes".to_string(),
            ])),
            extension: "dj".to_string(),
            datetime: setup_datetime(),
            ..Default::default()
        };

        // Act
        let result = args.build(&config);

        // Assert
        assert_eq!(
            expected.identifier, result.identifier,
            "Identifier mismatch:\nExpected: {:#?}\nReceived: {:#?}",
            expected.identifier, result.identifier
        );
        assert_eq!(
            expected.signature, result.signature,
            "Signature mismatch:\nExpected: {:#?}\nReceived: {:#?}",
            expected.signature, result.signature
        );
        assert_eq!(
            expected.title, result.title,
            "Title mismatch:\nExpected: {:#?}\nReceived: {:#?}",
            expected.title, result.title
        );
        assert_eq!(
            expected.title_raw, result.title_raw,
            "Title raw mismatch:\nExpected: {:#?}\nReceived: {:#?}",
            expected.title_raw, result.title_raw
        );
        assert_eq!(
            expected.keywords, result.keywords,
            "Keywords mismatch:\nExpected: {:#?}\nReceived: {:#?}",
            expected.keywords, result.keywords
        );
        assert_eq!(
            expected.extension, result.extension,
            "Extension mismatch:\nExpected: {:#?}\nReceived: {:#?}",
            expected.extension, result.extension
        );
        assert_eq!(
            expected.datetime, result.datetime,
            "Datetime mismatch:\nExpected: {:#?}\nReceived: {:#?}",
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
            expected, result,
            "Input: {:#?}\nExpected sanitized string: {:#?}\nReceived: {:#?}",
            input, expected, result
        );
    }
}
