//! TODO

/// TODO
// TODO: this can be used abstractly as-is
// In main, when renaming, we parse the file contents and separate the frontmatter section
// and the content section. the content section becomes the content equivalent to the template in
// a new note.
pub fn concatenate_file_content(
    frontmatter: Option<impl AsRef<[u8]>>,
    content: Option<impl AsRef<[u8]>>,
) -> Vec<u8> {
    let frontmatter_vec = frontmatter.map(|f| f.as_ref().to_vec());
    let content_vec = content.map(|c| c.as_ref().to_vec());

    match (frontmatter_vec, content_vec) {
        (Some(f), Some(c)) => [f, c].concat(),
        (Some(f), None) => f,
        (None, Some(c)) => c,
        (None, None) => Vec::new(),
    }
}

///////////
// Tests //
///////////

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn concatenate_with_frontmatter_no_template() {
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

    #[test]
    fn concatenate_with_template_no_frontmatter() {
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

    #[test]
    fn concatenate_with_frontmatter_and_template() {
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

    #[test]
    fn concatenate_with_neither_frontmatter_nor_template() {
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