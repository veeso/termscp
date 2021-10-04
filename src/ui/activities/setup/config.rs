//! ## SetupActivity
//!
//! `setup_activity` is the module which implements the Setup activity, which is the activity to
//! work on termscp configuration

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
// Locals
use super::SetupActivity;
// Ext
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::env;

impl SetupActivity {
    /// ### save_config
    ///
    /// Save configuration
    pub(super) fn save_config(&mut self) -> Result<(), String> {
        match self.config().write_config() {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("Could not save configuration: {}", err)),
        }
    }

    /// ### reset_config_changes
    ///
    /// Reset configuration changes; pratically read config from file, overwriting any change made
    /// since last write action
    pub(super) fn reset_config_changes(&mut self) -> Result<(), String> {
        self.config_mut()
            .read_config()
            .map_err(|e| format!("Could not reload configuration: {}", e))
    }

    /// ### save_theme
    ///
    /// Save theme to file
    pub(super) fn save_theme(&mut self) -> Result<(), String> {
        self.theme_provider()
            .save()
            .map_err(|e| format!("Could not save theme: {}", e))
    }

    /// ### reset_theme_changes
    ///
    /// Reset changes committed to theme
    pub(super) fn reset_theme_changes(&mut self) -> Result<(), String> {
        self.theme_provider()
            .load()
            .map_err(|e| format!("Could not restore theme: {}", e))
    }

    /// ### delete_ssh_key
    ///
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

    /// ### edit_ssh_key
    ///
    /// Edit selected ssh key
    pub(super) fn edit_ssh_key(&mut self, idx: usize) -> Result<(), String> {
        match self.context.as_mut() {
            None => Ok(()),
            Some(ctx) => {
                // Set editor if config client exists
                env::set_var("EDITOR", ctx.config().get_text_editor());
                // Prepare terminal
                if let Err(err) = disable_raw_mode() {
                    error!("Failed to disable raw mode: {}", err);
                }
                // Leave alternate mode
                ctx.leave_alternate_screen();
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
                ctx.clear_screen();
                // Enter alternate mode
                ctx.enter_alternate_screen();
                // Re-enable raw mode
                if let Err(err) = enable_raw_mode() {
                    error!("Failed to enter raw mode: {}", err);
                }
                // Return result
                result
            }
        }
    }

    /// ### add_ssh_key
    ///
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
