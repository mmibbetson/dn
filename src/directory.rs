// SPDX-FileCopyrightText: 2024 Matthew Mark Ibbetson
// SPDX-FileContributor: Matthew Mark Ibbetson
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! Utilities for accessing and/or creating necessary directories.

use std::{env, fs, path::PathBuf};

use anyhow::Error;

/// Retrieves the path to the "notes" directory inside the user's "Documents" directory.
///
/// This function attempts to locate the user's "Documents/notes" directory, creating it if it does not exist.
///
/// # Errors
///
/// This function will return an error if there is a failure in retrieving environment variables
/// (like `HOME` on Unix systems or `USERPROFILE` on Windows), or if the directory cannot be created
/// due to file system errors.
///
/// # Example
///
/// ```
/// let notes_dir = environment_notes_dir();
/// assert!(notes_dir.is_ok());
/// ```
pub fn environment_notes_dir() -> Result<PathBuf, Error> {
    #[cfg(unix)]
    let home_dir = env::var("HOME")?;
    #[cfg(windows)]
    let home_dir = env::var("USERPROFILE")?;

    match PathBuf::from(home_dir).join("Documents").join("notes") {
        path if path.exists() && path.is_dir() => Ok(path),
        path => {
            fs::create_dir_all(&path)?;

            Ok(path)
        }
    }
}

/// Retrieves the path to the "dn" directory inside the user's configuration folder.
///
/// This function looks for the user's configuration directory and attempts to locate the "dn" subdirectory,
/// creating it if it does not exist. On Unix systems, it will first check `$XDG_CONFIG_HOME`, and if that
/// is unavailable, fall back to `$HOME/.config`. On Windows, it will check `USERPROFILE/AppData/Roaming`.
///
/// # Errors
///
/// This function may return an error if environment variables (`XDG_CONFIG_HOME`, `HOME`, `USERPROFILE`) cannot
/// be accessed, or if directory creation fails.
///
/// # Example
///
/// ```
/// let config_dir = environment_config_dir();
/// assert!(config_dir.is_ok());
/// ```
pub fn environment_config_dir() -> Result<PathBuf, Error> {
    #[cfg(unix)]
    let config_dir = {
        let config_home = env::var("XDG_CONFIG_HOME").map(PathBuf::from);
        let home_home = env::var("HOME").map(|h| PathBuf::from(h).join(".config"));

        config_home.or(home_home)?
    };
    #[cfg(windows)]
    let config_dir =
        env::var("USERPROFILE").map(|h| PathBuf::from(h).join("AppData").join("Roaming"))?;

    match config_dir.join("dn") {
        path if path.exists() && path.is_dir() => Ok(path),
        path => {
            fs::create_dir_all(&path)?;

            Ok(path)
        }
    }
}

///////////
// Tests //
///////////

#[cfg(test)]
mod tests {}
