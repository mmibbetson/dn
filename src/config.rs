use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::directory::get_default_notes_dir;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "file")]
    pub file_config: FileConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileConfig {
    #[serde(default = "get_default_notes_dir")]
    pub directory: PathBuf,

    #[serde(default = "default_segment_order")]
    pub segment_order: [FilenameSegment; 5],

    #[serde(default = "default_file_extension")]
    pub default_extension: String,

    #[serde(default = "default_regenerate_identifier")]
    pub regenerate_identifier: bool,
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

fn default_regenerate_identifier() -> bool {
    false
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FilenameSegment {
    Identifier,
    Signature,
    Title,
    Keywords,
    Extension,
}

impl Default for FileConfig {
    fn default() -> Self {
        Self {
            directory: get_default_notes_dir(),
            segment_order: default_segment_order(),
            default_extension: default_file_extension(),
            regenerate_identifier: default_regenerate_identifier(),
        }
    }
}

// parse toml config file
pub fn read_config<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let config = toml::from_str(&contents)?;
    Ok(config)
}

// overlay config file onto default configuration
// pub fn merge_configs(, template) {}
