use std::{env, path::PathBuf};

use anyhow::Error;

/// Get the default notes directory in a platform-agnostic way.
/// Tries to get `$HOME` (Unix-like systems) or `$USERPROFILE` (Windows).
/// If both cases fail, falls back to current directory.
pub fn default_notes_directory_from_environment() -> Result<PathBuf, Error> {
    let home_dir = env::var("HOME").or_else(|_| env::var("USERPROFILE"))?;

    let path = match PathBuf::from(home_dir).join("Documents") {
        path if path.exists() && path.is_dir() => path,
        path => path,
    };

    Ok(path.join("dn"))
}

/// Get the default config directory in a platform-agnostic way.
/// Tries to get `$XDG_CONFIG_HOME` (Unix-like systems) or `$USERPROFILE\AppData\Roaming` (Windows)
/// If both cases fail, falls back to current directory.
pub fn default_config_directory_from_environment() -> Result<PathBuf, Error> {
    let config_dir = env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|_| {
            env::var("USERPROFILE")
                .map(|profile| PathBuf::from(profile).join("AppData").join("Roaming"))
        })?;

    Ok(config_dir.join("dn"))
}

///////////
// Tests //
///////////

#[cfg(test)]
mod tests {}
