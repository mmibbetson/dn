//! TODO

use std::{
    collections::HashSet,
    env, fs,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Error};
use serde::{Deserialize, Serialize};

use crate::{directory::environment_notes_dir, metadata::SEGMENT_SEPARATORS};

/// Represents the configuration state for dn as a whole.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub file: FileConfig,
    pub frontmatter: FrontmatterConfig,
}

/// A `mut self` builder that allows progressively updating an input state for a new `Config`.
#[derive(Debug, Default)]
struct ConfigBuilder {
    base_config: Option<Config>,
    file_directory: Option<String>,
    file_default_extension: Option<String>,
    file_regenerate_identifier: bool,
    file_template_path: Option<PathBuf>,
    frontmatter_enabled: bool,
    frontmatter_format: Option<String>,
}

/// The configuration values for the file name, directory, template, and general metadata.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

    /// Whether or not to replace an existing identifier if present on a renamed note.
    #[serde(default = "r#false")]
    pub regenerate_identifier: bool,

    /// A file path to the template file, the contents of which will be inserted in the new note.
    #[serde(default = "none")]
    pub template_path: Option<PathBuf>,

    /// Characters to be sanitised out of the file metadata.
    #[serde(default = "default_illegal_characters")]
    pub illegal_characters: HashSet<char>,
}

/// The segments which comprise a dn file name.
#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum FilenameSegment {
    Identifier,
    Signature,
    #[default]
    Title,
    Keywords,
    Extension,
}

/// The configuration values for the front matter.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum FrontmatterFormat {
    Text,
    Yaml,
    Toml,
    Org,
}

/// The valid front matter segments which dn concerns itself with.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum FrontmatterSegment {
    Title,
    Date,
    Keywords,
    Identifier,
}

/// The valid time formats for front matter datetimes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum FrontmatterTimeFormat {
    Hour24,
    Hour12,
    None,
}

impl Config {
    /// Creates a new builder initialised with default values.
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}

impl ConfigBuilder {
    /// Adds a path to a configuration file to the builder.
    pub fn with_base_config(mut self, value: Config) -> Self {
        self.base_config = Some(value);
        self
    }

    /// Adds the output directory for the file to the builder.
    pub fn with_file_directory(mut self, value: String) -> Self {
        self.file_directory = Some(value);
        self
    }

    /// Adds the default file extension for the file to the builder.
    pub fn with_file_default_extension(mut self, value: String) -> Self {
        self.file_default_extension = Some(value);
        self
    }

    /// Sets whether or not to regenerate an identifier when renaming a file on the builder.
    pub fn with_file_regenerate_identifier(mut self, value: bool) -> ConfigBuilder {
        self.file_regenerate_identifier = value;
        self
    }

    /// Adds the input path for a template file to the builder.
    pub fn with_file_template_path(mut self, value: PathBuf) -> Self {
        self.file_template_path = Some(value);
        self
    }

    /// Sets whether or not to generate and/or replace frontmatter for a file on the builder.
    pub fn with_frontmatter_enabled(mut self, value: bool) -> Self {
        self.frontmatter_enabled = value;
        self
    }

    /// Sets which format to use for the frontmatter to the builder.
    pub fn with_frontmatter_format(mut self, value: String) -> Self {
        self.frontmatter_format = Some(value);
        self
    }

