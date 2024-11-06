use std::{
    env, fs,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Error};
use serde::{Deserialize, Serialize};

use crate::directory::environment_notes_dir;

/// Represents the configuration state for dn as a whole.
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub file: FileConfig,
    pub frontmatter: FrontmatterConfig,
}

/// TODO
#[derive(Debug, Default)]
struct ConfigBuilder {
    config_path: Option<PathBuf>,
    file_directory: Option<String>,
    file_default_extension: Option<String>,
    file_regenerate_identifier: bool,
    file_template_path: Option<PathBuf>,
    frontmatter_enabled: bool,
    frontmatter_format: Option<String>,
}

/// The configuration values for the file name, directory, template, and general metadata.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileConfig {
    /// The directory in which notes will be created.
    #[serde(default = "default_notes_directory")]
    pub directory: PathBuf,

    /// The order in which file name segments will appear.
    #[serde(default = "default_segment_order")]
    pub segment_order: [FilenameSegment; 5],

    /// The file extension to be used in file names when none is provided.
    #[serde(default = "default_file_extension")]
    pub default_extension: String,

    /// Whether to replace an existing identifier if present on a renamed note.
    #[serde(default = "r#false")]
    pub regenerate_identifier: bool,

    /// A file path to the template file, the contents of which will be inserted in the new note.
    #[serde(default = "none")]
    pub template_path: Option<PathBuf>,

    /// Characters to be sanitised out of the file metadata.
    #[serde(default = "default_illegal_characters")]
    pub illegal_characters: Vec<char>,
}

/// The segments which comprise a dn file name.
#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FilenameSegment {
    Identifier,
    Signature,
    Title,
    Keywords,
    Extension,
}

/// The configuration values for the front matter.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FrontmatterConfig {
    /// Whether or not to generate front matter on file creation.
    #[serde(default = "r#false")]
    pub enabled: bool,

    /// Which format to use for generated front matter.
    #[serde(default = "default_frontmatter_format")]
    pub format: FrontmatterFormat,

    /// Which time style to be used in the date segment of generated front matter.
    #[serde(default = "default_frontmatter_time_format")]
    pub time_style: FrontmatterTimeFormat,

    /// The order in which generated front matter segments appear.
    #[serde(default = "default_frontmatter_segment_order")]
    pub order: Vec<FrontmatterSegment>,
}

/// The possible valid formats for front matter.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FrontmatterFormat {
    Text,
    YAML,
    TOML,
    Org,
}

/// The valid front matter segments which dn concerns itself with.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FrontmatterSegment {
    Title,
    Date,
    Keywords,
    Identifier,
}

/// The valid time formats for front matter datetimes.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FrontmatterTimeFormat {
    Hour24,
    Hour12,
    None,
}

impl Config {
    /// TODO
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}

impl ConfigBuilder {
    /// TODO
    pub fn with_config_path(mut self, value: PathBuf) -> Self {
        self.config_path = Some(value);
        self
    }

    /// TODO
    pub fn with_file_directory(mut self, value: String) -> Self {
        self.file_directory = Some(value);
        self
    }

    /// TODO
    pub fn with_file_default_extension(mut self, value: String) -> Self {
        self.file_default_extension = Some(value);
        self
    }

    /// TODO
    pub fn with_file_regenerate_identifier(mut self, value: bool) -> ConfigBuilder {
        self.file_regenerate_identifier = value;
        self
    }

    /// TODO
    pub fn with_file_template_path(mut self, value: PathBuf) -> Self {
        self.file_template_path = Some(value);
        self
    }

    /// TODO
    pub fn with_frontmatter_enabled(mut self, value: bool) -> Self {
        self.frontmatter_enabled = value;
        self
    }

    /// TODO
    pub fn with_frontmatter_format(mut self, value: String) -> Self {
        self.frontmatter_format = Some(value);
        self
    }

    /// TODO
    pub fn build(&self) -> Config {
        let base_config = match &self.config_path {
            Some(path) => match read_config(path) {
                Ok(config) => config,
                Err(error) => {
                    eprintln!("Error reading configuration: {}", error);
                    std::process::exit(1);
                }
            },
            None => Config::default(),
        };

        let directory = self
            .file_directory
            .as_ref()
            .map(PathBuf::from)
            .unwrap_or(base_config.file.directory);

        let default_extension = self
            .file_default_extension
            .as_ref()
            .unwrap_or(&base_config.file.default_extension)
            .to_string();

        let regenerate_identifier = self
            .file_regenerate_identifier
            .then(|| true)
            .unwrap_or(base_config.file.regenerate_identifier);

        let template_path = self
            .file_template_path
            .as_ref()
            .or(base_config.file.template_path.as_ref())
            .cloned();

        let enabled = self
            .frontmatter_enabled
            .then(|| true)
            .unwrap_or(base_config.frontmatter.enabled);

        let format = self
            .frontmatter_format
            .clone()
            .map(|f| match determine_frontmatter_format(&f) {
                Ok(format) => format,
                Err(error) => {
                    eprintln!("Error determining frontmatter format: {}", error);
                    std::process::exit(1);
                }
            })
            .unwrap_or(base_config.frontmatter.format);

        Config {
            file: FileConfig {
                directory,
                default_extension,
                regenerate_identifier,
                template_path,
                ..base_config.file
            },
            frontmatter: FrontmatterConfig {
                enabled,
                format,
                ..base_config.frontmatter
            },
            ..base_config
        }
    }
}

