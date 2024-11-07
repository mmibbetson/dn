//! TODO

use std::fmt::Display;

use chrono::Local;
use regex::Regex;

use crate::{
    config::{FileConfig, FilenameSegment},
    format::DN_IDENTIFIER_FORMAT,
    metadata::FileMetadata,
};

/// Represents the possible segments of a dn file name, as well as the order in which
/// they should be concatenated.
#[derive(Debug, Default, Clone)]
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
            .clone()
            .map(|seg| match seg {
                FilenameSegment::Identifier => self.identifier.clone(),
                FilenameSegment::Signature => self.signature.clone().unwrap_or_default(),
                FilenameSegment::Title => self.title.clone().unwrap_or_default(),
                FilenameSegment::Keywords => self.keywords.clone().unwrap_or_default(),
                FilenameSegment::Extension => self.extension.clone(),
            })
            .concat();

        write!(f, "{}", ordered)
    }
}

impl ToFilename for String {
    fn to_filename(&self, config: &FileConfig) -> Filename {
        const IDENTIFIER_PATTERN: &str = r"(\b[0-9]{8}T[0-9]{6}\b)";
        const SIGNATURE_PATTERN: &str = r"(==[^\@\-\_\.]*)";
        const TITLE_PATTERN: &str = r"(--[^\@\=\_\.]*)";
        const KEYWORDS_PATTERN: &str = r"(__[^\@\=\-\.]*)";
        const EXTENSION_PATTERN: &str = r"(\.[^\@\=\-\_]*)";

        let identifier = match parse_segment(&self, IDENTIFIER_PATTERN) {
            Some(identifier) => identifier.to_string(),
            None => Local::now().format(DN_IDENTIFIER_FORMAT).to_string(),
        };
        let signature = parse_segment(self, SIGNATURE_PATTERN);
        let title = parse_segment(self, TITLE_PATTERN);
        let keywords = parse_segment(self, KEYWORDS_PATTERN);
        let extension = match parse_segment(self, EXTENSION_PATTERN) {
            Some(extension) => extension.to_string(),
            None => config.default_extension.clone(),
        };

        // TODO: If only extension is present, treat eveything else as title.

        Filename {
            identifier,
            signature,
            title,
            keywords,
            extension,
            // NOTE: We don't care about segment order when parsing as it should be determined
            // by the configuration options.
            ..Default::default()
        }
    }
}

impl ToFilename for FileMetadata {
    fn to_filename(&self, config: &FileConfig) -> Filename {
        let identifier = match config.segment_order[0] {
            FilenameSegment::Identifier => &self.identifier,
            _ => &prefix_segment(self.identifier.clone(), &FilenameSegment::Identifier),
        };

        Filename {
            identifier: identifier.clone(),
            signature: self.signature.clone(),
            title: self.title.clone(),
            keywords: self.keywords.clone().map(|words| words.concat()),
            extension: self.extension.clone(),
            segment_order: config.segment_order.clone(),
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

/// Applies a prefix corresponding to the FilenameSegment variant to an input string.
fn prefix_segment(value: String, segment: &FilenameSegment) -> String {
    let prefix = match segment {
        FilenameSegment::Identifier => "@@",
        FilenameSegment::Signature => "==",
        FilenameSegment::Title => "--",
        FilenameSegment::Keywords => "__",
        FilenameSegment::Extension => ".",
    };

    format!("{}{}", prefix, value)
}

///////////
// Tests //
///////////

#[cfg(test)]
mod tests {}
