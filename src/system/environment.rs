//! ## Environment
//!
//! `environment` is the module which provides Path and values for the system environment

/*
*
*   Copyright (C) 2020-2021 Christian Visintin - christian.visintin1997@gmail.com
*
* 	This file is part of "TermSCP"
*
*   TermSCP is free software: you can redistribute it and/or modify
*   it under the terms of the GNU General Public License as published by
*   the Free Software Foundation, either version 3 of the License, or
*   (at your option) any later version.
*
*   TermSCP is distributed in the hope that it will be useful,
*   but WITHOUT ANY WARRANTY; without even the implied warranty of
*   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
*   GNU General Public License for more details.
*
*   You should have received a copy of the GNU General Public License
*   along with TermSCP.  If not, see <http://www.gnu.org/licenses/>.
*
*/

// Deps
extern crate dirs;

// Ext
use std::path::{Path, PathBuf};

/// ### get_config_dir
///
/// Get termscp configuration directory path.
/// Returns None, if it's not possible to get it
pub fn init_config_dir() -> Result<Option<PathBuf>, String> {
    // Get file
    lazy_static! {
        static ref CONF_DIR: Option<PathBuf> = dirs::config_dir();
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
/// Returns: path of bookmarks.toml and path of key
pub fn get_bookmarks_paths(config_dir: &Path) -> (PathBuf, PathBuf) {
    // Prepare paths
    let mut bookmarks_file: PathBuf = PathBuf::from(config_dir);
    bookmarks_file.push("bookmarks.toml");
    let mut key_file: PathBuf = PathBuf::from(config_dir);
    key_file.push(".bookmarks.key"); // key file is hidden
    (bookmarks_file, key_file)
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

#[cfg(test)]
mod tests {

    use super::*;

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
        let mut conf_dir: PathBuf = dirs::config_dir().unwrap();
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
            (
                PathBuf::from("/home/omar/.config/termscp/bookmarks.toml"),
                PathBuf::from("/home/omar/.config/termscp/.bookmarks.key")
            )
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
}
