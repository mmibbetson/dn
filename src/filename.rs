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

const SEGMENT_SEPARATORS: [char; 4] = ['=', '-', '_', '.'];

// TODO: Illegal characters and default file extension will be configurable.
// Therefore, we will want to receive them from the config struct.
const DEFAULT_ILLEGAL_CHARACTERS: [char; 31] = [
    '[', ']', '{', '}', '(', ')', '!', '@', '#', '$', '%', '^', '&', '*', '+', '\'', '\\', '"',
    '?', ',', '|', ';', ':', '~', '`', '‘', '’', '“', '”', '/', '*',
];
const DEFAULT_FILE_EXTENSION: &str = "txt";

pub fn get_filename(filename_details: FilenameDetails, order: &[Segment; 5]) -> String {
    let identifier = format_identifier(order[0] == Segment::Identifier);
    let signature = map_format(filename_details.signature_arg, "==".to_string());
    let title = map_format(filename_details.title_arg, "--".to_string());
    let keywords = map_format(filename_details.keywords_arg, "__".to_string());
    let extension = format_segment(
        filename_details.extension_arg.unwrap_or("txt".to_string()),
        ".".to_string(),
    );

    order
        .iter()
        .map(|segment| match segment {
            Segment::Identifier => identifier.clone(),
            Segment::Signature => signature.clone().unwrap_or_default(),
            Segment::Title => title.clone().unwrap_or_default(),
            Segment::Keywords => keywords.clone().unwrap_or_default(),
            Segment::Extension => extension.clone(),
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
fn map_format(segment: Option<String>, prefix: String) -> Option<String> {
    segment.map(|seg| format_segment(seg, prefix))
}

fn format_segment(segment: String, prefix: String) -> String {
    let out = segment
        .to_lowercase()
        .split([prefix.chars().nth(0).unwrap(), ' '].as_ref())
        .filter(|sub| !sub.is_empty())
        .map(sanitise_segment)
        .collect::<Vec<_>>()
        .join(&prefix[..1]);

    format!("{}{}", prefix, out)
}

fn sanitise_segment(segment: &str) -> String {
    segment
        .chars()
        .filter(|c| !SEGMENT_SEPARATORS.contains(c))
        .filter(|c| !DEFAULT_ILLEGAL_CHARACTERS.contains(c))
        .collect()
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_basic_filename_generation() {
        let details = FilenameDetails {
            title_arg: Some("My Document".to_string()),
            signature_arg: None,
            keywords_arg: None,
            extension_arg: None,
        };

        let order = [
            Segment::Identifier,
            Segment::Signature,
            Segment::Title,
            Segment::Keywords,
            Segment::Extension,
        ];

        let result = get_filename(details, &order);
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

        let order = [
            Segment::Identifier,
            Segment::Signature,
            Segment::Title,
            Segment::Keywords,
            Segment::Extension,
        ];

        let result = get_filename(details, &order);
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
        let order1 = [
            Segment::Identifier,
            Segment::Title,
            Segment::Signature,
            Segment::Keywords,
            Segment::Extension,
        ];
        let result1 = get_filename(details.clone(), &order1);

        // Identifier not first
        let order2 = [
            Segment::Title,
            Segment::Identifier,
            Segment::Signature,
            Segment::Keywords,
            Segment::Extension,
        ];
        let result2 = get_filename(details.clone(), &order2);

        assert!(!result1.contains("@@"));
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

        let order = [
            Segment::Identifier,
            Segment::Extension,
            Segment::Keywords,
            Segment::Title,
            Segment::Signature,
        ];
        
        let result = get_filename(details.clone(), &order);

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

        let order = [
            Segment::Identifier,
            Segment::Signature,
            Segment::Title,
            Segment::Keywords,
            Segment::Extension,
        ];

        let result = get_filename(details, &order);

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

        let order = [
            Segment::Identifier,
            Segment::Signature,
            Segment::Title,
            Segment::Keywords,
            Segment::Extension,
        ];

        let result = get_filename(details, &order);

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

        let order = [
            Segment::Identifier,
            Segment::Title,
            Segment::Keywords,
            Segment::Signature,
            Segment::Extension,
        ];

        let result = get_filename(details, &order);

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

        let order = [
            Segment::Identifier,
            Segment::Signature,
            Segment::Title,
            Segment::Keywords,
            Segment::Extension,
        ];

        let result = get_filename(details, &order);

        assert!(result.contains("--uppercase"));
        assert!(result.contains("==mixedcase"));
        assert!(result.contains("__camelcase"));
        assert!(result.ends_with(".pdf"));
    }
}
