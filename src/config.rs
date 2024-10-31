use crate::{
    filename::FilenameSegment,
    frontmatter::{FrontmatterFormat, FrontmatterSegment},
};
use std::path::PathBuf;

pub struct DnConfig {
    pub directory_config: DirectoryConfig,
    pub filename_config: FilenameConfig,
    pub frontmatter_config: FrontmatterConfig,
    pub template_config: TemplateConfig,
}

pub struct DirectoryConfig {
    pub dn_directory: PathBuf,
}

pub struct FilenameConfig {
    pub segment_order: [FilenameSegment; 5],
    pub default_file_extension: String,
    pub illegal_characters: Vec<char>,
}

impl Default for FilenameConfig {
    fn default() -> Self {
        FilenameConfig {
            segment_order: [
                FilenameSegment::Identifier,
                FilenameSegment::Signature,
                FilenameSegment::Title,
                FilenameSegment::Keywords,
                FilenameSegment::Extension,
            ],
            default_file_extension: "txt".to_string(),
            illegal_characters: vec![
                '[', ']', '{', '}', '(', ')', '!', '@', '#', '$', '%', '^', '&', '*', '+', '\'',
                '\\', '"', '?', ',', '|', ';', ':', '~', '`', '‘', '’', '“', '”', '/', '*',
            ],
        }
    }
}

pub struct FrontmatterConfig {
    pub enabled: bool,
    pub rewrite: bool,
    pub format: FrontmatterFormat,
    pub date_time_format: FrontmatterDateTimeFormat,
    pub segment_order: Vec<FrontmatterSegment>,
}

impl Default for FrontmatterConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            rewrite: false,
            format: FrontmatterFormat::Text,
            date_time_format: FrontmatterDateTimeFormat::TwentyFourHour,
            segment_order: vec![
                FrontmatterSegment::Title,
                FrontmatterSegment::Date,
                FrontmatterSegment::Keywords,
                FrontmatterSegment::Identifier,
            ],
        }
    }
}

#[derive(Clone, Copy)]
pub enum FrontmatterDateTimeFormat {
    TwentyFourHour,
    TwelveHour,
    None,
}

pub struct TemplateConfig {
    pub enabled: bool,
    pub default_path: PathBuf,
}

// TODO: When creating the config struct instances, we resolve the hierarchy:
// Command Line Option > Config File > Default
