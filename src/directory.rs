use std::{env, path::PathBuf};

// TODO: What happens if someone supplies an invalid config directory or note directory?

/// Get the default notes directory in a platform-agnostic way.
/// Tries to get the HOME (Unix-like systems) or USERPROFILE (Windows).
/// If both cases fail, falls back to current directory.
pub fn get_default_notes_dir() -> PathBuf {
    let home_dir = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .map_or(
            env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            PathBuf::from,
        );

    // TODO: If home_dir.join("Documents") exists, use that.join("dn")
    // Otherwise use just home_dir.join("dn")
    let notes_dir = home_dir.join("Documents").join("dn");

    notes_dir
}

/// Get the default config directory in a platform-agnostic way.
/// Tries to get the XDG_CONFIG_HOME (Unix-like systems) or APPDATA (Windows)
/// If both cases fail, falls back to current directory.
pub fn get_default_config_dir() -> PathBuf {
    let config_dir = env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|_| {
            env::var("USERPROFILE")
                .map(|profile| PathBuf::from(profile).join("AppData").join("Roaming"))
        })
        .unwrap_or_else(|_| PathBuf::from("."));

    config_dir.join("dn")
}
