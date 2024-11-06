use std::{env, fs, path::PathBuf};

use anyhow::Error;

/// Get the default notes directory `Documents/notes/` in a platform-agnostic way.
///
/// Tries to get relative to `$HOME` (Unix-like systems) or `$USERPROFILE` (Windows).
/// If one case succeeds but the `Documents/notes/` directory is not present, it will be created.
/// If both cases fail, falls back to current directory.
pub fn environment_notes_dir() -> Result<PathBuf, Error> {
    let home_dir = env::var("HOME").or_else(|_| env::var("USERPROFILE"))?;

    let path = match PathBuf::from(home_dir).join("Documents") {
        path if path.exists() && path.is_dir() => path,
        path => path,
    };

    let path = path.join("notes");

    path.try_exists()?.then(|| fs::create_dir_all(&path));

    Ok(path)
}

/// Get the default config directory `dn/` in a platform-agnostic way.
///
/// Tries to get relative to `$XDG_CONFIG_HOME` (Unix-like systems) or `$USERPROFILE\AppData\Roaming` (Windows)
/// If one case succeeds but the `dn/` directory is not present, it will be created.
/// If both cases fail, falls back to current directory.
pub fn environment_config_dir() -> Result<PathBuf, Error> {
    let config_dir = env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|_| {
            env::var("USERPROFILE")
                .map(|profile| PathBuf::from(profile).join("AppData").join("Roaming"))
        })?;

    let path = config_dir.join("dn");

    path.try_exists()?.then(|| fs::create_dir_all(&path));

    Ok(path)
}
