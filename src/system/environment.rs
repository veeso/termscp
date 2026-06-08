//! ## Environment
//!
//! `environment` is the module which provides Path and values for the system environment

// Ext
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

/// termscp's configuration directory, including the `termscp` project subdirectory.
///
/// See [`config_dir`] for the per-platform locations.
#[cfg(not(test))]
static CONF_DIR: LazyLock<Option<PathBuf>> = LazyLock::new(config_dir);
#[cfg(test)]
static CONF_DIR: LazyLock<Option<PathBuf>> =
    LazyLock::new(|| Some(std::env::temp_dir().join("termscp")));

/// termscp's cache directory, including the `termscp` project subdirectory.
///
/// The cache directory stays at the platform-native location (`dirs::cache_dir`)
/// on every platform.
#[cfg(not(test))]
static CACHE_DIR: LazyLock<Option<PathBuf>> =
    LazyLock::new(|| dirs::cache_dir().map(|dir| dir.join("termscp")));
#[cfg(test)]
static CACHE_DIR: LazyLock<Option<PathBuf>> =
    LazyLock::new(|| Some(std::env::temp_dir().join("termscp")));

/// Resolves termscp's configuration directory for the current platform,
/// including the `termscp` project subdirectory.
///
/// The location is platform-specific:
///
/// - **Linux** (and other Unix): `$XDG_CONFIG_HOME/termscp` (usually `~/.config/termscp`)
/// - **macOS**: `~/.config/termscp` (instead of `~/Library/Application Support/termscp`)
/// - **Windows**: `%USERPROFILE%\.termscp` (instead of the roaming `%APPDATA%\termscp`)
#[cfg(not(test))]
fn config_dir() -> Option<PathBuf> {
    #[cfg(macos)]
    {
        // macOS: use ~/.config/termscp for consistency with Linux and easier
        // CLI access, instead of ~/Library/Application Support/termscp.
        dirs::home_dir().map(|home| home.join(".config").join("termscp"))
    }
    #[cfg(win)]
    {
        // Windows: use %USERPROFILE%\.termscp instead of the roaming %APPDATA%.
        dirs::home_dir().map(|home| home.join(".termscp"))
    }
    #[cfg(not(any(macos, win)))]
    {
        // Linux and other platforms: keep the XDG-compliant location.
        dirs::config_dir().map(|dir| dir.join("termscp"))
    }
}

/// Returns the legacy (pre-1.1.0) configuration directory, if the current
/// platform used a different location than the one returned by [`config_dir`].
///
/// Used to migrate an existing user configuration to the new location.
/// Returns `None` on platforms where the location did not change (e.g. Linux).
#[cfg(not(test))]
fn legacy_config_dir() -> Option<PathBuf> {
    #[cfg(any(macos, win))]
    {
        // Before 1.1.0 the config dir was always `dirs::config_dir()/termscp`.
        dirs::config_dir().map(|dir| dir.join("termscp"))
    }
    #[cfg(not(any(macos, win)))]
    {
        None
    }
}

/// In tests we never want to touch the developer's real configuration, so there
/// is no legacy directory to migrate from.
#[cfg(test)]
fn legacy_config_dir() -> Option<PathBuf> {
    None
}

/// Get termscp config directory path and initialize it.
/// Returns None if it's not possible to initialize it
pub fn init_config_dir() -> Result<Option<PathBuf>, String> {
    let Some(dir) = CONF_DIR.as_deref() else {
        return Ok(None);
    };
    // Migrate an existing configuration from the legacy location, if necessary,
    // before creating (and thus claiming) the new directory.
    migrate_config_dir(legacy_config_dir().as_deref(), dir)?;
    init_dir(dir).map(Option::Some)
}

/// Get termscp cache directory path and initialize it.
/// Returns None if it's not possible to initialize it
pub fn init_cache_dir() -> Result<Option<PathBuf>, String> {
    if let Some(dir) = CACHE_DIR.as_deref() {
        init_dir(dir).map(Option::Some)
    } else {
        Ok(None)
    }
}

