use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use regex::Regex;

use crate::filename::{FilenameDetails, DN_IDENTIFIER_FORMAT};

const IDENTIFIER_PATTERN: &str = r"(\b[0-9]{8}T[0-9]{6}\b)";
const SIGNATURE_PATTERN: &str = r"(==[^\@\-\_\.]*)";
const TITLE_PATTERN: &str = r"(--[^\@\=\_\.]*)";
const KEYWORDS_PATTERN: &str = r"(__[^\@\=\-\.]*)";
const EXTENSION_PATTERN: &str = r"(\.[^\@\=\-\_]*)";

pub fn parse_filename_details(filename: &str, parse_time: DateTime<Local>) -> FilenameDetails {
    let identifier_arg = parse_segment(filename, IDENTIFIER_PATTERN);
    let signature_arg = parse_segment(filename, SIGNATURE_PATTERN);
    let title_arg = if identifier_arg.is_some() {
        parse_segment(filename, TITLE_PATTERN)
    } else {
        // Take the existing file name sans the extension.
        Some(filename.chars().take_while(|&c| c != '.').collect())
    };
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
mod tests {
    use chrono::{TimeZone, Timelike};

    use super::*;

    fn truncate_nanoseconds(datetime: DateTime<Local>) -> DateTime<Local> {
        datetime.with_nanosecond(0).unwrap()
    }

    #[test]
    fn test_non_dn_filename_parsing() {
        let parse_time: DateTime<Local> = Local::now();
        let filename = "My File!.txt";

        let expected_title_arg = "My File!".to_string();

        let result = parse_filename_details(filename, parse_time);

        assert_eq!(
            truncate_nanoseconds(result.creation_time),
            truncate_nanoseconds(parse_time),
            "Creation time should match parse time for non-DN files (ignoring subseconds). Expected {:?}, got {:?}",
            truncate_nanoseconds(parse_time),
            truncate_nanoseconds(result.creation_time)
        );
        assert!(
            result.identifier_arg.is_none(),
            "Identifier should be None for non-DN files, got {:?}",
            result.identifier_arg
        );
        assert!(
            result.signature_arg.is_none(),
            "Signature should be None for non-DN files, got {:?}",
            result.signature_arg
        );
        assert_eq!(
            result.title_arg,
            Some(expected_title_arg.clone()),
            "Title should be the full filename (without the file extension) for non-DN files. Expected {:?}, got {:?}",
            Some(expected_title_arg.clone()),
            result.title_arg
        );
        assert!(
            result.keywords_arg.is_none(),
            "Keywords should be None for non-DN files, got {:?}",
            result.keywords_arg
        );
        assert_eq!(
            result.extension_arg,
            Some(".txt".to_string()),
            "Extension should be .txt. Expected {:?}, got {:?}",
            Some(".txt".to_string()),
            result.extension_arg
        );
    }

    #[test]
    fn test_dn_filename_parsing() {
        let filename = "20241201T121212==123=4--my-t171e-__test_key.temp.txt";
        let parse_time: DateTime<Local> = Local::now();
        let expected_datetime = Local.with_ymd_and_hms(2024, 12, 1, 12, 12, 12).unwrap();

        let result = parse_filename_details(filename, parse_time);

        assert_eq!(
            truncate_nanoseconds(result.creation_time),
            truncate_nanoseconds(expected_datetime),
            "Creation time should match the datetime encoded in the filename (ignoring subseconds). Expected {:?}, got {:?}",
            truncate_nanoseconds(expected_datetime),
            truncate_nanoseconds(result.creation_time)
        );
        assert_eq!(
            result.identifier_arg,
            Some("20241201T121212".to_string()),
            "Identifier should match the datetime string. Expected {:?}, got {:?}",
            Some("20241201T121212".to_string()),
            result.identifier_arg
        );
        assert_eq!(
            result.signature_arg,
            Some("==123=4".to_string()),
            "Signature should match the == segment. Expected {:?}, got {:?}",
            Some("==123=4".to_string()),
            result.signature_arg
        );
        assert_eq!(
            result.title_arg,
            Some("--my-t171e-".to_string()),
            "Title should match the -- segment. Expected {:?}, got {:?}",
            Some("--my-t171e-".to_string()),
            result.title_arg
        );
        assert_eq!(
            result.keywords_arg,
            Some("__test_key".to_string()),
            "Keywords should match the __ segment. Expected {:?}, got {:?}",
            Some("__test_key".to_string()),
            result.keywords_arg
        );
        assert_eq!(
            result.extension_arg,
            Some(".temp.txt".to_string()),
            "Extension should match the full extension string. Expected {:?}, got {:?}",
            Some(".temp.txt".to_string()),
            result.extension_arg
        );
    }

    #[test]
    fn test_derive_creation_time() {
        let identifier = "20241201T121212";
        let result = derive_creation_time(identifier);
        let expected = Local.with_ymd_and_hms(2024, 12, 1, 12, 12, 12).unwrap();

        assert_eq!(
            truncate_nanoseconds(result),
            truncate_nanoseconds(expected),
            "DateTime parsing failed. Input: {}, Format: {}, Expected: {:?}, Got: {:?}",
            identifier,
            DN_IDENTIFIER_FORMAT,
            truncate_nanoseconds(expected),
            truncate_nanoseconds(result)
        );
    }
}
