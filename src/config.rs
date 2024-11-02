use std::path::PathBuf;

pub struct DnConfig {
    pub filename_config: FileConfig,
}

pub struct FileConfig {
    pub directory: PathBuf,
    pub segment_order: [String; 5],
    pub default_extension: String,
}

impl Default for FileConfig {
    fn default() -> Self {
        Self {
            directory: Default::default(),
            segment_order: Default::default(),
            default_extension: Default::default(),
        }
    }
}

// parse toml config file
// pub fn parse_toml_config(path: PathBuf, )

// overlay config file onto default configuration
// pub fn override_defaults()
