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
use crate::config::themes::Theme;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::env;
use tuirealm::tui::style::Color;
use tuirealm::{Payload, Value};

impl SetupActivity {
    /// ### action_save_config
    ///
    /// Save configuration
    pub(super) fn action_save_all(&mut self) -> Result<(), String> {
        // Collect input values
        self.collect_input_values();
        self.save_config()?;
        // save theme
        self.collect_styles()
            .map_err(|e| format!("'{}' has an invalid color", e))?;
        self.save_theme()
    }

    /// ### action_reset_config
    ///
    /// Reset configuration input fields
    pub(super) fn action_reset_config(&mut self) -> Result<(), String> {
        match self.reset_config_changes() {
            Err(err) => Err(err),
            Ok(_) => {
                self.load_input_values();
                Ok(())
            }
        }
    }

    /// ### action_reset_theme
    ///
    /// Reset configuration input fields
    pub(super) fn action_reset_theme(&mut self) -> Result<(), String> {
        match self.reset_theme_changes() {
            Err(err) => Err(err),
            Ok(_) => {
                self.load_styles();
                Ok(())
            }
        }
    }

    /// ### action_delete_ssh_key
    ///
    /// delete of a ssh key
    pub(super) fn action_delete_ssh_key(&mut self) {
        // Get key
        // get index
        let idx: Option<usize> = match self.view.get_state(super::COMPONENT_LIST_SSH_KEYS) {
            Some(Payload::One(Value::Usize(idx))) => Some(idx),
            _ => None,
        };
        if let Some(idx) = idx {
            let key: Option<String> = self.config().iter_ssh_keys().nth(idx).cloned();
            if let Some(key) = key {
                match self.config().get_ssh_key(&key) {
                    Ok(opt) => {
                        if let Some((host, username, _)) = opt {
                            if let Err(err) = self.delete_ssh_key(host.as_str(), username.as_str())
                            {
                                // Report error
                                self.mount_error(err.as_str());
                            }
                        }
                    }
                    Err(err) => {
                        // Report error
                        self.mount_error(
                            format!("Could not get ssh key \"{}\": {}", key, err).as_str(),
                        );
                    }
                }
            }
        }
    }

    /// ### action_new_ssh_key
    ///
    /// Create a new ssh key
    pub(super) fn action_new_ssh_key(&mut self) {
        // get parameters
        let host: String = match self.view.get_state(super::COMPONENT_INPUT_SSH_HOST) {
            Some(Payload::One(Value::Str(host))) => host,
            _ => String::new(),
        };
        let username: String = match self.view.get_state(super::COMPONENT_INPUT_SSH_USERNAME) {
            Some(Payload::One(Value::Str(user))) => user,
            _ => String::new(),
        };
        // Prepare text editor
        env::set_var("EDITOR", self.config().get_text_editor());
        let placeholder: String = format!("# Type private SSH key for {}@{}\n", username, host);
        // Put input mode back to normal
        if let Err(err) = disable_raw_mode() {
            error!("Failed to disable raw mode: {}", err);
        }
        // Leave alternate mode
        #[cfg(not(target_os = "windows"))]
        if let Some(ctx) = self.context.as_mut() {
            ctx.leave_alternate_screen();
        }
        // Re-enable raw mode
        if let Err(err) = enable_raw_mode() {
            error!("Failed to enter raw mode: {}", err);
        }
        // Write key to file
        match edit::edit(placeholder.as_bytes()) {
            Ok(rsa_key) => {
                // Remove placeholder from `rsa_key`
                let rsa_key: String = rsa_key.as_str().replace(placeholder.as_str(), "");
                if rsa_key.is_empty() {
                    // Report error: empty key
                    self.mount_error("SSH key is empty!");
                } else {
                    // Add key
                    if let Err(err) =
                        self.add_ssh_key(host.as_str(), username.as_str(), rsa_key.as_str())
                    {
                        self.mount_error(
                            format!("Could not create new private key: {}", err).as_str(),
                        );
                    }
                }
            }
            Err(err) => {
                // Report error
                self.mount_error(format!("Could not write private key to file: {}", err).as_str());
            }
        }
        // Restore terminal
        #[cfg(not(target_os = "windows"))]
        if let Some(ctx) = self.context.as_mut() {
            // Clear screen
            ctx.clear_screen();
            // Enter alternate mode
            ctx.enter_alternate_screen();
        }
    }

