use crate::config::FilenameConfig;
use chrono::{DateTime, Local};

#[derive(Clone, Default)]
pub struct FilenameDetails {
    pub creation_time: DateTime<Local>,
    pub identifier_arg: Option<String>,
    pub signature_arg: Option<String>,
    pub title_arg: Option<String>,
    pub keywords_arg: Option<String>,
    pub extension_arg: Option<String>,
}

#[derive(PartialEq)]
pub enum FilenameSegment {
    Identifier,
    Signature,
    Title,
    Keywords,
    Extension,
}

pub const DN_IDENTIFIER_FORMAT: &str = "%Y%m%dT%H%M%S";

/// TODO: Documentation.
pub fn get_filename(filename_details: &FilenameDetails, config: &FilenameConfig) -> String {
    config
        .segment_order
        .iter()
        .map(|segment| process_segment(segment, filename_details, config))
        .collect::<Vec<_>>()
        .concat()
}

fn process_segment(
    segment: &FilenameSegment,
    details: &FilenameDetails,
    config: &FilenameConfig,
) -> String {
    let arg = match segment {
        FilenameSegment::Identifier => &details.identifier_arg,
        FilenameSegment::Signature => &details.signature_arg,
        FilenameSegment::Title => &details.title_arg,
        FilenameSegment::Keywords => &details.keywords_arg,
        FilenameSegment::Extension => &details.extension_arg,
    };
    let prefix = segment_prefix(segment);

    match segment {
        FilenameSegment::Identifier => {
            format_identifier(details.creation_time, config.segment_order[0] == *segment)
        }
        FilenameSegment::Extension => {
            let extension = arg.as_deref().unwrap_or(&config.default_file_extension);
            let formatted = format_segment(extension, prefix, &config.illegal_characters);

            if formatted.is_empty() {
                format!("{}{}", prefix, config.default_file_extension)
            } else {
                formatted
            }
        }
        _ => format_optional(arg, prefix, &config.illegal_characters),
    }
}

fn segment_prefix(segment: &FilenameSegment) -> &'static str {
    match segment {
        FilenameSegment::Identifier => "@@",
        FilenameSegment::Signature => "==",
        FilenameSegment::Title => "--",
        FilenameSegment::Keywords => "__",
        FilenameSegment::Extension => ".",
    }
}

pub fn format_identifier(creation_time: DateTime<Local>, is_first: bool) -> String {
    let time = creation_time.format(DN_IDENTIFIER_FORMAT).to_string();

    match is_first {
        true => time,
        false => format!("{}{}", segment_prefix(&FilenameSegment::Identifier), time),
    }
}

fn format_optional(
    segment: &Option<String>,
    prefix: &str,
    illegal_characters: &Vec<char>,
) -> String {
    segment.as_deref().map_or(String::new(), |seg| {
        format_segment(seg, prefix, illegal_characters)
    })
}

fn format_segment(segment: &str, prefix: &str, illegal_characters: &Vec<char>) -> String {
    let out = segment
        .to_lowercase()
        .split([prefix.chars().nth(0).unwrap(), ' '].as_ref())
        .filter(|sub| !sub.is_empty())
        .map(|sub| sanitise_segment(sub, illegal_characters))
        .collect::<Vec<_>>()
        .join(&prefix[..1]);

    match out == "".to_string() {
        true => out,
        false => format!("{}{}", prefix, out),
    }
}

fn sanitise_segment(segment: &str, illegal_characters: &Vec<char>) -> String {
    const SEGMENT_SEPARATORS: [char; 4] = ['=', '-', '_', '.'];

    segment
        .chars()
        .filter(|c| !SEGMENT_SEPARATORS.contains(c))
        .filter(|c| !illegal_characters.contains(c))
        .collect()
}

