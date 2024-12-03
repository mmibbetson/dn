// SPDX-FileCopyrightText: 2024 Matthew Mark Ibbetson
// SPDX-FileContributor: Matthew Mark Ibbetson
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! Parser functions for acquiring metadata from file names and front matter.

use winnow::{
    combinator::{opt, preceded, separated},
    token::{take_till, take_while},
    PResult, Parser,
};

/// Format string for use with `chrono`'s `format` function.
/// Represents the structure of a dn `Identifier`.
pub const DN_IDENTIFIER_FORMAT: &str = "%Y%m%dT%H%M%S";

/// TODO
const IDENTIFIER_PREFIX: &str = "@@";

/// TODO
const IDENTIFIER_DELIMITER: char = '@';

/// TODO
const SIGNATURE_PREFIX: &str = "==";

/// TODO
const SIGNATURE_DELIMITER: char = '=';

/// TODO
const TITLE_PREFIX: &str = "--";

/// TODO
const TITLE_DELIMITER: char = '-';

/// TODO
const KEYWORDS_PREFIX: &str = "__";

/// TODO
const KEYWORDS_DELIMITER: char = '_';

/// TODO
const EXTENSION_PREFIX: &str = ".";

/// TODO
const EXTENSION_DELIMITER: char = '.';

/// Parser to match the `Identifier` segment of a file name.
pub fn segment_identifier<'i>(input: &mut &'i str) -> PResult<String> {
    let token = preceded(
        opt(IDENTIFIER_PREFIX),
        take_while(15, |_| {
            // ERROR: This fails when identifier isn't the only part of the input.
            input.len() >= 15
                && input[0..8].chars().all(|c| c.is_ascii_digit())
                && input.chars().nth(8) == Some('T')
                && input[9..15].chars().all(|c| c.is_ascii_digit())
        }),
    )
    .parse_next(input)?
    .to_owned();

    Ok(token)
}

/// Parser to match the `Signature` segment of a file name.
pub fn segment_signature<'i>(input: &mut &'i str) -> PResult<String> {
    let token = preceded(
        SIGNATURE_PREFIX,
        take_till(1.., |c| {
            c == IDENTIFIER_DELIMITER
                || c == TITLE_DELIMITER
                || c == KEYWORDS_DELIMITER
                || c == EXTENSION_DELIMITER
        }),
    )
    .parse_next(input)?
    .to_owned();

    Ok(token)
}

/// Parser to match the `Title` segment of a file name.
pub fn segment_title<'i>(input: &mut &'i str) -> PResult<String> {
    let token = preceded(
        TITLE_PREFIX,
        take_till(1.., |c| {
            c == IDENTIFIER_DELIMITER
                || c == SIGNATURE_DELIMITER
                || c == KEYWORDS_DELIMITER
                || c == EXTENSION_DELIMITER
        }),
    )
    .parse_next(input)?
    .to_owned();

    Ok(token)
}

/// Parser to match the `Keywords` segment of a file name.
pub fn segment_keywords<'i>(input: &mut &'i str) -> PResult<Vec<String>> {
    let token = preceded(
        KEYWORDS_PREFIX,
        separated(
            1..,
            take_till(1.., |c| {
                c == IDENTIFIER_DELIMITER
                    || c == SIGNATURE_DELIMITER
                    || c == KEYWORDS_DELIMITER
                    || c == EXTENSION_DELIMITER
            }),
            KEYWORDS_DELIMITER,
        )
        .map(|v: Vec<&str>| v.into_iter().map(String::from).collect()),
    )
    .parse_next(input)?;

    Ok(token)
}

/// Parser to match the `Extension` segment of a file name.
pub fn segment_extension<'i>(input: &mut &'i str) -> PResult<String> {
    let token = preceded(
        EXTENSION_PREFIX,
        take_till(1.., |c| {
            c == IDENTIFIER_DELIMITER
                || c == SIGNATURE_DELIMITER
                || c == TITLE_DELIMITER
                || c == KEYWORDS_DELIMITER
        }),
    )
    .parse_next(input)?
    .to_owned();

    Ok(token)
}

// Text Frontmatter Parsers
pub fn text_frontmatter_suffix(input: &mut &str) -> PResult<()> {
    todo!()
}

pub fn text_frontmatter_title(input: &mut &str) -> PResult<String> {
    todo!()
}

