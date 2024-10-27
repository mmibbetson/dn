use chrono::{DateTime, Local};
use regex::Regex;

use crate::filename::{FilenameDetails, DN_IDENTIFIER_FORMAT};

// Match 8 digits, the letter 'T', then 6 digits.
const IDENTIFIER_PATTERN: &str = r"(\b[0-9]{8}T[0-9]{6}\b)";
const SIGNATURE_PATTERN: &str = r"(==[^\@\-\_\.]*)";
const TITLE_PATTERN: &str = r"(--[^\@\=\_\.]*)";
const KEYWORDS_PATTERN: &str = r"(__[^\@\=\-\.]*)";
const EXTENSION_PATTERN: &str = r"(\.[^\@\=\-\_]*)";

pub fn parse_filename_details(filename: &str, parse_time: DateTime<Local>) -> FilenameDetails {
    let identifier_arg = parse_segment(filename, IDENTIFIER_PATTERN);
    let signature_arg = parse_segment(filename, SIGNATURE_PATTERN);
    let title_arg = parse_segment(filename, TITLE_PATTERN);
    let keywords_arg = parse_segment(filename, KEYWORDS_PATTERN);
    let extension_arg = parse_segment(filename, EXTENSION_PATTERN);
    let creation_time = identifier_arg
        .as_deref()
        .map_or(parse_time, derive_creation_time);

    FilenameDetails {
        creation_time,
        identifier_arg,
        signature_arg,
        title_arg,
        keywords_arg,
        extension_arg,
    }
}

fn parse_segment(filename: &str, pattern: &str) -> Option<String> {
    Regex::new(pattern)
        .unwrap()
        .find(filename)
        .map(|m| m.as_str().to_owned())
}

fn derive_creation_time(identifier: &str) -> DateTime<Local> {
    DateTime::parse_from_str(&identifier, DN_IDENTIFIER_FORMAT)
        .map(|dt| dt.with_timezone(&Local))
        .unwrap_or_else(|_| Local::now()) // Fallback in case of parsing error
}
