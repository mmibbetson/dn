use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Error;
use serde::{Deserialize, Serialize};

use crate::directory::get_default_notes_dir;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub file: FileConfig,
    pub frontmatter: FrontmatterConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileConfig {
    #[serde(default = "get_default_notes_dir")]
    pub directory: PathBuf,

    #[serde(default = "default_segment_order")]
    pub segment_order: [FilenameSegment; 5],

    #[serde(default = "default_file_extension")]
    pub default_extension: String,

    #[serde(default = "r#false")]
    pub regenerate_identifier: bool,

    #[serde(default = "none")]
    pub template_path: Option<PathBuf>,

    #[serde(default = "default_illegal_characters")]
    pub illegal_characters: Vec<char>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FrontmatterConfig {
    #[serde(default = "r#false")]
    pub enabled: bool,

    #[serde(default = "r#true")]
    pub rewrite: bool,

    #[serde(default = "default_frontmatter_format")]
    pub format: FrontmatterFormat,

    #[serde(default = "default_frontmatter_time_format")]
    pub time_style: FrontmatterTimeFormat,

    #[serde(default = "default_frontmatter_segment_order")]
    pub order: Vec<FrontmatterSegment>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FrontmatterFormat {
    Text,
    YAML,
    TOML,
    Org,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FrontmatterTimeFormat {
    Hour24,
    Hour12,
    None,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FrontmatterSegment {
    Title,
    Date,
    Keywords,
    Identifier,
}

fn default_segment_order() -> [FilenameSegment; 5] {
    [
        FilenameSegment::Identifier,
        FilenameSegment::Signature,
        FilenameSegment::Title,
        FilenameSegment::Keywords,
        FilenameSegment::Extension,
    ]
}

fn default_file_extension() -> String {
    "txt".to_owned()
}

fn default_frontmatter_format() -> FrontmatterFormat {
    FrontmatterFormat::Text
}

fn default_frontmatter_time_format() -> FrontmatterTimeFormat {
    FrontmatterTimeFormat::Hour24
}

fn default_frontmatter_segment_order() -> Vec<FrontmatterSegment> {
    vec![
        FrontmatterSegment::Title,
        FrontmatterSegment::Date,
        FrontmatterSegment::Keywords,
        FrontmatterSegment::Identifier,
    ]
}

fn r#false() -> bool {
    false
}

fn r#true() -> bool {
    true
}

fn none<T>() -> Option<T> {
    None
}

fn default_illegal_characters() -> Vec<char> {
    vec![
        '[', ']', '{', '}', '(', ')', '!', '#', '$', '%', '^', '&', '*', '+', '\'', '\\', '"', '?',
        ',', '|', ';', ':', '~', '`', '‘', '’', '“', '”', '/', '*', ' ', '@', '=', '-', '_', '.',
    ]
}

#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FilenameSegment {
    Identifier,
    Signature,
    Title,
    Keywords,
    Extension,
}

impl Default for FilenameSegment {
    fn default() -> Self {
        FilenameSegment::Identifier
    }
}

impl Default for FileConfig {
    fn default() -> Self {
        Self {
            directory: get_default_notes_dir(),
            segment_order: default_segment_order(),
            default_extension: default_file_extension(),
            regenerate_identifier: r#false(),
            template_path: none::<PathBuf>(),
            illegal_characters: default_illegal_characters(),
        }
    }
}

impl Default for FrontmatterConfig {
    fn default() -> Self {
        Self {
            enabled: Default::default(),
            rewrite: Default::default(),
            format: default_frontmatter_format(),
            time_style: default_frontmatter_time_format(),
            order: Default::default(),
        }
    }
}

// parse toml config file
pub fn read_config<P: AsRef<Path>>(path: P) -> Result<Config, Error> {
    let contents = fs::read_to_string(path)?;
    let config = toml::from_str(&contents)?;
    Ok(config)
}

///////////
// Tests //
///////////

#[cfg(test)]
mod tests {}