/// Moves the legacy configuration directory to the new location when a migration
/// is needed.
///
/// A migration happens only when `new_dir` does not exist yet and `legacy` is
/// `Some` and points to an existing directory; otherwise this is a no-op.
fn migrate_config_dir(legacy: Option<&Path>, new_dir: &Path) -> Result<(), String> {
    let Some(legacy) = legacy else {
        return Ok(());
    };
    // Nothing to migrate if the new dir is already there or the legacy one is gone.
    if new_dir.exists() || !legacy.exists() {
        return Ok(());
    }
    // Ensure the parent of the new dir exists before moving into it.
    if let Some(parent) = new_dir.parent() {
        std::fs::create_dir_all(parent).map_err(|err| err.to_string())?;
    }
    std::fs::rename(legacy, new_dir).map_err(|err| err.to_string())
}

/// Init a termscp env dir, creating it if it doesn't already exist.
fn init_dir(p: &Path) -> Result<PathBuf, String> {
    // If the directory already exists, there's nothing to do.
    if p.is_dir() {
        return Ok(p.to_path_buf());
    }
    // Directory doesn't exist; create it recursively. This fails if the path is
    // already occupied by a non-directory file.
    std::fs::create_dir_all(p).map_err(|err| err.to_string())?;
    Ok(p.to_path_buf())
}

/// Get paths for bookmarks client
/// Returns: path of bookmarks.toml
pub fn get_bookmarks_paths(config_dir: &Path) -> PathBuf {
    // Prepare paths
    let mut bookmarks_file: PathBuf = PathBuf::from(config_dir);
    bookmarks_file.push("bookmarks.toml");
    bookmarks_file
}

/// Returns paths for config client
/// Returns: path of config.toml and path for ssh keys
pub fn get_config_paths(config_dir: &Path) -> (PathBuf, PathBuf) {
    // Prepare paths
    let mut bookmarks_file: PathBuf = PathBuf::from(config_dir);
    bookmarks_file.push("config.toml");
    let mut keys_dir: PathBuf = PathBuf::from(config_dir);
    keys_dir.push(".ssh/"); // Path where keys are stored
    (bookmarks_file, keys_dir)
}

/// Returns the path for the supposed log file
pub fn get_log_paths(cache_dir: &Path) -> PathBuf {
    let mut log_file: PathBuf = PathBuf::from(cache_dir);
    log_file.push("termscp.log");
    log_file
}

/// Get paths for theme provider
/// Returns: path of theme.toml
pub fn get_theme_path(config_dir: &Path) -> PathBuf {
    // Prepare paths
    let mut theme_file: PathBuf = PathBuf::from(config_dir);
    theme_file.push("theme.toml");
    theme_file
}

#[cfg(test)]
mod tests {

    use std::fs::{File, OpenOptions};
    use std::io::Write;

    use pretty_assertions::assert_eq;
    use serial_test::serial;

    use super::*;

    #[test]
    #[serial]
    fn test_system_environment_get_config_dir() {
        // Create and get conf_dir
        let conf_dir: PathBuf = init_config_dir().ok().unwrap().unwrap();
        // Remove dir
        assert!(std::fs::remove_dir_all(conf_dir.as_path()).is_ok());
    }

    #[test]
    #[serial]
    fn should_get_cache_dir() {
        // Create and get cache_dir
        let cache_dir: PathBuf = init_cache_dir().ok().unwrap().unwrap();
        // Remove dir
        assert!(std::fs::remove_dir_all(cache_dir.as_path()).is_ok());
    }

