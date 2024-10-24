use chrono::Local;

pub struct FilenameDetails {
    title_arg: String,
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
const ILLEGAL_CHARACTERS: [char; 33] = [
    ' ', '[', ']', '{', '}', '(', ')', '!', '@', '#', '$', '%', '^', '&', '*', '+', '\'', '\\',
    '"', '?', ',', '.', '|', ';', ':', '~', '`', '‘', '’', '“', '”', '/', '*',
];

pub fn get_filename(filename_details: FilenameDetails, order: [Segment; 5]) -> String {
    let identifier_is_first = order[0] == Segment::Identifier;
    let identifier = format_identifier(identifier_is_first);
    let signature = format_signature(filename_details.signature_arg);
    let title = format_title(filename_details.title_arg);
    let keywords = format_keywords(filename_details.keywords_arg);
    let extension = format_extension(filename_details.extension_arg);

    order
        .iter()
        .map(|segment| match segment {
            Segment::Identifier => identifier.to_owned(),
            Segment::Signature => signature.to_owned().unwrap_or_default(),
            Segment::Title => title.to_owned(),
            Segment::Keywords => keywords.to_owned().unwrap_or_default(),
            Segment::Extension => extension.to_owned(),
        })
        .collect::<Vec<String>>()
        .concat()
}

fn format_identifier(is_first: bool) -> String {
    let time = Local::now().format("%Y%m%dT%H%M%S").to_string();

    if is_first {
        time
    } else {
        format!("@@{}", time)
    }
}

fn format_signature(signature: Option<String>) -> Option<String> {
    fn process(sig: String) -> String {
        let out = sig
            .to_lowercase()
            .split('=')
            .map(sanitise_segment)
            .collect::<Vec<String>>()
            .join("=");

        format!("=={}", out)
    }

    signature.map(process)
}

fn format_title(title: String) -> String {
    let processed = title
        .to_lowercase()
        .split('-')
        .map(sanitise_segment)
        .collect::<Vec<String>>()
        .join("-");

    format!("--{}", processed)
}

fn format_keywords(keywords: Option<String>) -> Option<String> {
    fn process(key: String) -> String {
        let out = key
            .to_lowercase()
            .split('_')
            .map(sanitise_segment)
            .collect::<Vec<String>>()
            .join("_");

        format!("__{}", out)
    }

    keywords.map(process)
}

fn format_extension(extension: Option<String>) -> String {
    const DEFAULT_FILE_EXTENSION: &str = "txt";

    fn process(ext: String) -> String {
        let out = ext
            .to_lowercase()
            .split('.')
            .map(sanitise_segment)
            .collect::<Vec<String>>()
            .join(".");

        format!(".{}", out)
    }

    extension
        .map(process)
        .unwrap_or(DEFAULT_FILE_EXTENSION.to_owned())
}

fn sanitise_segment(segment: &str) -> String {
    segment
        .chars()
        .filter(|c| !SEGMENT_SEPARATORS.contains(c))
        .filter(|c| !ILLEGAL_CHARACTERS.contains(c))
        .collect()
}

// Tests
