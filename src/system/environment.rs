//! ## Environment
//!
//! `environment` is the module which provides Path and values for the system environment

/**
 * MIT License
 *
 * termscp - Copyright (c) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
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
        match p.exists() {
            true => Ok(Some(p)),
            false => match std::fs::create_dir(p.as_path()) {
                Ok(_) => Ok(Some(p)),
                Err(err) => Err(err.to_string()),
            },
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
    use std::fs::{File, OpenOptions};
    use std::io::Write;

    #[test]
    fn test_system_environment_get_config_dir() {
        // Create and get conf_dir
        let conf_dir: PathBuf = init_config_dir().ok().unwrap().unwrap();
        // Remove dir
        assert!(std::fs::remove_dir_all(conf_dir.as_path()).is_ok());
    }

    #[test]
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
    fn test_system_environment_get_bookmarks_paths() {
        assert_eq!(
            get_bookmarks_paths(&Path::new("/home/omar/.config/termscp/")),
            PathBuf::from("/home/omar/.config/termscp/bookmarks.toml"),
        );
    }

    #[test]
    fn test_system_environment_get_config_paths() {
        assert_eq!(
            get_config_paths(&Path::new("/home/omar/.config/termscp/")),
            (
                PathBuf::from("/home/omar/.config/termscp/config.toml"),
                PathBuf::from("/home/omar/.config/termscp/.ssh/")
            )
        );
    }

    #[test]
    fn test_system_environment_get_log_paths() {
        assert_eq!(
            get_log_paths(&Path::new("/home/omar/.config/termscp/")),
            PathBuf::from("/home/omar/.config/termscp/termscp.log"),
        );
    }

    #[test]
    fn test_system_environment_get_theme_path() {
        assert_eq!(
            get_theme_path(&Path::new("/home/omar/.config/termscp/")),
            PathBuf::from("/home/omar/.config/termscp/theme.toml"),
        );
    }
}