    #[test]
    #[serial]
    fn test_system_environment_get_config_dir_err() {
        let mut conf_dir: PathBuf = std::env::temp_dir();
        conf_dir.push("termscp");
        // Create file
        let mut f: File = OpenOptions::new()
            .create(true)
            .write(true)
            .open(conf_dir.as_path())
            .ok()
            .unwrap();
        // Write
        assert!(writeln!(f, "Hello world!").is_ok());
        // Drop file
        drop(f);
        // Get config dir (will fail)
        assert!(init_config_dir().is_err());
        // Remove file
        assert!(std::fs::remove_file(conf_dir.as_path()).is_ok());
    }

    #[test]
    #[serial]
    fn should_migrate_legacy_config_dir() {
        let base = std::env::temp_dir().join("termscp-migrate-test-move");
        let legacy = base.join("legacy");
        let new_dir = base.join("new");
        // Clean up any leftovers from a previous run
        let _ = std::fs::remove_dir_all(&base);
        // Set up a legacy dir holding a config file
        std::fs::create_dir_all(&legacy).unwrap();
        let legacy_file = legacy.join("config.toml");
        std::fs::write(&legacy_file, b"hello").unwrap();
        // Migrate
        assert!(migrate_config_dir(Some(legacy.as_path()), new_dir.as_path()).is_ok());
        // Legacy dir is gone, new dir holds the file
        assert!(!legacy.exists());
        assert!(new_dir.join("config.toml").exists());
        // Cleanup
        let _ = std::fs::remove_dir_all(&base);
    }

    #[test]
    #[serial]
    fn should_not_migrate_when_new_dir_exists() {
        let base = std::env::temp_dir().join("termscp-migrate-test-keep");
        let legacy = base.join("legacy");
        let new_dir = base.join("new");
        let _ = std::fs::remove_dir_all(&base);
        std::fs::create_dir_all(&legacy).unwrap();
        std::fs::create_dir_all(&new_dir).unwrap();
        // Both exist: must be a no-op, legacy left untouched
        assert!(migrate_config_dir(Some(legacy.as_path()), new_dir.as_path()).is_ok());
        assert!(legacy.exists());
        let _ = std::fs::remove_dir_all(&base);
    }

    #[test]
    #[serial]
    fn should_not_migrate_without_legacy_dir() {
        let base = std::env::temp_dir().join("termscp-migrate-test-none");
        let new_dir = base.join("new");
        let _ = std::fs::remove_dir_all(&base);
        // No legacy dir at all
        assert!(migrate_config_dir(None, new_dir.as_path()).is_ok());
        assert!(!new_dir.exists());
        // Legacy provided but missing
        let missing = base.join("missing");
        assert!(migrate_config_dir(Some(missing.as_path()), new_dir.as_path()).is_ok());
        assert!(!new_dir.exists());
        let _ = std::fs::remove_dir_all(&base);
    }

    #[test]
    #[serial]
    fn test_system_environment_get_bookmarks_paths() {
        assert_eq!(
            get_bookmarks_paths(Path::new("/home/omar/.config/termscp/")),
            PathBuf::from("/home/omar/.config/termscp/bookmarks.toml"),
        );
    }

    #[test]
    #[serial]
    fn test_system_environment_get_config_paths() {
        assert_eq!(
            get_config_paths(Path::new("/home/omar/.config/termscp/")),
            (
                PathBuf::from("/home/omar/.config/termscp/config.toml"),
                PathBuf::from("/home/omar/.config/termscp/.ssh/")
            )
        );
    }

    #[test]
    #[serial]
    fn test_system_environment_get_log_paths() {
        assert_eq!(
            get_log_paths(Path::new("/home/omar/.cache/termscp/")),
            PathBuf::from("/home/omar/.cache/termscp/termscp.log"),
        );
    }

    #[test]
    #[serial]
    fn test_system_environment_get_theme_path() {
        assert_eq!(
            get_theme_path(Path::new("/home/omar/.config/termscp/")),
            PathBuf::from("/home/omar/.config/termscp/theme.toml"),
        );
    }
}
