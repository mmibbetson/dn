use chrono::Local;
use regex::Regex;

use crate::{
    config::{FileConfig, FilenameSegment},
    format::DN_IDENTIFIER_FORMAT,
    metadata::FileMetadata,
};

/// TODO
#[derive(Debug, Default, Clone)]
pub struct Filename {
    pub identifier: String,
    pub signature: Option<String>,
    pub title: Option<String>,
    pub keywords: Option<String>,
    pub extension: String,
    pub segment_order: [FilenameSegment; 5],
}

/// TODO
pub trait ToFilename {
    fn to_filename(&self, config: &FileConfig) -> Filename;
}

impl ToString for Filename {
    fn to_string(&self) -> String {
        self.segment_order
            .clone()
            .map(|seg| match seg {
                FilenameSegment::Identifier => self.identifier.clone(),
                FilenameSegment::Signature => self.signature.clone().unwrap_or_default(),
                FilenameSegment::Title => self.title.clone().unwrap_or_default(),
                FilenameSegment::Keywords => self.keywords.clone().unwrap_or_default(),
                FilenameSegment::Extension => self.extension.clone(),
            })
            .concat()
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

/// TODO
fn parse_segment(filename: &str, pattern: &str) -> Option<String> {
    Regex::new(pattern)
        // WARN: Unwrap may panic. Do we want to alert the user of an error creating this regex?
        .unwrap()
        .find(filename)
        .map(|m| m.as_str().to_owned())
}

/// TODO
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
