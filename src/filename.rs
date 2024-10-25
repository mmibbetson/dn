use crate::config::FilenameConfig;
use chrono::{DateTime, Local};

#[derive(Clone, Default)]
pub struct FilenameDetails {
    existing_filename: Option<String>, // TODO: Can be either a dn-compatible or non-compatible? Rename vs. Convert?
    creation_time: DateTime<Local>,
    title_arg: Option<String>,
    signature_arg: Option<String>,
    keywords_arg: Option<String>,
    extension_arg: Option<String>,
}

#[derive(PartialEq)]
pub enum Segment {
    Identifier,
    Signature,
    Title,
    Keywords,
    Extension,
}

fn segment_prefix(segment: &Segment) -> &'static str {
    match segment {
        Segment::Identifier => "@@",
        Segment::Signature => "==",
        Segment::Title => "--",
        Segment::Keywords => "__",
        Segment::Extension => ".",
    }
}

// TODO: Implementation of renaming checks/logic.
// If preservation is on, include any non-provided existing details
// But if any segment info is provided, add or overwrite with it.
/// TODO: Documentation.
pub fn get_filename(filename_details: &FilenameDetails, config: &FilenameConfig) -> String {
    config
        .segment_order
        .iter()
        .map(|segment| match segment {
            Segment::Identifier => format_identifier(
                filename_details.creation_time,
                config.segment_order[0] == Segment::Identifier,
            ),
            Segment::Signature => format_optional(
                &filename_details.signature_arg,
                segment_prefix(segment),
                &config.illegal_characters,
            ),
            Segment::Title => format_optional(
                &filename_details.title_arg,
                segment_prefix(segment),
                &config.illegal_characters,
            ),
            Segment::Keywords => format_optional(
                &filename_details.keywords_arg,
                segment_prefix(segment),
                &config.illegal_characters,
            ),
            Segment::Extension => format_segment(
                filename_details
                    .extension_arg
                    .as_deref()
                    .unwrap_or(&config.default_file_extension),
                segment_prefix(segment),
                &config.illegal_characters,
            ),
        })
        .collect::<Vec<_>>()
        .concat()
}

fn format_identifier(creation_time: DateTime<Local>, is_first: bool) -> String {
    let time = creation_time.format("%Y%m%dT%H%M%S").to_string();

    match is_first {
        true => time,
        false => format!("{}{}", segment_prefix(&Segment::Identifier), time),
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

    format!("{}{}", prefix, out)
}

fn sanitise_segment(segment: &str, illegal_characters: &Vec<char>) -> String {
    const SEGMENT_SEPARATORS: [char; 4] = ['=', '-', '_', '.'];

    segment
        .chars()
        .filter(|c| !SEGMENT_SEPARATORS.contains(c))
        .filter(|c| !illegal_characters.contains(c))
        .collect()
}

///////////
// Tests //
///////////

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

        assert!(result.contains("--my-document.txt"));
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

        assert!(result.contains("==123--test-title__key1_key2.md"));
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
                Segment::Title,
                Segment::Identifier,
                Segment::Signature,
                Segment::Keywords,
                Segment::Extension,
            ],
            ..Default::default()
        };
        let result2 = get_filename(&details, &config_2);

        assert!(!result_1.contains("@@"));
        assert!(result2.contains("@@"));
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
                Segment::Identifier,
                Segment::Extension,
                Segment::Keywords,
                Segment::Title,
                Segment::Signature,
            ],
            ..Default::default()
        };
        let result = get_filename(&details, &config);

        assert!(result.contains(".txt__key1_key2--my-title==123"));
    }

    // TODO: test specific illegal character vec
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

        assert!(
            result
                .chars()
                .filter(|c| config.illegal_characters.contains(c))
                .collect::<String>()
                == "".to_string()
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

        assert!(!result.contains("=="));
        assert!(!result.contains("--"));
        assert!(!result.contains("__"));
        assert!(result.ends_with(".txt"));
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

        assert!(result.contains("--firstsecond-thirdfourth__kwrd_check.tar.gz"));
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

        assert!(result.contains("--uppercase"));
        assert!(result.contains("==mixedcase"));
        assert!(result.contains("__camelcase"));
        assert!(result.ends_with(".org"));
    }

    #[test]
    fn test_renaming_dn_format_file() {
        let details_1 = FilenameDetails {
            existing_filename: Some("20241025T184500==321--my-file__test.md".to_string()),
            title_arg: Some("My New Title".to_string()),
            ..Default::default()
        };
        let details_2 = FilenameDetails {
            existing_filename: Some("20241025T184500==321--my-file__test.md".to_string()),
            title_arg: Some("My New Title".to_string()),
            signature_arg: Some("123".to_string()),
            keywords_arg: Some("IMPORTANT_test".to_string()),
            extension_arg: Some("dj".to_string()),
            ..Default::default()
        };

        let config_1 = FilenameConfig::default();
        let config_2 = FilenameConfig {
            preserve_existing_details: false,
            ..Default::default()
        };

        let result_1 = get_filename(&details_1, &config_1);
        let result_2 = get_filename(&details_2, &config_2);

        assert!(result_1.contains("20241025T184500==321--my-new-title__test.md"));
        assert!(result_2.contains("==123--my-new-title__important_test.dj"));
    }

    #[test]
    fn test_renaming_non_dn_format_file() {
        let details = FilenameDetails {
            existing_filename: Some("my_file.md".to_string()),
            ..Default::default()
        };

        let config_1 = FilenameConfig::default();
        let config_2 = FilenameConfig {
            preserve_existing_details: false,
            ..Default::default()
        };

        let result_1 = get_filename(&details, &config_1);
        let result_2 = get_filename(&details, &config_2);

        assert!(result_1.contains("--myfile.md"));
        assert!(!result_2.contains("=="));
        assert!(!result_2.contains("--"));
        assert!(!result_2.contains("__"));
        assert!(result_2.contains(".txt"));
    }
}
