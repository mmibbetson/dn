// SPDX-FileCopyrightText: 2024-2025 Matthew Mark Ibbetson
// SPDX-FileContributor: Matthew Mark Ibbetson
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! Serialisation and deserialisation of dn-compatible file names.

use std::{fmt::Display, sync::LazyLock};

use chrono::Local;
use regex::Regex;

use crate::{
    config::{FileConfig, FilenameSegment},
    metadata::{FileMetadata, DN_IDENTIFIER_FORMAT},
};

/// Regex to match the `Identifier` segment of a file name.
static REGEX_SEGMENT_IDENTIFIER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(\b[0-9]{8}T[0-9]{6}\b)").expect("Invalid identifier segment regex pattern")
});

/// Regex to match the `Signature` segment of a file name.
static REGEX_SEGMENT_SIGNATURE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(==[^\@\-\_\.]*)").expect("Invalid signature segment regex pattern")
});

/// Regex to match the `Title` segment of a file name.
static REGEX_SEGMENT_TITLE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(--[^\@\=\_\.]*)").expect("Invalid title segment regex pattern"));

/// Regex to match the `Keywords` segment of a file name.
static REGEX_SEGMENT_KEYWORDS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(__[^\@\=\-\.]*)").expect("Invalid keywords segment regex pattern")
});

/// Regex to match the `Extension` segment of a file name.
static REGEX_SEGMENT_EXTENSION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(\.[^\@\=\-\_]*)").expect("Invalid extension segment regex pattern")
});

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
/// This trait does **not** perform any sanitization on the data before creating the `Filename`.
/// It is intended for use between the filename written to the file system and the `FileMetadata`
/// struct.
///
/// - When converting from `FileMetadata` to `Filename`, the content will be sanitized.
/// - When converting from `String` to `Filename`, the resulting `Filename` should be converted
///   into a `FileMetadata` using the `FileMetadataBuilder` to ensure content validity. From there,
///   it can be converted back into a `Filename` or any other type convertible from `FileMetadata`.
///
/// # Example
/// ```
/// let config = FileConfig::default();
/// let filename = "--example__tag1.txt".to_filename(&config);
/// ```
pub trait ToFilename {
    /// Converts the value to a `Filename`.
    fn to_filename(&self, config: &FileConfig) -> Filename;
}

impl Display for Filename {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ordered = self
            .segment_order
            .map(|s| match s {
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
            if let Some(identifier) = parse_segment(self, &REGEX_SEGMENT_IDENTIFIER) {
                let signature = parse_segment(self, &REGEX_SEGMENT_SIGNATURE);
                let title = parse_segment(self, &REGEX_SEGMENT_TITLE);
                let keywords = parse_segment(self, &REGEX_SEGMENT_KEYWORDS);

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

        let extension = parse_segment(self, &REGEX_SEGMENT_EXTENSION)
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

        let extension = prefix_segment(&self.extension, FilenameSegment::Extension);

        Filename {
            identifier,
            title,
            signature,
            keywords,
            extension,
            segment_order: config.segment_order,
        }
    }
}
/// Attempts to parse a segment from a filename using the provided regex, returning it as an `Option<String>`.
///
/// # Example
///
/// ```
/// let regex = LazyLock::new(|| Regex::new(r"\d+").unwrap());
/// let result = parse_segment("file123.txt", &regex);
/// assert_eq!(result, Some("123".to_string()));
/// ```
fn parse_segment(filename: &str, regex: &LazyLock<Regex>) -> Option<String> {
    regex.find(filename).map(|m| m.as_str().to_owned())
}

/// Applies a prefix corresponding to the `FilenameSegment` variant to an input string.
///
/// # Example
///
/// ```
/// let result = prefix_segment("file", FilenameSegment::Title);
/// assert_eq!(result, "--file");
/// ```
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

    use super::*;
    use chrono::Local;

    #[test]
    fn string_to_filename_with_all_segments() {
        // Arrange
        let input = "20240101T120000==signature--title__keywords.txt".to_owned();
        let config = FileConfig::default();
        let expected = Filename {
            identifier: "20240101T120000".to_owned(),
            signature: Some("==signature".to_owned()),
            title: Some("--title".to_owned()),
            keywords: Some("__keywords".to_owned()),
            extension: ".txt".to_owned(),
            segment_order: Default::default(),
        };

        // Act
        let result = input.to_filename(&config);

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    fn string_to_filename_non_dn_format() {
        // Arrange
        let input = "@@I am a nau==ghty __STR1NG!.txt".to_owned();
        let config = FileConfig::default();
        let now = Local::now();
        let expected = Filename {
            identifier: now.format(DN_IDENTIFIER_FORMAT).to_string(),
            signature: None,
            title: Some("@@I am a nau==ghty __STR1NG!".to_owned()),
            keywords: None,
            extension: ".txt".to_owned(),
            segment_order: Default::default(),
        };

        // Act
        let result = input.to_filename(&config);

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    fn metadata_to_filename_full() {
        // Arrange
        let config = FileConfig::default();
        let metadata = FileMetadata {
            identifier: "20240101T120000".to_owned(),
            signature: Some("test-sig".to_owned()),
            title: Some("test-title".to_owned()),
            keywords: Some(vec!["key1".to_owned(), "key2".to_owned()]),
            extension: "txt".to_owned(),
            ..Default::default()
        };
        let expected = Filename {
            identifier: "20240101T120000".to_owned(),
            signature: Some("==test-sig".to_owned()),
            title: Some("--test-title".to_owned()),
            keywords: Some("__key1_key2".to_owned()),
            extension: ".txt".to_owned(),
            segment_order: config.segment_order,
        };

        let expected_keywords = vec!["key1", "key2"];

        // Act
        let result = metadata.to_filename(&config);
        let result_keywords = result
            .keywords
            .as_ref()
            .unwrap()
            .split('_')
            .filter(|&w| !w.is_empty())
            .collect::<Vec<_>>();

        // Assert
        assert_eq!(expected.identifier, result.identifier);
        assert_eq!(expected.signature, result.signature);
        assert_eq!(expected.title, result.title);
        assert_eq!(expected_keywords, result_keywords);
        assert_eq!(expected.extension, result.extension);
    }

    #[test]
    fn filename_display_format() {
        // Arrange
        let filename = Filename {
            identifier: "20240101T120000".to_owned(),
            signature: Some("==signature".to_owned()),
            title: Some("--title".to_owned()),
            keywords: Some("__keywords".to_owned()),
            extension: ".txt".to_owned(),
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
        assert_eq!(expected, result);
    }

    #[test]
    fn parse_segment_extracts_correct_parts() {
        // Arrange
        let filename = "20240101T120000==signature--title__keywords.txt";
        let test_cases = [
            (
                &REGEX_SEGMENT_IDENTIFIER,
                Some("20240101T120000".to_owned()),
            ),
            (&REGEX_SEGMENT_SIGNATURE, Some("==signature".to_owned())),
            (&REGEX_SEGMENT_TITLE, Some("--title".to_owned())),
            (&REGEX_SEGMENT_KEYWORDS, Some("__keywords".to_owned())),
            (&REGEX_SEGMENT_EXTENSION, Some(".txt".to_owned())),
        ];

        for (regex, expected) in test_cases {
            // Act
            let result = parse_segment(filename, regex);

            // Assert
            assert_eq!(expected, result);
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
            assert_eq!(expected, result);
        }
    }
}
