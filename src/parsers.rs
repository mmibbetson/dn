// SPDX-FileCopyrightText: 2024 Matthew Mark Ibbetson
// SPDX-FileContributor: Matthew Mark Ibbetson
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! Parser functions for managing front matter.

use winnow::{PResult};

/// Format string for use with `chrono`'s `format` function.
/// Represents the structure of a dn `Identifier`.
pub const DN_IDENTIFIER_FORMAT: &str = "%Y%m%dT%H%M%S";

// Text Frontmatter Parsers
fn text_frontmatter_suffix(input: &mut &str) -> PResult<()> {
    todo!()
}

fn text_frontmatter_title(input: &mut &str) -> PResult<String> {
    todo!()
}

fn text_frontmatter_keywords(input: &mut &str) -> PResult<Vec<String>> {
    todo!()
}

fn text_frontmatter_identifier(input: &mut &str) -> PResult<String> {
    todo!()
}

// YAML Frontmatter Parsers
fn yaml_frontmatter_container(input: &mut &str) -> PResult<()> {
    todo!()
}

fn yaml_frontmatter_title(input: &mut &str) -> PResult<String> {
    todo!()
}

fn yaml_frontmatter_keywords(input: &mut &str) -> PResult<Vec<String>> {
    todo!()
}

fn yaml_frontmatter_identifier(input: &mut &str) -> PResult<String> {
    todo!()
}

// TOML Frontmatter Parsers
fn toml_frontmatter_container(input: &mut &str) -> PResult<()> {
    todo!()
}

fn toml_frontmatter_title(input: &mut &str) -> PResult<String> {
    todo!()
}

fn toml_frontmatter_keywords(input: &mut &str) -> PResult<Vec<String>> {
    todo!()
}

fn toml_frontmatter_identifier(input: &mut &str) -> PResult<String> {
    todo!()
}

// JSON Frontmatter Parsers
fn json_frontmatter_prefix(input: &mut &str) -> PResult<()> {
    todo!()
}

fn json_frontmatter_suffix(input: &mut &str) -> PResult<()> {
    todo!()
}

fn json_frontmatter_title(input: &mut &str) -> PResult<String> {
    todo!()
}

fn json_frontmatter_keywords(input: &mut &str) -> PResult<Vec<String>> {
    todo!()
}

fn json_frontmatter_identifier(input: &mut &str) -> PResult<String> {
    todo!()
}

#[cfg(test)]
mod tests {}