    /// ### set_color
    ///
    /// Given a component and a color, save the color into the theme
    pub(super) fn action_save_color(&mut self, component: &str, color: Color) {
        let theme: &mut Theme = self.theme_mut();
        match component {
            super::COMPONENT_COLOR_AUTH_ADDR => {
                theme.auth_address = color;
            }
            super::COMPONENT_COLOR_AUTH_BOOKMARKS => {
                theme.auth_bookmarks = color;
            }
            super::COMPONENT_COLOR_AUTH_PASSWORD => {
                theme.auth_password = color;
            }
            super::COMPONENT_COLOR_AUTH_PORT => {
                theme.auth_port = color;
            }
            super::COMPONENT_COLOR_AUTH_PROTOCOL => {
                theme.auth_protocol = color;
            }
            super::COMPONENT_COLOR_AUTH_RECENTS => {
                theme.auth_recents = color;
            }
            super::COMPONENT_COLOR_AUTH_USERNAME => {
                theme.auth_username = color;
            }
            super::COMPONENT_COLOR_MISC_ERROR => {
                theme.misc_error_dialog = color;
            }
            super::COMPONENT_COLOR_MISC_INPUT => {
                theme.misc_input_dialog = color;
            }
            super::COMPONENT_COLOR_MISC_KEYS => {
                theme.misc_keys = color;
            }
            super::COMPONENT_COLOR_MISC_QUIT => {
                theme.misc_quit_dialog = color;
            }
            super::COMPONENT_COLOR_MISC_SAVE => {
                theme.misc_save_dialog = color;
            }
            super::COMPONENT_COLOR_MISC_WARN => {
                theme.misc_warn_dialog = color;
            }
            super::COMPONENT_COLOR_TRANSFER_EXPLORER_LOCAL_BG => {
                theme.transfer_local_explorer_background = color;
            }
            super::COMPONENT_COLOR_TRANSFER_EXPLORER_LOCAL_FG => {
                theme.transfer_local_explorer_foreground = color;
            }
            super::COMPONENT_COLOR_TRANSFER_EXPLORER_LOCAL_HG => {
                theme.transfer_local_explorer_highlighted = color;
            }
            super::COMPONENT_COLOR_TRANSFER_EXPLORER_REMOTE_BG => {
                theme.transfer_remote_explorer_background = color;
            }
            super::COMPONENT_COLOR_TRANSFER_EXPLORER_REMOTE_FG => {
                theme.transfer_remote_explorer_foreground = color;
            }
            super::COMPONENT_COLOR_TRANSFER_EXPLORER_REMOTE_HG => {
                theme.transfer_remote_explorer_highlighted = color;
            }
            super::COMPONENT_COLOR_TRANSFER_LOG_BG => {
                theme.transfer_log_background = color;
            }
            super::COMPONENT_COLOR_TRANSFER_LOG_WIN => {
                theme.transfer_log_window = color;
            }
            super::COMPONENT_COLOR_TRANSFER_PROG_BAR_FULL => {
                theme.transfer_progress_bar_full = color;
            }
            super::COMPONENT_COLOR_TRANSFER_PROG_BAR_PARTIAL => {
                theme.transfer_progress_bar_partial = color;
            }
            super::COMPONENT_COLOR_TRANSFER_STATUS_HIDDEN => {
                theme.transfer_status_hidden = color;
            }
            super::COMPONENT_COLOR_TRANSFER_STATUS_SORTING => {
                theme.transfer_status_sorting = color;
            }
            super::COMPONENT_COLOR_TRANSFER_STATUS_SYNC => {
                theme.transfer_status_sync_browsing = color;
            }
            _ => {}
        }
    }
}
