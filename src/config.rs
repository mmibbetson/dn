use crate::filename::Segment;
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
    pub segment_order: [Segment; 5],
    pub default_file_extension: String,
    pub illegal_characters: Vec<char>,
    pub preserve_existing_details: bool,
}

impl Default for FilenameConfig {
    fn default() -> Self {
        FilenameConfig {
            segment_order: [
                Segment::Identifier,
                Segment::Signature,
                Segment::Title,
                Segment::Keywords,
                Segment::Extension,
            ],
            default_file_extension: "txt".to_string(),
            illegal_characters: vec![
                '[', ']', '{', '}', '(', ')', '!', '@', '#', '$', '%', '^', '&', '*', '+', '\'',
                '\\', '"', '?', ',', '|', ';', ':', '~', '`', '‘', '’', '“', '”', '/', '*',
            ],
            preserve_existing_details: true,
        }
    }
}

// TODO: This is a draft.
pub enum FrontmatterOrder {
    Title,
    Date,
    Identifier,
    Signature,
    Keywords,
    Author,
}

// TODO: This is a draft.
pub enum FrontmatterDateTimeFormat {
    TwentyFourHour,
    TwelveHour
}

// TODO: Consider either using none or an explicit enabled boolean. Uniformity matters here.
pub struct FrontmatterConfig {
    pub enabled: bool,
    pub format: String,
    pub date_time_format: Option<FrontmatterDateTimeFormat>,
    pub order: FrontmatterOrder,
}

pub struct TemplateConfig {
    pub enabled: bool,
    pub default_path: PathBuf,
}
