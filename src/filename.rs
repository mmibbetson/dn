use crate::config::FilenameConfig;
use chrono::Local;

#[derive(Clone)]
pub struct FilenameDetails {
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

/// TODO: Documentation.
pub fn get_filename(filename_details: &FilenameDetails, config: &FilenameConfig) -> String {
    config
        .segment_order
        .iter()
        .map(|segment| match segment {
            Segment::Identifier => {
                format_identifier(config.segment_order[0] == Segment::Identifier)
            }
            Segment::Signature => format_optional(
                &filename_details.signature_arg,
                "==",
                &config.illegal_characters,
            ),
            Segment::Title => {
                format_optional(&filename_details.title_arg, "--", &config.illegal_characters)
            }
            Segment::Keywords => format_optional(
                &filename_details.keywords_arg,
                "__",
                &config.illegal_characters,
            ),
            Segment::Extension => format_segment(
                filename_details
                    .extension_arg
                    .as_deref()
                    .unwrap_or(&config.default_file_extension),
                ".",
                &config.illegal_characters,
            ),
        })
        .collect::<Vec<_>>()
        .concat()
}

/// Generate a formatted identifier segment by getting the local time.
fn format_identifier(is_first: bool) -> String {
    let time = Local::now().format("%Y%m%dT%H%M%S").to_string();

    if is_first {
        time
    } else {
        format!("@@{}", time)
    }
}

/// Generate a formatted non-identifier segment.
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
    use crate::config::DEFAULT_ILLEGAL_CHARACTERS;
    use super::*;

    #[test]
    fn test_basic_filename_generation() {
        let details = FilenameDetails {
            title_arg: Some("My Document".to_string()),
            signature_arg: None,
            keywords_arg: None,
            extension_arg: None,
        };

        let config = FilenameConfig {
            segment_order: [
                Segment::Identifier,
                Segment::Signature,
                Segment::Title,
                Segment::Keywords,
                Segment::Extension,
            ],
            default_file_extension: "txt".to_string(),
            illegal_characters: Vec::new(),
        };

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
        };

        let config = FilenameConfig {
            segment_order: [
                Segment::Identifier,
                Segment::Signature,
                Segment::Title,
                Segment::Keywords,
                Segment::Extension,
            ],
            default_file_extension: "txt".to_string(),
            illegal_characters: Vec::new(),
        };

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
        };

        // Identifier first
        let config_1 = FilenameConfig {
            segment_order: [
                Segment::Identifier,
                Segment::Signature,
                Segment::Title,
                Segment::Keywords,
                Segment::Extension,
            ],
            default_file_extension: "txt".to_string(),
            illegal_characters: Vec::new(),
        };

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
            default_file_extension: "txt".to_string(),
            illegal_characters: Vec::new(),
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
        };

        let config = FilenameConfig {
            segment_order: [
                Segment::Identifier,
                Segment::Extension,
                Segment::Keywords,
                Segment::Title,
                Segment::Signature,
            ],
            default_file_extension: "txt".to_string(),
            illegal_characters: Vec::new(),
        };

        let result = get_filename(&details, &config);

        assert!(result.contains(".txt__key1_key2--my-title==123"));
    }

    #[test]
    fn test_illegal_characters() {
        let details = FilenameDetails {
            title_arg: Some("Test! @#$ Title".to_string()),
            signature_arg: Some("Auth[or](Name)".to_string()),
            keywords_arg: Some("key1&&^key2".to_string()),
            extension_arg: Some("...org".to_string()),
        };

        let config = FilenameConfig {
            segment_order: [
                Segment::Identifier,
                Segment::Signature,
                Segment::Title,
                Segment::Keywords,
                Segment::Extension,
            ],
            default_file_extension: "txt".to_string(),
            illegal_characters: DEFAULT_ILLEGAL_CHARACTERS.to_vec(),
        };

        let result = get_filename(&details, &config);

        assert!(
            result
                .chars()
                .filter(|c| DEFAULT_ILLEGAL_CHARACTERS.contains(c))
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
        };

        let config = FilenameConfig {
            segment_order: [
                Segment::Identifier,
                Segment::Signature,
                Segment::Title,
                Segment::Keywords,
                Segment::Extension,
            ],
            default_file_extension: "txt".to_string(),
            illegal_characters: Vec::new(),
        };

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
        };

        let config = FilenameConfig {
            segment_order: [
                Segment::Identifier,
                Segment::Signature,
                Segment::Title,
                Segment::Keywords,
                Segment::Extension,
            ],
            default_file_extension: "txt".to_string(),
            illegal_characters: Vec::new(),
        };

        let result = get_filename(&details, &config);

        assert!(result.contains("--firstsecond-thirdfourth__kwrd_check.tar.gz"));
    }

    #[test]
    fn test_case_sensitivity() {
        let details = FilenameDetails {
            title_arg: Some("UPPERCASE".to_string()),
            signature_arg: Some("MixedCase".to_string()),
            keywords_arg: Some("CamelCase".to_string()),
            extension_arg: Some("PDF".to_string()),
        };

        let config = FilenameConfig {
            segment_order: [
                Segment::Identifier,
                Segment::Signature,
                Segment::Title,
                Segment::Keywords,
                Segment::Extension,
            ],
            default_file_extension: "txt".to_string(),
            illegal_characters: Vec::new(),
        };

        let result = get_filename(&details, &config);
        println!("{}", result);

        assert!(result.contains("--uppercase"));
        assert!(result.contains("==mixedcase"));
        assert!(result.contains("__camelcase"));
        assert!(result.ends_with(".pdf"));
    }
}