impl Default for FileConfig {
    fn default() -> Self {
        Self {
            directory: default_notes_directory(),
            segment_order: default_segment_order(),
            default_extension: default_file_extension(),
            regenerate_identifier: r#false(),
            template_path: none::<PathBuf>(),
            illegal_characters: default_illegal_characters(),
        }
    }
}

impl Default for FilenameSegment {
    fn default() -> Self {
        FilenameSegment::Title
    }
}

impl Default for FrontmatterConfig {
    fn default() -> Self {
        Self {
            enabled: Default::default(),
            format: default_frontmatter_format(),
            time_style: default_frontmatter_time_format(),
            order: Default::default(),
        }
    }
}

/// Attempt to read the entire contents of a file and parse it into a `Config`.
fn read_config<P: AsRef<Path>>(path: P) -> Result<Config, Error> {
    let contents = fs::read_to_string(path)?;
    let config = toml::from_str(&contents)?;

    Ok(config)
}

/// Attempt to parse a string slice into a `FrontmatterFormat`.
fn determine_frontmatter_format(format_arg: &str) -> Result<FrontmatterFormat, Error> {
    match format_arg.to_lowercase().as_str() {
        "text" => Ok(FrontmatterFormat::Text),
        "yaml" => Ok(FrontmatterFormat::YAML),
        "toml" => Ok(FrontmatterFormat::TOML),
        "org" => Ok(FrontmatterFormat::Org),
        _ => Err(anyhow!(
            "Invalid frontmatter format provided, must be one of: text, yaml, toml, org"
        )),
    }
}

/// Returns the default notes directory for dn. For use in serde macros.
///
/// ## Value
///
/// It's value is a PathBuf from the first of these paths:
/// - $HOME/Documents/notes
/// - $USERPROFILE/Documents/notes
/// - .
fn default_notes_directory() -> PathBuf {
    environment_notes_dir().unwrap_or(env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
}

/// Returns the default value for file name segment order in FilenameConfig. For use in serde macros.
///
/// ## Value
///
/// ```rust
/// [
///     FilenameSegment::Identifier,
///     FilenameSegment::Signature,
///     FilenameSegment::Title,
///     FilenameSegment::Keywords,
///     FilenameSegment::Extension,
/// ]
/// ```
fn default_segment_order() -> [FilenameSegment; 5] {
    [
        FilenameSegment::Identifier,
        FilenameSegment::Signature,
        FilenameSegment::Title,
        FilenameSegment::Keywords,
        FilenameSegment::Extension,
    ]
}

/// Returns the default value for front matter segment order in FrontmatterConfig. For use in serde macros.
///
/// ## Value
///
/// ```rust
/// "txt".to_owned()
/// ```
fn default_file_extension() -> String {
    "txt".to_owned()
}

/// Returns the default value for front matter segment order in FrontmatterConfig. For use in serde macros.
///
/// ## Value
///
/// ```rust
/// FrontmatterFormat::Text
/// ```
fn default_frontmatter_format() -> FrontmatterFormat {
    FrontmatterFormat::Text
}

/// Returns the default value for front matter segment order in FrontmatterConfig. For use in serde macros.
///
/// ## Value
///
/// ```rust
/// FrontmatterTimeFormat::Hour24
/// ```
fn default_frontmatter_time_format() -> FrontmatterTimeFormat {
    FrontmatterTimeFormat::Hour24
}

/// Returns the default value for front matter segment order in FrontmatterConfig. For use in serde macros.
///
/// ## Value
///
/// ```rust
/// vec![
///     FrontmatterSegment::Title,
///     FrontmatterSegment::Date,
///     FrontmatterSegment::Keywords,
///     FrontmatterSegment::Identifier,
/// ]
/// ```
fn default_frontmatter_segment_order() -> Vec<FrontmatterSegment> {
    vec![
        FrontmatterSegment::Title,
        FrontmatterSegment::Date,
        FrontmatterSegment::Keywords,
        FrontmatterSegment::Identifier,
    ]
}
/// Returns the default value for illegal characters in FileConfig. For use in serde macros.
///
/// ## Value
///
/// ```rust
/// vec![
///     '[', ']', '{', '}', '(', ')', '!', '#', '$', '%', '^', '&', '*', '+', '\'', '\\', '"', '?',
///     ',', '|', ';', ':', '~', '`', '‘', '’', '“', '”', '/', '*', ' ', '@', '=', '-', '_', '.',
/// ]
/// ```
fn default_illegal_characters() -> Vec<char> {
    vec![
        '[', ']', '{', '}', '(', ')', '!', '#', '$', '%', '^', '&', '*', '+', '\'', '\\', '"', '?',
        ',', '|', ';', ':', '~', '`', '‘', '’', '“', '”', '/', '*', ' ', '@', '=', '-', '_', '.',
    ]
}

/// Returns `true`. For use in serde macros.
fn r#true() -> bool {
    true
}

/// Returns `false`. For use in serde macros.
fn r#false() -> bool {
    false
}

/// Returns `None`. For use in serde macros.
fn none<T>() -> Option<T> {
    None
}