pub fn text_frontmatter_keywords(input: &mut &str) -> PResult<Vec<String>> {
    todo!()
}

pub fn text_frontmatter_identifier(input: &mut &str) -> PResult<String> {
    todo!()
}

// YAML Frontmatter Parsers
pub fn yaml_frontmatter_container(input: &mut &str) -> PResult<()> {
    todo!()
}

pub fn yaml_frontmatter_title(input: &mut &str) -> PResult<String> {
    todo!()
}

pub fn yaml_frontmatter_keywords(input: &mut &str) -> PResult<Vec<String>> {
    todo!()
}

pub fn yaml_frontmatter_identifier(input: &mut &str) -> PResult<String> {
    todo!()
}

// TOML Frontmatter Parsers
pub fn toml_frontmatter_container(input: &mut &str) -> PResult<()> {
    todo!()
}

pub fn toml_frontmatter_title(input: &mut &str) -> PResult<String> {
    todo!()
}

pub fn toml_frontmatter_keywords(input: &mut &str) -> PResult<Vec<String>> {
    todo!()
}

pub fn toml_frontmatter_identifier(input: &mut &str) -> PResult<String> {
    todo!()
}

// JSON Frontmatter Parsers
pub fn json_frontmatter_prefix(input: &mut &str) -> PResult<()> {
    todo!()
}

pub fn json_frontmatter_suffix(input: &mut &str) -> PResult<()> {
    todo!()
}

pub fn json_frontmatter_title(input: &mut &str) -> PResult<String> {
    todo!()
}

pub fn json_frontmatter_keywords(input: &mut &str) -> PResult<Vec<String>> {
    todo!()
}

pub fn json_frontmatter_identifier(input: &mut &str) -> PResult<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    mod filename_parsers {
        use std::vec;

        use crate::parsers::{
            segment_extension, segment_identifier, segment_keywords, segment_signature,
            segment_title,
        };

        #[test]
        fn identifier() {
            // Arrange
            let mut input_valid_1 = "20241212T121212";
            let mut input_valid_2 = "@@20241212T121212";
            let mut input_invalid = "20241212@121212";
            let expected = "20241212T121212".to_owned();

            // Act
            let result_1 = segment_identifier(&mut input_valid_1);
            let result_2 = segment_identifier(&mut input_valid_2);
            let result_3 = segment_identifier(&mut input_invalid);

            // Assert
            assert_eq!(expected, result_1.unwrap());
            assert_eq!(expected, result_2.unwrap());
            assert!(result_3.is_err());
        }

        #[test]
        fn signature() {
            // Arrange
            let mut input_valid = "==1ab";
            let mut input_invalid = "1ab";
            let expected = "1ab".to_owned();

            // Act
            let result_1 = segment_signature(&mut input_valid);
            let result_2 = segment_signature(&mut input_invalid);

            // Assert
            assert_eq!(expected, result_1.unwrap());
            assert!(result_2.is_err());
        }

        #[test]
        fn title() {
            // Arrange
            let mut input_valid = "--this-title";
            let mut input_invalid = "this-title";
            let expected = "this-title".to_owned();

            // Act
            let result_1 = segment_title(&mut input_valid);
            let result_2 = segment_title(&mut input_invalid);

            // Assert
            assert_eq!(expected, result_1.unwrap());
            assert!(result_2.is_err());
        }

        #[test]
        fn keywords() {
            // Arrange
            let mut input_valid = "__foo_bar";
            let mut input_invalid = "foo_bar";
            let expected = vec!["foo", "bar"];

            // Act
            let result_1 = segment_keywords(&mut input_valid);
            let result_2 = segment_keywords(&mut input_invalid);

            // Assert
            assert_eq!(expected, result_1.unwrap());
            assert!(result_2.is_err());
        }

        #[test]
        fn extension() {
            // Arrange
            let mut input_valid = ".txt";
            let mut input_invalid = "__some_keys";
            let expected = "txt".to_owned();

            // Act
            let result_1 = segment_extension(&mut input_valid);
            let result_2 = segment_extension(&mut input_invalid);

            // Assert
            assert_eq!(expected, result_1.unwrap());
            assert!(result_2.is_err());
        }
    }

    mod text_frontmatter_parsers {}

    mod yaml_frontmatter_parsers {}

    mod toml_frontmatter_parsers {}

    mod json_frontmatter_parsers {}
}
