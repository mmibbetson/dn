//! TODO

use std::fmt::Display;

use chrono::Local;
use regex::Regex;

use crate::{
    config::{FileConfig, FilenameSegment},
    format::DN_IDENTIFIER_FORMAT,
    metadata::FileMetadata,
};

const PATTERN_SEGMENT_IDENTIFIER: &str = r"(\b[0-9]{8}T[0-9]{6}\b)";
const PATTERN_SEGMENT_SIGNATURE: &str = r"(==[^\@\-\_\.]*)";
const PATTERN_SEGMENT_TITLE: &str = r"(--[^\@\=\_\.]*)";
const PATTERN_SEGMENT_KEYWORDS: &str = r"(__[^\@\=\-\.]*)";
const PATTERN_SEGMENT_EXTENSION: &str = r"(\.[^\@\=\-\_]*)";

/// Represents the possible segments of a dn file name, as well as the order in which
/// they should be concatenated.
#[derive(Debug, PartialEq, Default, Clone)]
pub struct Filename {
    pub identifier: String,
    pub signature: Option<String>,
    pub title: Option<String>,
    pub keywords: Option<String>,
    pub extension: String,
    pub segment_order: [FilenameSegment; 5],
}

/// A trait for converting a value to a `Filename`.
///
/// Does not perform sanitisation on the data before creating the `Filename`.
/// Intended to be used between the file name written to the file system and the
/// `FileMetadata` struct.
///
/// When converting from `FileMetada` to `Filename`, the content will be
/// sanitised. When converting from `String` to `Filename`, the resulting struct
/// should be converted into a `FileMetadata` via the `FileMetadataBuilder` to
/// ensure the validity of the content. From there it can be converted into a
/// `Filename` again or any other type covertible from `FileMetadata`.
pub trait ToFilename {
    /// Converts the given value to a `Filename`.
    fn to_filename(&self, config: &FileConfig) -> Filename;
}

impl Display for Filename {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ordered = self
            .segment_order
            .map(|seg| match seg {
                FilenameSegment::Identifier => self.identifier.clone(),
                FilenameSegment::Signature => self.signature.clone().unwrap_or_default(),
                FilenameSegment::Title => self.title.clone().unwrap_or_default(),
                FilenameSegment::Keywords => self.keywords.clone().unwrap_or_default(),
                FilenameSegment::Extension => self.extension.clone(),
            })
            .concat();

        write!(f, "{ordered}")
    }
}

impl ToFilename for String {
    fn to_filename(&self, config: &FileConfig) -> Filename {
        let (identifier, signature, title, keywords) =
            if let Some(identifier) = parse_segment(self, PATTERN_SEGMENT_IDENTIFIER) {
                let signature = parse_segment(self, PATTERN_SEGMENT_SIGNATURE);
                let title = parse_segment(self, PATTERN_SEGMENT_TITLE);
                let keywords = parse_segment(self, PATTERN_SEGMENT_KEYWORDS);

                (identifier, signature, title, keywords)
            } else {
                let identifier = Local::now().format(DN_IDENTIFIER_FORMAT).to_string();
                let title = self
                    .chars()
                    .take_while(|&c| c != '.')
                    .collect::<String>()
                    .into();

                (identifier, None, title, None)
            };

        let extension = parse_segment(self, PATTERN_SEGMENT_EXTENSION)
            .unwrap_or_else(|| config.default_extension.clone());

        Filename {
            identifier,
            signature,
            title,
            keywords,
            extension,
            ..Default::default()
        }
    }
}

impl ToFilename for FileMetadata {
    fn to_filename(&self, config: &FileConfig) -> Filename {
        let identifier = match config.segment_order[0] {
            FilenameSegment::Identifier => self.identifier.clone(),
            _ => prefix_segment(&self.identifier, FilenameSegment::Identifier),
        };

        let signature = self
            .signature
            .clone()
            .map(|s| prefix_segment(&s, FilenameSegment::Signature));

        let title = self
            .title
            .clone()
            .map(|t| prefix_segment(&t, FilenameSegment::Title));

        let keywords = self.keywords.clone().map(|w| {
            prefix_segment(
                &w.into_iter().collect::<Vec<_>>().join("_"),
                FilenameSegment::Keywords,
            )
        });

        Filename {
            identifier,
            title,
            signature,
            keywords,
            extension: self.extension.clone(),
            segment_order: config.segment_order,
        }
    }
}

/// Attempts to parse a segment from a filename based on a regular expression and
/// return it as an `Option<String>`
///
/// **WARN**: Currently may panic on unwrap if the regex fails to be constructed.
fn parse_segment(filename: &str, pattern: &str) -> Option<String> {
    Regex::new(pattern)
        // WARN: Unwrap may panic. Do we want to alert the user of an error creating this regex?
        .unwrap()
        .find(filename)
        .map(|m| m.as_str().to_owned())
}

/// Applies a prefix corresponding to the `FilenameSegment` variant to an input string.
fn prefix_segment(value: &str, segment: FilenameSegment) -> String {
    let prefix = match segment {
        FilenameSegment::Identifier => "@@",
        FilenameSegment::Signature => "==",
        FilenameSegment::Title => "--",
        FilenameSegment::Keywords => "__",
        FilenameSegment::Extension => ".",
    };

    format!("{prefix}{value}")
}

