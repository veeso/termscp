//! ## SetupActivity
//!
//! `setup_activity` is the module which implements the Setup activity, which is the activity to
//! work on termscp configuration

// Locals
use super::SetupActivity;
// Ext
use std::env;

impl SetupActivity {
    /// Save configuration
    pub(super) fn save_config(&mut self) -> Result<(), String> {
        match self.config().write_config() {
            Ok(_) => Ok(()),
            Err(err) => {
                error!("Could not save configuration: {}", err);
                Err(format!("Could not save configuration: {}", err))
            }
        }
    }

    /// Reset configuration changes; pratically read config from file, overwriting any change made
    /// since last write action
    pub(super) fn reset_config_changes(&mut self) -> Result<(), String> {
        self.config_mut()
            .read_config()
            .map_err(|e| format!("Could not reload configuration: {}", e))
    }

    /// Save theme to file
    pub(super) fn save_theme(&mut self) -> Result<(), String> {
        self.theme_provider()
            .save()
            .map_err(|e| format!("Could not save theme: {}", e))
    }

    /// Reset changes committed to theme
    pub(super) fn reset_theme_changes(&mut self) -> Result<(), String> {
        self.theme_provider()
            .load()
            .map_err(|e| format!("Could not restore theme: {}", e))
    }

    /// Delete ssh key from config cli
    pub(super) fn delete_ssh_key(&mut self, host: &str, username: &str) -> Result<(), String> {
        match self.config_mut().del_ssh_key(host, username) {
            Ok(_) => Ok(()),
            Err(err) => Err(format!(
                "Could not delete ssh key \"{}@{}\": {}",
                host, username, err
            )),
        }
    }

    /// Edit selected ssh key
    pub(super) fn edit_ssh_key(&mut self, idx: usize) -> Result<(), String> {
        match self.context.as_mut() {
            None => Ok(()),
            Some(ctx) => {
                // Set editor if config client exists
                env::set_var("EDITOR", ctx.config().get_text_editor());
                // Prepare terminal
                if let Err(err) = ctx.terminal().disable_raw_mode() {
                    error!("Failed to disable raw mode: {}", err);
                }
                // Leave alternate mode
                if let Err(err) = ctx.terminal().leave_alternate_screen() {
                    error!("Could not leave alternate screen: {}", err);
                }
                // Lock ports
                assert!(self.app.lock_ports().is_ok());
                // Get result
                let result: Result<(), String> = match ctx.config().iter_ssh_keys().nth(idx) {
                    Some(key) => {
                        // Get key path
                        match ctx.config().get_ssh_key(key) {
                            Ok(ssh_key) => match ssh_key {
                                None => Ok(()),
                                Some((_, _, key_path)) => {
                                    match edit::edit_file(key_path.as_path()) {
                                        Ok(_) => Ok(()),
                                        Err(err) => Err(format!("Could not edit ssh key: {}", err)),
                                    }
                                }
                            },
                            Err(err) => Err(format!("Could not read ssh key: {}", err)),
                        }
                    }
                    None => Ok(()),
                };
                // Restore terminal
                // Clear screen
                if let Err(err) = ctx.terminal().clear_screen() {
                    error!("Could not clear screen screen: {}", err);
                }
                // Enter alternate mode
                if let Err(err) = ctx.terminal().enter_alternate_screen() {
                    error!("Could not enter alternate screen: {}", err);
                }
                // Re-enable raw mode
                if let Err(err) = ctx.terminal().enable_raw_mode() {
                    error!("Failed to enter raw mode: {}", err);
                }
                // Unlock ports
                assert!(self.app.unlock_ports().is_ok());
                // Return result
                result
            }
        }
    }

    /// Add provided ssh key to config client
    pub(super) fn add_ssh_key(
        &mut self,
        host: &str,
        username: &str,
        rsa_key: &str,
    ) -> Result<(), String> {
        self.config_mut()
            .add_ssh_key(host, username, rsa_key)
            .map_err(|e| format!("Could not add SSH key: {}", e))
    }
}
