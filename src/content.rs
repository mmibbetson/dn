//! TODO

/// TODO
pub fn concatenate_file_content(
    frontmatter: Option<impl AsRef<[u8]>>,
    template: Option<impl AsRef<[u8]>>,
) -> Vec<u8> {
    let front_vec = frontmatter.map(|f| f.as_ref().to_vec());
    let templ_vec = template.map(|t| t.as_ref().to_vec());

    match (front_vec, templ_vec) {
        (Some(f), Some(t)) => [f, t].concat(),
        (Some(f), None) => f,
        (None, Some(t)) => t,
        (None, None) => Vec::new(),
    }
}

/// TODO
// takes the preexisting content, if new frontmatter, prepend.
// if overwriting frontmatter, remove old and prepend new.
pub fn concatenate_rename_content(
    frontmatter: Option<impl AsRef<[u8]>>,
    template: impl AsRef<[u8]>,
) -> Vec<u8> {
    let front_vec = frontmatter.map(|f| f.as_ref().to_vec());
    let templ_vec = template.as_ref().to_vec();

    todo!()
}

///////////
// Tests //
///////////

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn derive_datetime_with_identifier() {
        // Arrange
        let input = todo!();
        let expected = todo!();

        // Act
        let result = todo!();

        // Assert
        assert_eq!(
            expected, result,
            "Input: {:#?}\nExpected datetime: {:#?}\nReceived: {:#?}",
            input, expected, result
        );
    }
}