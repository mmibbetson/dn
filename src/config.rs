use crate::filename::Segment;

const DEFAULT_SEGMENT_ORDER: [Segment; 5] = [
    Segment::Identifier,
    Segment::Signature,
    Segment::Title,
    Segment::Keywords,
    Segment::Extension,
];

pub const DEFAULT_ILLEGAL_CHARACTERS: [char; 31] = [
    '[', ']', '{', '}', '(', ')', '!', '@', '#', '$', '%', '^', '&', '*', '+', '\'', '\\', '"',
    '?', ',', '|', ';', ':', '~', '`', '‘', '’', '“', '”', '/', '*',
];
const DEFAULT_FILE_EXTENSION: &str = "txt";

pub struct DnConfig {
    pub directory_config: DirectoryConfig,
    pub filename_config: FilenameConfig,
    pub frontmatter_config: FrontmatterConfig,
    pub template_config: TemplateConfig,
}

pub struct DirectoryConfig {}

pub struct FilenameConfig {
    pub segment_order: [Segment; 5],
    pub default_file_extension: String,
    pub illegal_characters: Vec<char>,
}

pub struct FrontmatterConfig {}

pub struct TemplateConfig {}

pub struct RenameConfig {
    pub preserve_extension: bool,
    pub preserve_identifier: bool,
}
