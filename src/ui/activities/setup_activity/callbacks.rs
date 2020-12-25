//! ## SetupActivity
//!
//! `setup_activity` is the module which implements the Setup activity, which is the activity to
//! work on termscp configuration

/*
*
*   Copyright (C) 2020 Christian Visintin - christian.visintin1997@gmail.com
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

// Locals
use super::{Color, Popup, SetupActivity};
// Ext
use std::env;

impl SetupActivity {
    /// ### callback_nothing_to_do
    ///
    /// Self titled
    pub(super) fn callback_nothing_to_do(&mut self) {}

    /// ### callback_save_config_and_quit
    ///
    /// Save configuration and quit
    pub(super) fn callback_save_config_and_quit(&mut self) {
        match self.save_config() {
            Ok(_) => self.quit = true, // Quit after successful save
            Err(err) => self.popup = Some(Popup::Alert(Color::Red, err)), // Show error and don't quit
        }
    }

    /// ### callback_save_config
    ///
    /// Save configuration callback
    pub(super) fn callback_save_config(&mut self) {
        if let Err(err) = self.save_config() {
            self.popup = Some(Popup::Alert(Color::Red, err)); // Show save error
        }
    }

    /// ### callback_reset_config_changes
    ///
    /// Reset config changes callback
    pub(super) fn callback_reset_config_changes(&mut self) {
        if let Err(err) = self.reset_config_changes() {
            self.popup = Some(Popup::Alert(Color::Red, err)); // Show reset error
        }
    }

    /// ### callback_delete_ssh_key
    ///
    /// Callback for performing the delete of a ssh key
    pub(super) fn callback_delete_ssh_key(&mut self) {
        // Get key
        if let Some(config_cli) = self.config_cli.as_mut() {
            let key: Option<String> = match config_cli.iter_ssh_keys().nth(self.ssh_key_idx) {
                Some(k) => Some(k.clone()),
                None => None,
            };
            if let Some(key) = key {
                match config_cli.get_ssh_key(&key) {
                    Ok(opt) => {
                        if let Some((host, username, _)) = opt {
                            if let Err(err) = self.delete_ssh_key(host.as_str(), username.as_str())
                            {
                                // Report error
                                self.popup = Some(Popup::Alert(Color::Red, err));
                            }
                        }
                    }
                    Err(err) => {
                        self.popup = Some(Popup::Alert(
                            Color::Red,
                            format!("Could not get ssh key \"{}\": {}", key, err),
                        ))
                    } // Report error
                }
            }
        }
    }

    /// ### callback_new_ssh_key
    ///
    /// Create a new ssh key with provided parameters
    pub(super) fn callback_new_ssh_key(&mut self, host: String, username: String) {
        if let Some(cli) = self.config_cli.as_ref() {
            // Prepare text editor
            env::set_var("EDITOR", cli.get_text_editor());
            let placeholder: String = format!("# Type private SSH key for {}@{}", username, host);
            // Write key to file
            match edit::edit(placeholder.as_bytes()) {
                Ok(rsa_key) => {
                    // Remove placeholder from `rsa_key`
                    let rsa_key: String = rsa_key.as_str().replace(placeholder.as_str(), "");
                    // Add key
                    if let Err(err) =
                        self.add_ssh_key(host.as_str(), username.as_str(), rsa_key.as_str())
                    {
                        self.popup = Some(Popup::Alert(
                            Color::Red,
                            format!("Could not create new private key: {}", err),
                        ))
                    }
                }
                Err(err) => {
                    // Report error
                    self.popup = Some(Popup::Alert(
                        Color::Red,
                        format!("Could not write private key to file: {}", err),
                    ))
                }
            }
        }
    }
}