///////////
// Tests //
///////////

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use chrono::Local;

    #[test]
    fn string_to_filename_with_all_segments() {
        // Arrange
        let input = "20240101T120000==signature--title__keywords.txt".to_string();
        let config = FileConfig::default();
        let expected = Filename {
            identifier: "20240101T120000".to_string(),
            signature: Some("==signature".to_string()),
            title: Some("--title".to_string()),
            keywords: Some("__keywords".to_string()),
            extension: ".txt".to_string(),
            segment_order: Default::default(),
        };

        // Act
        let result = input.to_filename(&config);

        // Assert
        assert_eq!(
            expected, result,
            "\nInput: {input:#?}\nExpected: {expected:#?}\nReceived: {result:#?}"
        );
    }

    #[test]
    fn string_to_filename_non_dn_format() {
        // Arrange
        let input = "@@I am a nau==ghty __STR1NG!.txt".to_string();
        let config = FileConfig::default();
        let now = Local::now();
        let expected = Filename {
            identifier: now.format(DN_IDENTIFIER_FORMAT).to_string(),
            signature: None,
            title: Some("@@I am a nau==ghty __STR1NG!".to_string()),
            keywords: None,
            extension: ".txt".to_string(),
            segment_order: Default::default(),
        };

        // Act
        let result = input.to_filename(&config);

        // Assert
        assert_eq!(
            expected, result,
            "\nInput: {input:#?}\nExpected: {expected:#?}\nReceived: {result:#?}"
        );
    }

    #[test]
    fn metadata_to_filename_full() {
        // Arrange
        let config = FileConfig::default();
        let metadata = FileMetadata {
            identifier: "20240101T120000".to_string(),
            signature: Some("test-sig".to_string()),
            title: Some("test-title".to_string()),
            keywords: Some(HashSet::from(["key1".to_string(), "key2".to_string()])),
            extension: ".txt".to_string(),
            ..Default::default()
        };
        let expected = Filename {
            identifier: "20240101T120000".to_string(),
            signature: Some("==test-sig".to_string()),
            title: Some("--test-title".to_string()),
            keywords: Some("__key1_key2".to_string()),
            extension: ".txt".to_string(),
            segment_order: config.segment_order,
        };

        let expected_keywords = HashSet::from(["key1", "key2"]);

        // Act
        let result = metadata.to_filename(&config);
        let result_keywords = result
            .keywords
            .as_ref()
            .unwrap()
            .split('_')
            .filter(|&w| !w.is_empty())
            .collect::<HashSet<_>>();

        // Assert
        assert_eq!(
            expected.identifier, result.identifier,
            "\nInput: {:#?}\nExpected: {:#?}\nReceived: {:#?}",
            metadata.identifier, expected.identifier, result.identifier
        );

        assert_eq!(
            expected.signature, result.signature,
            "\nInput: {:#?}\nExpected: {:#?}\nReceived: {:#?}",
            metadata.signature, expected.signature, result.signature
        );

        assert_eq!(
            expected.title, result.title,
            "\nInput: {:#?}\nExpected: {:#?}\nReceived: {:#?}",
            metadata.title, expected.title, result.title
        );

        assert_eq!(
            expected_keywords, result_keywords,
            "\nInput: {:#?}\nExpected: {:#?}\nReceived: {:#?}",
            metadata.keywords, expected.keywords, result.keywords
        );

        assert_eq!(
            expected.extension, result.extension,
            "\nInput: {:#?}\nExpected: {:#?}\nReceived: {:#?}",
            metadata.extension, expected.extension, result.extension
        );
    }

    #[test]
    fn filename_display_format() {
        // Arrange
        let filename = Filename {
            identifier: "20240101T120000".to_string(),
            signature: Some("==signature".to_string()),
            title: Some("--title".to_string()),
            keywords: Some("__keywords".to_string()),
            extension: ".txt".to_string(),
            segment_order: [
                FilenameSegment::Identifier,
                FilenameSegment::Signature,
                FilenameSegment::Title,
                FilenameSegment::Keywords,
                FilenameSegment::Extension,
            ],
        };
        let expected = "20240101T120000==signature--title__keywords.txt";

        // Act
        let result = filename.to_string();

        // Assert
        assert_eq!(
            expected, result,
            "\nInput: {filename:#?}\nExpected: {expected}\nReceived: {result}"
        );
    }

    #[test]
    fn parse_segment_extracts_correct_parts() {
        // Arrange
        let filename = "20240101T120000==signature--title__keywords.txt";
        let test_cases = [
            (
                PATTERN_SEGMENT_IDENTIFIER,
                Some("20240101T120000".to_string()),
            ),
            (PATTERN_SEGMENT_SIGNATURE, Some("==signature".to_string())),
            (PATTERN_SEGMENT_TITLE, Some("--title".to_string())),
            (PATTERN_SEGMENT_KEYWORDS, Some("__keywords".to_string())),
            (PATTERN_SEGMENT_EXTENSION, Some(".txt".to_string())),
        ];

        for (pattern, expected) in test_cases {
            // Act
            let result = parse_segment(filename, pattern);

            // Assert
            assert_eq!(
                expected, result,
                "\nInput: {filename}\nPattern: {pattern}\nExpected: {expected:#?}\nReceived: {result:#?}"
            );
        }
    }

    #[test]
    fn prefix_segment_adds_correct_prefix() {
        // Arrange
        let test_cases = [
            (FilenameSegment::Identifier, "@@test"),
            (FilenameSegment::Signature, "==test"),
            (FilenameSegment::Title, "--test"),
            (FilenameSegment::Keywords, "__test"),
            (FilenameSegment::Extension, ".test"),
        ];
        let input = "test";

        for (segment, expected) in test_cases {
            // Act
            let result = prefix_segment(input, segment);

            // Assert
            assert_eq!(
                expected, result,
                "\nInput: {input}\nSegment: {segment:#?}\nExpected: {expected}\nReceived: {result}"
            );
        }
    }
}
