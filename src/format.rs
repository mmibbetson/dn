fn sanitise() {}

fn format_title() {}

fn format_segment(segment: &str, prefix: &str, illegal_characters: &Vec<char>) -> Option<String> {
    let out = segment
        .to_lowercase()
        .split_whitespace()
        .filter(|sub| !sub.is_empty())
        .map(|sub| sanitise_segment(sub, illegal_characters))
        .collect::<Vec<_>>();

    match out.is_empty() {
        true => None,
        false => Some(out),
    }
}

fn parse_keywords(arg: String) {}

fn sanitise_segment(segment: &str, illegal_characters: &Vec<char>) -> String {
    const SEGMENT_SEPARATORS: [char; 4] = ['=', '-', '_', '.'];

    segment
        .chars()
        .filter(|c| !SEGMENT_SEPARATORS.contains(c))
        .filter(|c| !illegal_characters.contains(c))
        .collect()
}

pub const DN_IDENTIFIER_FORMAT: &str = "%Y%m%dT%H%M%S";
