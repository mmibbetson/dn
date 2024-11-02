use std::{env, path::PathBuf};

// TODO: What happens if someone supplies an invalid config directory or note directory?

/// Get the default notes directory in a platform-agnostic way.
pub fn get_default_notes_dir() -> PathBuf {
    // Try to get the HOME (Unix-like systems) or USERPROFILE (Windows)
    let home_dir = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .map_or(
            env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            PathBuf::from,
        );

    #[cfg(target_os = "windows")]
    let notes_dir = home_dir.join("Documents").join("dnotes");

    #[cfg(not(target_os = "windows"))]
    let notes_dir = home_dir.join("dnotes");

    notes_dir
}

/// Get the default config directory in a platform-agnostic way.
pub fn get_default_config_dir() -> PathBuf {
    // Try to get the XDG_CONFIG_HOME (Unix-like systems) or APPDATA (Windows)
    let config_dir = env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|_| {
            env::var("USERPROFILE")
                .map(|profile| PathBuf::from(profile).join("AppData").join("Roaming"))
        })
        .unwrap_or_else(|_| PathBuf::from(".")); // fallback to current directory if all else fails

    config_dir.join("dn")
}
