//! ## Support
//!
//! this module exposes some extra run modes for termscp, meant to be used for "support", such as installing themes

mod import_ssh_hosts;

use std::fs;
use std::path::{Path, PathBuf};

pub use self::import_ssh_hosts::import_ssh_hosts;
use crate::system::auto_update::{Update, UpdateStatus};
use crate::system::bookmarks_client::BookmarksClient;
use crate::system::config_client::ConfigClient;
use crate::system::environment;
use crate::system::notifications::Notification;
use crate::system::theme_provider::ThemeProvider;

/// Import theme at provided path into termscp
pub fn import_theme(p: &Path) -> Result<(), String> {
    if !p.exists() {
        return Err(String::from(
            "Could not import theme: No such file or directory",
        ));
    }
    // Validate theme file
    ThemeProvider::new(p).map_err(|e| format!("Invalid theme error: {e}"))?;
    // get config dir
    let cfg_dir: PathBuf = get_config_dir()?;
    // Get theme directory
    let theme_file: PathBuf = environment::get_theme_path(cfg_dir.as_path());
    // Copy theme to theme_dir
    fs::copy(p, theme_file.as_path())
        .map(|_| ())
        .map_err(|e| format!("Could not import theme: {e}"))
}

/// Install latest version of termscp if an update is available
pub fn install_update() -> Result<String, String> {
    match Update::default()
        .show_progress(true)
        .ask_confirm(true)
        .upgrade()
    {
        Ok(UpdateStatus::AlreadyUptodate) => Ok("termscp is already up to date".to_string()),
        Ok(UpdateStatus::UpdateInstalled(v)) => {
            if get_config_client()
                .map(|x| x.get_notifications())
                .unwrap_or(true)
            {
                Notification::update_installed(v.as_str());
            }
            Ok(format!("termscp has been updated to version {v}"))
        }
        Err(err) => {
            if get_config_client()
                .map(|x| x.get_notifications())
                .unwrap_or(true)
            {
                Notification::update_failed(err.to_string());
            }
            Err(err.to_string())
        }
    }
}

/// Get configuration directory
fn get_config_dir() -> Result<PathBuf, String> {
    match environment::init_config_dir() {
        Ok(Some(config_dir)) => Ok(config_dir),
        Ok(None) => Err(String::from(
            "Your system doesn't provide a configuration directory",
        )),
        Err(err) => Err(format!(
            "Could not initialize configuration directory: {err}"
        )),
    }
}

/// Get configuration client
fn get_config_client() -> Option<ConfigClient> {
    match get_config_dir() {
        Err(_) => None,
        Ok(dir) => {
            let (cfg_path, ssh_key_dir) = environment::get_config_paths(dir.as_path());
            ConfigClient::new(cfg_path.as_path(), ssh_key_dir.as_path()).ok()
        }
    }
}

/// Init [`BookmarksClient`].
pub fn bookmarks_client(keyring: bool) -> Result<Option<BookmarksClient>, String> {
    // Get config dir
    match environment::init_config_dir() {
        Ok(path) => {
            // If some configure client, otherwise do nothing; don't bother users telling them that bookmarks are not supported on their system.
            if let Some(config_dir_path) = path {
                let bookmarks_file: PathBuf =
                    environment::get_bookmarks_paths(config_dir_path.as_path());
                // Initialize client
                BookmarksClient::new(
                    bookmarks_file.as_path(),
                    config_dir_path.as_path(),
                    16,
                    keyring,
                )
                .map(Option::Some)
                .map_err(|e| {
                    format!(
                        "Could not initialize bookmarks (at \"{}\", \"{}\"): {}",
                        bookmarks_file.display(),
                        config_dir_path.display(),
                        e
                    )
                })
            } else {
                Ok(None)
            }
        }
        Err(err) => Err(err),
    }
}
