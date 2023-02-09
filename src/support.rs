//! ## Support
//!
//! this module exposes some extra run modes for termscp, meant to be used for "support", such as installing themes

// mod
use crate::system::{
    auto_update::{Update, UpdateStatus},
    config_client::ConfigClient,
    environment,
    notifications::Notification,
    theme_provider::ThemeProvider,
};
use std::fs;
use std::path::{Path, PathBuf};

/// ### import_theme
///
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

/// ### install_update
///
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

/// ### get_config_dir
///
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

/// ### get_config_client
///
/// Get configuration client
fn get_config_client() -> Option<ConfigClient> {
    match get_config_dir() {
        Err(_) => None,
        Ok(dir) => {
            let (cfg_path, ssh_key_dir) = environment::get_config_paths(dir.as_path());
            match ConfigClient::new(cfg_path.as_path(), ssh_key_dir.as_path()) {
                Err(_) => None,
                Ok(c) => Some(c),
            }
        }
    }
}
