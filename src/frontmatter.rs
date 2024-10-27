use crate::{config::FrontmatterConfig, filename::FilenameDetails};

pub enum FrontmatterSegment {
    Title,
    Date,
    Keywords,
    Identifier,
}

pub fn get_frontmatter(filename_details: &FilenameDetails, config: &FrontmatterConfig) -> String {
    todo!()
}

// TODO: Handle processing for YAML, TOML, org, plaintext frontmatter.

// TODO: Handle parsing existing frontmatter for a rename.
