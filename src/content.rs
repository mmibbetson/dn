// SPDX-FileCopyrightText: 2024 Matthew Mark Ibbetson
// SPDX-FileContributor: Matthew Mark Ibbetson
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! Utilities for handling internal file contents.

/// Concatenates the frontmatter and content, with front matter first. If either portion is `None`, it is ignored.
///
/// # Example
/// ```
/// let frontmatter = Some(b"---\ntitle: Example\n---\n\n");
/// let content = Some(b"Hello, world!");
/// let result = concatenate_file_content(frontmatter, content);
/// assert_eq!(result, b"---\ntitle: Example\n---\n\nHello, world!");
/// ```
pub fn concatenate_file_content<T: AsRef<[u8]>>(
    frontmatter: Option<T>,
    content: Option<T>,
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

    #[test]
    fn concatenate_with_frontmatter_no_template() {
        // Arrange
        let input = (Some("hello\n---\n\n"), None::<&str>);
        let (frontmatter, content) = input;
        let expected = Vec::from("hello\n---\n\n");

        // Act
        let result = concatenate_file_content(frontmatter, content);

        // Assert
        assert_eq!(expected, result,);
    }

    #[test]
    fn concatenate_with_template_no_frontmatter() {
        // Arrange
        let input = (None::<&str>, Some("world!"));
        let (frontmatter, content) = input;
        let expected = Vec::from("world!");

        // Act
        let result = concatenate_file_content(frontmatter, content);

        // Assert
        assert_eq!(expected, result,);
    }

    #[test]
    fn concatenate_with_frontmatter_and_template() {
        // Arrange
        let input = (Some("hello\n---\n\n"), Some("world!"));
        let (frontmatter, content) = input;
        let expected = Vec::from("hello\n---\n\nworld!");

        // Act
        let result = concatenate_file_content(frontmatter, content);

        // Assert
        assert_eq!(expected, result,);
    }

    #[test]
    fn concatenate_with_neither_frontmatter_nor_template() {
        // Arrange
        let input = (None::<&str>, None::<&str>);
        let (frontmatter, content) = input;
        let expected = Vec::from("");

        // Act
        let result = concatenate_file_content(frontmatter, content);

        // Assert
        assert_eq!(expected, result,);
    }
}
