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
use crate::ui::layout::Payload;
// Ext
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::env;

impl SetupActivity {
    /// ### action_save_config
    ///
    /// Save configuration
    pub(super) fn action_save_config(&mut self) -> Result<(), String> {
        // Collect input values
        self.collect_input_values();
        self.save_config()
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

    /// ### action_delete_ssh_key
    ///
    /// delete of a ssh key
    pub(super) fn action_delete_ssh_key(&mut self) {
        // Get key
        if let Some(config_cli) = self.context.as_mut().unwrap().config_client.as_mut() {
            // get index
            let idx: Option<usize> = match self.view.get_value(super::COMPONENT_LIST_SSH_KEYS) {
                Some(Payload::Unsigned(idx)) => Some(idx),
                _ => None,
            };
            if let Some(idx) = idx {
                let key: Option<String> = config_cli.iter_ssh_keys().nth(idx).cloned();
                if let Some(key) = key {
                    match config_cli.get_ssh_key(&key) {
                        Ok(opt) => {
                            if let Some((host, username, _)) = opt {
                                if let Err(err) =
                                    self.delete_ssh_key(host.as_str(), username.as_str())
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
    }

    /// ### action_new_ssh_key
    ///
    /// Create a new ssh key
    pub(super) fn action_new_ssh_key(&mut self) {
        if let Some(cli) = self.context.as_mut().unwrap().config_client.as_mut() {
            // get parameters
            let host: String = match self.view.get_value(super::COMPONENT_INPUT_SSH_HOST) {
                Some(Payload::Text(host)) => host,
                _ => String::new(),
            };
            let username: String = match self.view.get_value(super::COMPONENT_INPUT_SSH_USERNAME) {
                Some(Payload::Text(user)) => user,
                _ => String::new(),
            };
            // Prepare text editor
            env::set_var("EDITOR", cli.get_text_editor());
            let placeholder: String = format!("# Type private SSH key for {}@{}\n", username, host);
            // Put input mode back to normal
            let _ = disable_raw_mode();
            // Leave alternate mode
            if let Some(ctx) = self.context.as_mut() {
                ctx.leave_alternate_screen();
            }
            // Re-enable raw mode
            let _ = enable_raw_mode();
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
                    self.mount_error(
                        format!("Could not write private key to file: {}", err).as_str(),
                    );
                }
            }
            // Restore terminal
            if let Some(ctx) = self.context.as_mut() {
                // Clear screen
                ctx.clear_screen();
                // Enter alternate mode
                ctx.enter_alternate_screen();
            }
        }
    }
}
