// TODO: Parse existing filename if provided to provide a filedetails struct for use elsewhere.

// regex match group 
fn parse_identifier(filename: &str) -> Option<String> {
    // Match 8 digits, the letter 'T', then 6 digits.
    let pattern = r"(\b[0-9]{8}T[0-9]{6}\b)";
    todo!()
}

fn parse_signature(filename: &str) -> Option<String> {
    let pattern = r"(==[^\@\-\_\.]*)";
    todo!()
}

fn parse_title(filename: &str) -> Option<String> {
    let pattern = r"(--[^\@\=\_\.]*)";
    todo!()
}

fn parse_keywords(filename: &str) -> Option<String> {
    let pattern = r"(__[^\@\=\-\.]*)";
    todo!()
}

fn parse_extension(filename: &str) -> Option<String> {
    let pattern = r"(\.[^\@\=\-\_]*)";
    todo!()
}
