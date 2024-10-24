use chrono::Local;

#[derive(PartialEq)]
pub enum Segment {
    Identifier,
    Signature,
    Title,
    Keywords,
    Extension,
}

pub fn get_filename(order: Vec<Segment>) -> String {
    let identifier_is_first = order[0] == Segment::Identifier;
    let identifier = get_identifier(identifier_is_first);
    let signature = get_signature("123".to_string());
    let title = get_title("This is my title-thing!".to_string());
    let keywords = get_keywords("foo,bar".to_string());
    let extension = get_extension("txt".to_string());

    order
        .into_iter()
        .map(|segment| match segment {
            Segment::Identifier => identifier.clone(),
            Segment::Signature => signature.clone(),
            Segment::Title => title.clone(),
            Segment::Keywords => keywords.clone(),
            Segment::Extension => extension.clone(),
        })
        .collect::<Vec<String>>()
        .join("")
}

fn get_identifier(is_first: bool) -> String {
    let time = Local::now().format("%Y%m%dT%H%M%S").to_string();

    if is_first {
        time
    } else {
        format!("@@{}", time)
    }
}

fn get_signature(signature: String) -> String {
    format!("=={}", signature)
}

fn get_title(title: String) -> String {
    let processed = title
        .to_lowercase()
        .split_whitespace()
        .flat_map(|word| word.split('-'))
        .collect::<Vec<&str>>()
        .join("-");

    format!("--{}", processed)
}

// keywords is provided as a comma-separated list of keywords.
fn get_keywords(keywords: String) -> String {
    // NOTE: This is for the keywords as supplied by the user.
    const ARG_SEPARATOR: char = ',';

    let processed = keywords
        .to_lowercase()
        .split(ARG_SEPARATOR)
        .flat_map(|word| word.split('_'))
        .collect::<Vec<&str>>()
        .join("_");

    format!("__{}", processed)
}

fn get_extension(extension: String) -> String {
    format!(".{}", extension)
}

// Tests

