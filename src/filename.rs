use chrono::Utc;

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
    let signature = get_signature(todo!());
    let title = get_title(todo!());
    let keywords = get_keywords(todo!());
    let extension = get_extension(todo!());

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
    let time = Utc::now().format("%Y%m%dT%H%M%S").to_string();

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
    const arg_separator = ',';

    let processed = keywords
        .to_lowercase()
        .split(arg_separator)
        .flat_map(|word| word.split('_'))
        .collect::<Vec<&str>>()
        .join("_");

    format!("__{}", processed)
}

fn get_extension(extension: String) -> String {
    format!(".{}", extension)
}

// Tests

