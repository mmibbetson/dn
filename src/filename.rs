use std::str::FromStr;

use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use regex::Regex;

use crate::{
    config::{FileConfig, FilenameSegment},
    format::DN_IDENTIFIER_FORMAT,
    metadata::FileMetadata,
};

#[derive(Debug, Default, Clone)]
pub struct Filename {
    identifier: String,
    signature: Option<String>,
    title: Option<String>,
    keywords: Option<String>,
    extension: String,
    segment_order: [FilenameSegment; 5],
}

pub trait ToFilename {
    fn to_filename(&self, config: &FileConfig) -> Filename;
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

impl FromStr for Filename {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const IDENTIFIER_PATTERN: &str = r"(\b[0-9]{8}T[0-9]{6}\b)";
        const SIGNATURE_PATTERN: &str = r"(==[^\@\-\_\.]*)";
        const TITLE_PATTERN: &str = r"(--[^\@\=\_\.]*)";
        const KEYWORDS_PATTERN: &str = r"(__[^\@\=\-\.]*)";
        const EXTENSION_PATTERN: &str = r"(\.[^\@\=\-\_]*)";

        let identifier = parse_segment(s, IDENTIFIER_PATTERN);
        let signature = parse_segment(s, SIGNATURE_PATTERN);

        // NOTE: If there is no identifier, it's a non-dn name. Therefore the whole thing is the title.
        let title = if let Some(_) = identifier {
            parse_segment(s, TITLE_PATTERN)
        } else {
            Some(s.chars().take_while(|&c| c != '.').collect())
        };

        let keywords = parse_segment(s, KEYWORDS_PATTERN);
        let extension = parse_segment(s, EXTENSION_PATTERN);
        let segment_order = derive_segment_order();

        Ok(Filename {
            identifier,
            signature,
            title,
            keywords,
            extension,
            segment_order,
        })
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
        .unwrap()
        .find(filename)
        .map(|m| m.as_str().to_owned())
}

// TODO: Move to appropriate location. Will be used to get
// The creation_time for FileMetadata
fn derive_creation_time(identifier: &str) -> DateTime<Local> {
    match NaiveDateTime::parse_from_str(identifier, DN_IDENTIFIER_FORMAT) {
        Ok(naive) => Local
            .from_local_datetime(&naive)
            .single()
            .unwrap_or_else(|| Local::now()),
        Err(_) => Local::now(),
    }
}

///////////
// Tests //
///////////

#[cfg(test)]
mod tests {}
