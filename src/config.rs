// SPDX-FileCopyrightText: 2024-2025 Matthew Mark Ibbetson
// SPDX-FileContributor: Matthew Mark Ibbetson
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! Data types and utility functions for dealing with dn's configuration file
//! and the configuration of individual modules.

use std::{
    collections::HashSet,
    env, fs,
    path::{Path, PathBuf},
};

use anyhow::Error;
use serde::{Deserialize, Serialize};

use crate::{
    directory::{environment_config_dir, environment_notes_dir},
    metadata::SEGMENT_SEPARATORS,
};

/// A `mut self` builder that allows progressively updating an input state for a new `Config`.
#[derive(Debug, Default)]
pub struct ConfigBuilder {
    base_config: Option<Config>,
    file_directory: Option<String>,
    file_default_extension: Option<String>,
    file_regenerate_identifier: bool,
    file_template_path: Option<PathBuf>,
}

/// The configuration values for the file name, directory, template, and general metadata.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
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
#[derive(PartialEq, Copy, Clone, Default, Debug, Serialize, Deserialize)]
pub enum FilenameSegment {
    #[serde(alias = "identifier")]
    Identifier,
    #[serde(alias = "signature")]
    Signature,
    #[default]
    #[serde(alias = "title")]
    Title,
    #[serde(alias = "keywords")]
    Keywords,
    #[serde(alias = "extension")]
    Extension,
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

    /// Builds the final `Config` state, falling back to the base configuration file
    /// values where no builder value has been specified.
    ///
    /// Prioritises as follows: `builder method > config file > type default`.
    ///
    /// # Errors
    ///
    /// Returns an `anyhow::Error` if unable to determine the front matter
    /// format.
    ///
    /// # Example
    ///
    /// ```
    /// let builder = Config::builder();
    /// builder = builder.with_default_extension(&Some("Example Title!"));
    /// metadata = builder.build(config)
    /// assert_eq!(metadata.title, Some("example-title".to_owned()))
    /// ```
    pub fn build(&self) -> Config {
        let base_config = self.base_config.clone().unwrap_or_default();

        let directory = self
            .file_directory
            .as_ref()
            .map(PathBuf::from)
            .unwrap_or(base_config.directory);

        let default_extension = self
            .file_default_extension
            .as_ref()
            .unwrap_or(&base_config.default_extension)
            .to_owned();

        let regenerate_identifier = if self.file_regenerate_identifier {
            true
        } else {
            base_config.regenerate_identifier
        };

        let template_path = self
            .file_template_path
            .as_ref()
            .or(base_config.template_path.as_ref())
            .cloned();

        // NOTE: It is essential that @=-_. are ALWAYS in the illegal characters,
        // even when overwritten by users.
        let illegal_characters = base_config
            .illegal_characters
            .into_iter()
            .chain(SEGMENT_SEPARATORS)
            .collect::<HashSet<_>>();

        Config {
            directory,
            default_extension,
            regenerate_identifier,
            template_path,
            illegal_characters,
            ..base_config
        }
    }
}

impl Default for Config {
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

/// Attempts to read the entire contents of a file at the given path and parse it into a `Config`.
///
/// # Errors
///
/// This function will return an error if there is a problem reading the file or parsing the contents
/// as a valid `Config`.
///
/// # Example
///
/// ```
/// let builder = FileMetadata::builder();
/// let config = read_config("path/to/config.toml")?;
///
/// builder.build(&config);
/// ```
pub fn read_config<P: AsRef<Path>>(path: P) -> Result<Config, Error> {
    let contents = fs::read_to_string(path)?;
    let config = toml::from_str(&contents)?;

    Ok(config)
}

/// Attempts to load configuration from either a provided path or the environment configuration directory.
///
/// If a path is provided, attempts to load and validate it, failing if invalid.
/// If no path is provided, checks the environment configuration directory for "dn.toml".
/// Returns None if no environment config exists or if the directory isn't accessible.
///
/// # Errors
///
/// This function will return an error in the following cases:
/// - The configuration file cannot be read or parsed
///
/// # Examples
///
/// ```rust
/// // Load from a specific path
/// let config = load_config(&Some("config.toml".to_string()))?;
/// assert!(config.is_some());
///
/// // Load from environment, returning None if not found
/// let config = load_config(&None)?;
///
/// // Handle the result
/// match config {
///     Some(cfg) => println!("Config loaded successfully"),
///     None => println!("No config found, using defaults"),
/// }
pub fn load_config(provided_path: Option<&str>) -> Result<Option<Config>, Error> {
    match provided_path {
        Some(path) => read_config(PathBuf::from(path)).map(Some),
        None => environment_config_dir().map_or(Ok(None), |p| {
            let config_path = p.join("dn.toml");

            if config_path.exists() && config_path.is_file() {
                read_config(&config_path).map(Some)
            } else {
                Ok(None)
            }
        }),
    }
}

/// Returns the default notes directory for dn. For use in serde macros.
///
/// # Value
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
/// # Value
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

/// Returns the default value for file extension in `Config`. For use in serde macros.
///
/// # Value
///
/// ```rust
/// "txt".to_owned()
/// ```
fn default_file_extension() -> String {
    "txt".to_owned()
}

/// Returns the default value for illegal characters in `Config`. For use in serde macros.
///
/// # Value
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

    use crate::config::Config;

    #[test]
    fn builder_builds_defaults_if_unconfigured() {
        // Arrange
        let input = Config::builder();
        let expected = Config::default();

        // Act
        let result = input.build();

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    fn builder_builds_base_config_defaults_if_provided() {
        // Arrange
        let base_config = Config {
            default_extension: "dj".to_owned(),
            regenerate_identifier: true,
            illegal_characters: HashSet::from(['a', '2', '@']),
            ..Default::default()
        };
        let input = Config::builder().with_base_config(base_config.clone());
        let expected_illegal_characters = HashSet::from(['a', '2', '@', '=', '-', '_', '.']);
        let expected = Config {
            illegal_characters: expected_illegal_characters,
            ..base_config
        };

        // Act
        let result = input.build();

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    fn builder_builds_with_supplied_values() {
        // Arrange
        let default_extension = "dj".to_owned();
        let directory = ".".to_owned();
        let regenerate_identifier = true;
        let template_path = "./template.txt".to_owned();

        let input = Config::builder()
            .with_file_default_extension(default_extension.clone())
            .with_file_directory(directory.clone())
            .with_file_regenerate_identifier(regenerate_identifier)
            .with_file_template_path(template_path.clone().into());

        let expected = Config {
            directory: directory.into(),
            default_extension,
            regenerate_identifier,
            template_path: Some(template_path.into()),
            ..Default::default()
        };

        // Act
        let result = input.build();

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    fn built_config_illegal_characters_always_contains_path_separators() {
        // Arrange
        let base_config = Config {
            illegal_characters: HashSet::from(['a', '2', '@']),
            ..Default::default()
        };
        let input = Config::builder().with_base_config(base_config);
        let expected = HashSet::from(['a', '2', '@', '=', '-', '_', '.']);

        // Act
        let result = input.build().illegal_characters;

        // Assert
        assert_eq!(expected, result);
    }
}
