use regex::Regex;

use crate::{
    config::{FileConfig, FilenameSegment},
    format::DN_IDENTIFIER_FORMAT,
    metadata::FileMetadata,
};

#[derive(Debug, Default, Clone)]
pub struct Filename {
    pub identifier: String,
    pub signature: Option<String>,
    pub title: Option<String>,
    pub keywords: Option<String>,
    pub extension: String,
    pub segment_order: [FilenameSegment; 5],
}

pub trait ToFilename {
    fn to_filename(&self, config: &FileConfig) -> Filename;
}

impl ToFilename for String {
    fn to_filename(&self, config: &FileConfig) -> Filename {
        const IDENTIFIER_PATTERN: &str = r"(?P<identifier>\b[0-9]{8}T[0-9]{6}\b)";
        const SIGNATURE_PATTERN: &str = r"(?P<signature>==[^\@\-\_\.]*)";
        const TITLE_PATTERN: &str = r"(?P<title>--[^\@\=\_\.]*)";
        const KEYWORDS_PATTERN: &str = r"(?P<keywords>__[^\@\=\-\.]*)";
        const EXTENSION_PATTERN: &str = r"(?P<extension>\.[^\@\=\-\_]*)";

        let combined_pattern = format!(
            "{}|{}|{}|{}|{}",
            IDENTIFIER_PATTERN,
            SIGNATURE_PATTERN,
            TITLE_PATTERN,
            KEYWORDS_PATTERN,
            EXTENSION_PATTERN
        );

        // TODO: Iterate over captures to determine their values and order.

        let identifier = match identifier_capture {
            Some(id) => id.to_string(),
            None => chrono::Local::now()
                .format(DN_IDENTIFIER_FORMAT)
                .to_string(),
        };

        let extension = match extension_capture {
            Some(ext) => ext.to_string(),
            None => config.default_extension,
        };

        Filename {
            identifier,
            signature: todo!(),
            title: todo!(),
            keywords: todo!(),
            extension,
            segment_order: todo!(),
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

fn parse_segment(filename: &str, pattern: &str) -> Option<String> {
    Regex::new(pattern)
        // WARN: Unwrap may panic.
        .unwrap()
        .find(filename)
        .map(|m| m.as_str().to_owned())
}

///////////
// Tests //
///////////

#[cfg(test)]
mod tests {}