    /// Builds the final `Config` state, falling back to the base configuration file
    /// values where no builder value has been specified.
    ///
    /// Prioritises as follows: `builder method > config file > type default`.
    ///
    /// ## Errors
    ///
    /// Returns an `anyhow::Error` if unable to determine the front matter
    /// format.
    pub fn build(&self) -> Result<Config, Error> {
        let base_config = self.base_config.clone().unwrap_or_default();

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

        let regenerate_identifier = if self.file_regenerate_identifier {
            true
        } else {
            base_config.file.regenerate_identifier
        };

        let template_path = self
            .file_template_path
            .as_ref()
            .or(base_config.file.template_path.as_ref())
            .cloned();

        // NOTE: It is essential that @=-_. are ALWAYS in the illegal characters,
        // even when overwritten by users.
        let illegal_characters = base_config
            .file
            .illegal_characters
            .into_iter()
            .chain(SEGMENT_SEPARATORS)
            .collect::<HashSet<_>>();

        let enabled = if self.frontmatter_enabled {
            true
        } else {
            base_config.frontmatter.enabled
        };

        let format = {
            let format = self
                .frontmatter_format
                .clone()
                .map(|f| determine_frontmatter_format(&f));

            match format {
                Some(result) => result?,
                None => base_config.frontmatter.format,
            }
        };

        Ok(Config {
            file: FileConfig {
                directory,
                default_extension,
                regenerate_identifier,
                template_path,
                illegal_characters,
                ..base_config.file
            },
            frontmatter: FrontmatterConfig {
                enabled,
                format,
                ..base_config.frontmatter
            },
        })
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

impl Default for FrontmatterConfig {
    fn default() -> Self {
        Self {
            enabled: Default::default(),
            format: default_frontmatter_format(),
            time_style: default_frontmatter_time_format(),
            order: Vec::default(),
        }
    }
}

/// Attempt to read the entire contents of a file and parse it into a `Config`.
pub fn read_config<P: AsRef<Path>>(path: P) -> Result<Config, Error> {
    let contents = fs::read_to_string(path)?;
    let config = toml::from_str(&contents)?;

    Ok(config)
}

/// Attempt to parse a string slice into a `FrontmatterFormat`.
fn determine_frontmatter_format(format_arg: &str) -> Result<FrontmatterFormat, Error> {
    match format_arg.to_lowercase().as_str() {
        "text" => Ok(FrontmatterFormat::Text),
        "yaml" => Ok(FrontmatterFormat::Yaml),
        "toml" => Ok(FrontmatterFormat::Toml),
        "org" => Ok(FrontmatterFormat::Org),
        _ => Err(anyhow!(
            "Invalid frontmatter format provided, must be one of: text, yaml, toml, org.\nGot: {:#?}", format_arg
        )),
    }
}

/// Returns the default notes directory for dn. For use in serde macros.
///
/// ## Value
///
/// It's value is a `PathBuf` from the first of these paths:
/// - `$HOME/Documents/notes`
/// - `$USERPROFILE/Documents/notes`
/// - `.`
fn default_notes_directory() -> PathBuf {
    environment_notes_dir()
        .or_else(|_| env::current_dir())
        .unwrap_or_else(|_| PathBuf::from("."))
}

/// Returns the default value for file name segment order in `FilenameConfig`. For use in serde macros.
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

/// Returns the default value for front matter segment order in `FrontmatterConfig`. For use in serde macros.
///
/// ## Value
///
/// ```rust
/// "txt".to_owned()
/// ```
fn default_file_extension() -> String {
    "txt".to_owned()
}

/// Returns the default value for front matter segment order in `FrontmatterConfig`. For use in serde macros.
///
/// ## Value
///
/// ```rust
/// FrontmatterFormat::Text
/// ```
fn default_frontmatter_format() -> FrontmatterFormat {
    FrontmatterFormat::Text
}

/// Returns the default value for front matter segment order in `FrontmatterConfig`. For use in serde macros.
///
/// ## Value
///
/// ```rust
/// FrontmatterTimeFormat::Hour24
/// ```
fn default_frontmatter_time_format() -> FrontmatterTimeFormat {
    FrontmatterTimeFormat::Hour24
}

/// Returns the default value for front matter segment order in `FrontmatterConfig`. For use in serde macros.
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
/// Returns the default value for illegal characters in `FileConfig`. For use in serde macros.
///
/// ## Value
///
/// ```rust
/// HashSet::from([
///     '[', ']', '{', '}', '(', ')', '!', '#', '$', '%', '^', '&', '*', '+', '\'', '\\', '"', '?',
///     ',', '|', ';', ':', '~', '`', '‘', '’', '“', '”', '/', '*', ' ', '@', '=', '-', '_', '.',
/// ])
/// ```
fn default_illegal_characters() -> HashSet<char> {
    HashSet::from([
        '[', ']', '{', '}', '(', ')', '!', '#', '$', '%', '^', '&', '*', '+', '\'', '\\', '"', '?',
        ',', '|', ';', ':', '~', '`', '‘', '’', '“', '”', '/', '*', ' ', '@', '=', '-', '_', '.',
    ])
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

///////////
// Tests //
///////////

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::config::{Config, FileConfig, FrontmatterConfig, FrontmatterFormat};

    #[test]
    fn builder_builds_defaults_if_unconfigured() {
        // Arrange
        let input = Config::builder();
        let expected = Config::default();

        // Act
        let result = input.build().unwrap();

        // Assert
        assert_eq!(
            expected, result,
            "Input: {input:#?}\nExpected: {expected:#?}\nReceived: {result:#?}"
        );
    }

    #[test]
    fn builder_builds_base_config_defaults_if_provided() {
        // Arrange
        let base_config = Config {
            file: FileConfig {
                default_extension: "dj".to_owned(),
                regenerate_identifier: true,
                illegal_characters: HashSet::from(['a', '2', '@']),
                ..FileConfig::default()
            },
            frontmatter: FrontmatterConfig {
                enabled: true,
                format: FrontmatterFormat::Toml,
                ..FrontmatterConfig::default()
            },
        };
        let input = Config::builder().with_base_config(base_config.clone());
        let expected_illegal_characters = HashSet::from(['a', '2', '@', '=', '-', '_', '.']);
        let expected = Config {
            file: FileConfig {
                illegal_characters: expected_illegal_characters,
                ..base_config.file
            },
            ..base_config
        };

        // Act
        let result = input.build().unwrap();

        // Assert
        assert_eq!(
            expected, result,
            "Input: {input:#?}\nExpected: {expected:#?}\nReceived: {result:#?}"
        );
    }

    #[test]
    fn builder_builds_with_supplied_values() {
        // Arrange
        let default_extension = "dj".to_string();
        let directory = ".".to_string();
        let regenerate_identifier = true;
        let template_path = "./template.txt".to_string();
        let enabled = true;
        let format = "org".into();

        let input = Config::builder()
            .with_file_default_extension(default_extension.clone())
            .with_file_directory(directory.clone())
            .with_file_regenerate_identifier(regenerate_identifier)
            .with_file_template_path(template_path.clone().into())
            .with_frontmatter_enabled(enabled)
            .with_frontmatter_format(format);

        let expected = Config {
            file: FileConfig {
                directory: directory.into(),
                default_extension,
                regenerate_identifier,
                template_path: Some(template_path.into()),
                ..Default::default()
            },
            frontmatter: FrontmatterConfig {
                enabled,
                format: FrontmatterFormat::Org,
                ..Default::default()
            },
        };

        // Act
        let result = input.build().unwrap();

        // Assert
        assert_eq!(
            expected, result,
            "Input: {input:#?}\nExpected: {expected:#?}\nReceived: {result:#?}",
        );
    }

    #[test]
    fn built_config_illegal_characters_always_contains_path_separators() {
        // Arrange
        let base_config = Config {
            file: FileConfig {
                illegal_characters: HashSet::from(['a', '2', '@']),
                ..FileConfig::default()
            },
            ..Config::default()
        };
        let input = Config::builder().with_base_config(base_config.clone());
        let expected = HashSet::from(['a', '2', '@', '=', '-', '_', '.']);

        // Act
        let result = input.build().unwrap().file.illegal_characters;

        // Assert
        assert_eq!(
            expected, result,
            "Input: {input:#?}\nExpected: {expected:#?}\nReceived: {result:#?}"
        );
    }

    #[test]
    fn build_config_fails_with_invalid_frontmatter_format() {
        // Arrange
        let format = "scaml";
        let input = Config::builder().with_frontmatter_format(format.to_owned());

        // Act
        let result = input.build();

        // Assert
        assert!(
            result
                .as_ref()
                .is_err_and(|e| e.to_string().contains("Invalid frontmatter format")),
            "Input: {input:#?}\nExpected an error.\nReceived: {result:#?}",
        );
    }
}