//-------//
// Tests //
//-------//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_filename_generation() {
        let details = FilenameDetails {
            title_arg: Some("My Document".to_string()),
            signature_arg: None,
            keywords_arg: None,
            extension_arg: None,
            ..Default::default()
        };
        let config = FilenameConfig::default();
        let result = get_filename(&details, &config);

        assert!(
            result.contains("--my-document.txt"),
            "Expected filename to contain '--my-document.txt', but got: {:?}",
            result
        );
    }

    #[test]
    fn test_all_segments() {
        let details = FilenameDetails {
            title_arg: Some("Test Title".to_string()),
            signature_arg: Some("123".to_string()),
            keywords_arg: Some("key1_key2".to_string()),
            extension_arg: Some("md".to_string()),
            ..Default::default()
        };
        let config = FilenameConfig::default();
        let result = get_filename(&details, &config);

        assert!(
            result.contains("==123--test-title__key1_key2.md"),
            "Expected filename to contain all segments '==123--test-title__key1_key2.md', but got: {:?}",
            result
        );
    }

    #[test]
    fn test_identifier_positioning() {
        let details = FilenameDetails {
            title_arg: Some("Test".to_string()),
            signature_arg: None,
            keywords_arg: None,
            extension_arg: None,
            ..Default::default()
        };

        // Identifier first
        let config_1 = FilenameConfig::default();
        let result_1 = get_filename(&details, &config_1);

        // Identifier not first
        let config_2 = FilenameConfig {
            segment_order: [
                FilenameSegment::Title,
                FilenameSegment::Identifier,
                FilenameSegment::Signature,
                FilenameSegment::Keywords,
                FilenameSegment::Extension,
            ],
            ..Default::default()
        };
        let result2 = get_filename(&details, &config_2);

        assert!(
            !result_1.contains("@@"),
            "When identifier is first, '@@' prefix should not be present, but got filename: {:?}",
            result_1
        );
        assert!(
            result2.contains("@@"),
            "When identifier is not first, '@@' prefix should be present, but got filename: {:?}",
            result2
        );
    }

    #[test]
    fn test_segment_reordering() {
        let details = FilenameDetails {
            title_arg: Some("my title".to_string()),
            signature_arg: Some("123".to_string()),
            keywords_arg: Some("key1_key2".to_string()),
            extension_arg: None,
            ..Default::default()
        };
        let config = FilenameConfig {
            segment_order: [
                FilenameSegment::Identifier,
                FilenameSegment::Extension,
                FilenameSegment::Keywords,
                FilenameSegment::Title,
                FilenameSegment::Signature,
            ],
            ..Default::default()
        };
        let result = get_filename(&details, &config);

        assert!(
            result.contains(".txt__key1_key2--my-title==123"),
            "Expected segments in order '.txt__key1_key2--my-title==123', but got filename: {:?}",
            result
        );
    }

    #[test]
    fn test_illegal_characters() {
        let details = FilenameDetails {
            title_arg: Some("Test! @#$ Title".to_string()),
            signature_arg: Some("Auth[or](Name)".to_string()),
            keywords_arg: Some("key1&&^key2".to_string()),
            extension_arg: Some("...org".to_string()),
            ..Default::default()
        };
        let config = FilenameConfig::default();
        let result = get_filename(&details, &config);

        let illegal_chars: String = result
            .chars()
            .filter(|c| config.illegal_characters.contains(c))
            .collect();

        assert!(
            illegal_chars.is_empty(),
            "Found illegal characters in filename: '{:?}'. Illegal characters found: '{:?}'",
            result,
            illegal_chars
        );
    }

    #[test]
    fn test_empty_optional_segments() {
        let details = FilenameDetails {
            title_arg: None,
            signature_arg: None,
            keywords_arg: None,
            extension_arg: None,
            ..Default::default()
        };
        let config = FilenameConfig::default();
        let result = get_filename(&details, &config);

        assert!(
            !result.contains("=="),
            "Signature separator '==' should not be present when signature is None, but got filename: {:?}",
            result
        );
        assert!(
            !result.contains("--"),
            "Title separator '--' should not be present when title is None, but got filename: {:?}",
            result
        );
        assert!(
            !result.contains("__"),
            "Keywords separator '__' should not be present when keywords are None, but got filename: {:?}",
            result
        );
        assert!(
            result.ends_with(".txt"),
            "Filename should end with default extension '.txt', but got filename: {:?}",
            result
        );
    }

    #[test]
    fn test_segment_separator_sanitisation() {
        let details = FilenameDetails {
            title_arg: Some("first.second-third_fourth".to_string()),
            signature_arg: None,
            keywords_arg: Some("_kwrd__check".to_string()),
            extension_arg: Some(".tar.gz".to_string()),
            ..Default::default()
        };
        let config = FilenameConfig::default();
        let result = get_filename(&details, &config);

        assert!(
            result.contains("--firstsecond-thirdfourth__kwrd_check.tar.gz"),
            "Expected sanitized segments '--firstsecond-thirdfourth__kwrd_check.tar.gz', but got filename: {:?}",
            result
        );
    }

    #[test]
    fn test_case_sensitivity() {
        let details = FilenameDetails {
            title_arg: Some("UPPERCASE".to_string()),
            signature_arg: Some("MixedCase".to_string()),
            keywords_arg: Some("CamelCase".to_string()),
            extension_arg: Some("ORG".to_string()),
            ..Default::default()
        };
        let config = FilenameConfig::default();
        let result = get_filename(&details, &config);

        assert!(
            result.contains("--uppercase"),
            "Expected lowercase title '--uppercase', but got filename: {:?}",
            result
        );
        assert!(
            result.contains("==mixedcase"),
            "Expected lowercase signature '==mixedcase', but got filename: {:?}",
            result
        );
        assert!(
            result.contains("__camelcase"),
            "Expected lowercase keywords '__camelcase', but got filename: {:?}",
            result
        );
        assert!(
            result.ends_with(".org"),
            "Expected lowercase extension '.org', but got filename: {:?}",
            result
        );
    }

    #[test]
    fn test_empty_segments() {
        let details = FilenameDetails {
            title_arg: Some("-".to_string()),
            signature_arg: Some("==".to_string()),
            keywords_arg: Some("___".to_string()),
            extension_arg: Some("...".to_string()),
            ..Default::default()
        };
        let config = FilenameConfig::default();
        let result = get_filename(&details, &config);

        assert!(
            !result.contains("--"),
            "Expected no title, but got filename: {:?}",
            result
        );
        assert!(
            !result.contains("=="),
            "Expected no signature, but got filename: {:?}",
            result
        );
        assert!(
            !result.contains("__"),
            "Expected no keywords, but got filename: {:?}",
            result
        );
        assert!(
            result.ends_with(".txt"),
            "Expected lowercase extension '.txt', but got filename: {:?}",
            result
        );
    }

    #[test]
    fn test_use_existing_identifier() {
        let details = FilenameDetails {
            identifier_arg: Some("20001212T121212".to_string()),
            ..Default::default()
        };

        let config = FilenameConfig {
            regenerate_identifier: false,
            ..Default::default()
        };

        let result = get_filename(&details, &config);

        assert!(
            !result.contains("1970"),
            "Expected no unix epoch, but got filename: {:?}",
            result
        );
    }
}
