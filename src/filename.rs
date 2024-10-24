use chrono::Local;

#[derive(PartialEq)]
pub enum Segment {
    Identifier,
    Signature,
    Title,
    Keywords,
    Extension,
}

const SPLITTING_CHARACTERS: [char; 6] = ['@', '=', '-', '_', '.', ' '];

pub fn get_filename(order: [Segment; 5]) -> String {
    let identifier_is_first = order[0] == Segment::Identifier;
    let identifier = format_identifier(identifier_is_first);
    let signature = format_signature(Some("123-4=34".to_owned()));
    let title = format_title("This is my title-thing!".to_string());
    let keywords = format_keywords(Some("r3 0 12,#(-34v_,_-=".to_owned()));
    let extension = format_extension(Some("scm".to_string()));

    // TODO: Ensure that '.' only appears a single time (the last time)

    order
        .into_iter()
        .map(|segment| match segment {
            Segment::Identifier => identifier.clone(),
            Segment::Signature => signature.clone().unwrap_or_default(),
            Segment::Title => title.clone(),
            Segment::Keywords => keywords.clone().unwrap_or_default(),
            Segment::Extension => extension.clone(),
        })
        .collect::<Vec<String>>()
        .join("")
}

fn format_identifier(is_first: bool) -> String {
    let time = Local::now().format("%Y%m%dT%H%M%S").to_string();

    if is_first {
        time
    } else {
        format!("@@{}", time)
    }
}

// TODO: You can actually have multiple parts with single = separator.
fn format_signature(signature: Option<String>) -> Option<String> {
    fn process(sig: String) -> String {
        let out = sig
            .to_lowercase()
            .split(&SPLITTING_CHARACTERS)
            .filter(|c| !c.is_empty())
            .collect::<Vec<&str>>()
            .join("=");

        format!("=={}", out)
    }

    signature.map(process)
}

fn format_title(title: String) -> String {
    let processed = title
        .to_lowercase()
        .split(&SPLITTING_CHARACTERS)
        .filter(|c| !c.is_empty())
        .collect::<Vec<&str>>()
        .join("-");

    format!("--{}", processed)
}

// keywords is provided as a comma-separated list of keywords.
// TODO: join on spaces and other delimiters - = @
fn format_keywords(keywords: Option<String>) -> Option<String> {
    // NOTE: This is for the keywords as supplied by the user.
    const KEYWORD_ARG_SEPARATOR: char = ',';

    fn process(key: String) -> String {
        let out = key
            .to_lowercase()
            .split(KEYWORD_ARG_SEPARATOR)
            .flat_map(|word| word.split(&SPLITTING_CHARACTERS))
            .filter(|c| !c.is_empty())
            .collect::<Vec<&str>>()
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
            .split(&SPLITTING_CHARACTERS)
            .filter(|c| !c.is_empty())
            .collect::<Vec<&str>>()
            .concat();

        format!(".{}", out)
    }

    extension.map(process).unwrap_or(DEFAULT_FILE_EXTENSION.to_owned())
}

// Tests
