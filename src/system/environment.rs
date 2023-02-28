//! ## Environment
//!
//! `environment` is the module which provides Path and values for the system environment

// Ext
use std::path::{Path, PathBuf};

/// ### get_config_dir
///
/// Get termscp configuration directory path.
/// Returns None, if it's not possible to get it
pub fn init_config_dir() -> Result<Option<PathBuf>, String> {
    // Get file
    #[cfg(not(test))]
    lazy_static! {
        static ref CONF_DIR: Option<PathBuf> = dirs::config_dir();
    }
    #[cfg(test)]
    lazy_static! {
        static ref CONF_DIR: Option<PathBuf> = Some(std::env::temp_dir());
    }
    if CONF_DIR.is_some() {
        // Get path of bookmarks
        let mut p: PathBuf = CONF_DIR.as_ref().unwrap().clone();
        // Append termscp dir
        p.push("termscp/");
        // If directory doesn't exist, create it
        if p.exists() {
            return Ok(Some(p));
        }
        // directory doesn't exist; create dir recursively
        match std::fs::create_dir_all(p.as_path()) {
            Ok(_) => Ok(Some(p)),
            Err(err) => Err(err.to_string()),
        }
    } else {
        Ok(None)
    }
}

/// ### get_bookmarks_paths
///
/// Get paths for bookmarks client
/// Returns: path of bookmarks.toml
pub fn get_bookmarks_paths(config_dir: &Path) -> PathBuf {
    // Prepare paths
    let mut bookmarks_file: PathBuf = PathBuf::from(config_dir);
    bookmarks_file.push("bookmarks.toml");
    bookmarks_file
}

/// ### get_config_paths
///
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

/// ### get_log_paths
///
/// Returns the path for the supposed log file
pub fn get_log_paths(config_dir: &Path) -> PathBuf {
    let mut log_file: PathBuf = PathBuf::from(config_dir);
    log_file.push("termscp.log");
    log_file
}

/// ### get_theme_path
///
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

    use super::*;

    use pretty_assertions::assert_eq;
    use serial_test::serial;
    use std::fs::{File, OpenOptions};
    use std::io::Write;

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
            get_log_paths(Path::new("/home/omar/.config/termscp/")),
            PathBuf::from("/home/omar/.config/termscp/termscp.log"),
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
