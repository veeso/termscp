//! ## ThemeProvider
//!
//! `theme_provider` is the module which provides an API between the theme configuration and the system

// Locals
// Ext
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use std::string::ToString;

use crate::config::serialization::{deserialize, serialize, SerializerError, SerializerErrorKind};
use crate::config::themes::Theme;

/// ThemeProvider provides a high level API to communicate with the termscp theme
pub struct ThemeProvider {
    theme: Theme,        // Theme loaded
    theme_path: PathBuf, // Theme TOML Path
    degraded: bool,      // Fallback mode; won't work with file system
}

impl ThemeProvider {
    /// Instantiates a new `ThemeProvider`
    pub fn new(theme_path: &Path) -> Result<Self, SerializerError> {
        let default_theme: Theme = Theme::default();
        info!(
            "Setting up theme provider with thene path {} ",
            theme_path.display(),
        );
        // Create provider
        let mut provider: ThemeProvider = ThemeProvider {
            theme: default_theme,
            theme_path: theme_path.to_path_buf(),
            degraded: false,
        };
        // If Config file doesn't exist, create it
        if !theme_path.exists() {
            if let Err(err) = provider.save() {
                error!("Couldn't write theme file: {}", err);
                return Err(err);
            }
            debug!("Theme file didn't exist; created file");
        } else {
            // otherwise Load configuration from file
            if let Err(err) = provider.load() {
                error!("Couldn't read thene file: {}", err);
                return Err(err);
            }
            debug!("Read theme file");
        }
        Ok(provider)
    }

    /// Create a new theme provider which won't work with file system.
    /// This is done in order to prevent a lot of `unwrap_or` on Ui
    pub fn degraded() -> Self {
        Self {
            theme: Theme::default(),
            theme_path: PathBuf::default(),
            degraded: true,
        }
    }

    // -- getters

    /// Returns theme as reference
    pub fn theme(&self) -> &Theme {
        &self.theme
    }

    /// Returns a mutable reference to the theme
    pub fn theme_mut(&mut self) -> &mut Theme {
        &mut self.theme
    }

    // -- io

    /// Load theme from file
    pub fn load(&mut self) -> Result<(), SerializerError> {
        if self.degraded {
            warn!("Configuration won't be loaded, since degraded; reloading default...");
            self.theme = Theme::default();
            return Err(SerializerError::new_ex(
                SerializerErrorKind::Generic,
                String::from("Can't access theme file"),
            ));
        }
        // Open theme file for read
        debug!("Loading theme from file...");
        match OpenOptions::new()
            .read(true)
            .open(self.theme_path.as_path())
        {
            Ok(reader) => {
                // Deserialize
                match deserialize(Box::new(reader)) {
                    Ok(theme) => {
                        self.theme = theme;
                        Ok(())
                    }
                    Err(err) => Err(err),
                }
            }
            Err(err) => {
                error!("Failed to read theme: {}", err);
                Err(SerializerError::new_ex(
                    SerializerErrorKind::Io,
                    err.to_string(),
                ))
            }
        }
    }

    /// Save theme to file
    pub fn save(&self) -> Result<(), SerializerError> {
        if self.degraded {
            warn!("Configuration won't be saved, since in degraded mode");
            return Err(SerializerError::new_ex(
                SerializerErrorKind::Generic,
                String::from("Can't access theme file"),
            ));
        }
        // Open file
        debug!("Writing theme");
        match OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(self.theme_path.as_path())
        {
            Ok(writer) => serialize(self.theme(), Box::new(writer)),
            Err(err) => {
                error!("Failed to write theme: {}", err);
                Err(SerializerError::new_ex(
                    SerializerErrorKind::Io,
                    err.to_string(),
                ))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use tempfile::TempDir;
    use tuirealm::ratatui::style::Color;

    use super::*;

    #[test]
    fn test_system_theme_provider_new() {
        let tmp_dir: tempfile::TempDir = TempDir::new().ok().unwrap();
        let theme_path: PathBuf = get_theme_path(tmp_dir.path());
        // Initialize a new bookmarks client
        let mut provider: ThemeProvider = ThemeProvider::new(theme_path.as_path()).unwrap();
        // Verify client
        assert_eq!(provider.theme().auth_address, Color::Yellow);
        assert_eq!(provider.theme_path, theme_path);
        assert_eq!(provider.degraded, false);
        // Mutation
        provider.theme_mut().auth_address = Color::Green;
        assert_eq!(provider.theme().auth_address, Color::Green);
    }

    #[test]
    fn test_system_theme_provider_load_and_save() {
        let tmp_dir: tempfile::TempDir = TempDir::new().ok().unwrap();
        let theme_path: PathBuf = get_theme_path(tmp_dir.path());
        // Initialize a new bookmarks client
        let mut provider: ThemeProvider = ThemeProvider::new(theme_path.as_path()).unwrap();
        // Write
        provider.theme_mut().auth_address = Color::Green;
        assert!(provider.save().is_ok());
        provider.theme_mut().auth_address = Color::Blue;
        // Reload
        assert!(provider.load().is_ok());
        // Unchanged
        assert_eq!(provider.theme().auth_address, Color::Green);
        // Instantiate a new provider
        let provider: ThemeProvider = ThemeProvider::new(theme_path.as_path()).unwrap();
        assert_eq!(provider.theme().auth_address, Color::Green); // Unchanged
    }

    #[test]
    fn test_system_theme_provider_degraded() {
        let mut provider: ThemeProvider = ThemeProvider::degraded();
        assert_eq!(provider.theme().auth_address, Color::Yellow);
        assert_eq!(provider.degraded, true);
        provider.theme_mut().auth_address = Color::Green;
        assert!(provider.load().is_err());
        assert_eq!(provider.theme().auth_address, Color::Yellow);
        assert!(provider.save().is_err());
    }

    #[test]
    fn test_system_theme_provider_err() {
        assert!(ThemeProvider::new(Path::new("/tmp/oifoif/omar")).is_err());
    }

    /// Get paths for theme file
    fn get_theme_path(dir: &Path) -> PathBuf {
        let mut p: PathBuf = PathBuf::from(dir);
        p.push("theme.toml");
        p
    }
}
