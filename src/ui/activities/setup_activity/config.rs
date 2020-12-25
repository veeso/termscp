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
use super::{ConfigClient, Popup, SetupActivity};
use crate::system::environment;
// Ext
use std::env;
use std::path::PathBuf;

impl SetupActivity {
    /// ### init_config_dir
    ///
    /// Initialize configuration directory
    pub(super) fn init_config_client(&mut self) {
        match environment::init_config_dir() {
            Ok(config_dir) => match config_dir {
                Some(config_dir) => {
                    // Get paths
                    let (config_file, ssh_dir): (PathBuf, PathBuf) =
                        environment::get_config_paths(config_dir.as_path());
                    // Create config client
                    match ConfigClient::new(config_file.as_path(), ssh_dir.as_path()) {
                        Ok(cli) => self.config_cli = Some(cli),
                        Err(err) => {
                            self.popup = Some(Popup::Fatal(format!(
                                "Could not initialize configuration client: {}",
                                err
                            )))
                        }
                    }
                }
                None => {
                    self.popup = Some(Popup::Fatal(format!(
                        "No configuration directory is available on your system"
                    )))
                }
            },
            Err(err) => {
                self.popup = Some(Popup::Fatal(format!(
                    "Could not initialize configuration directory: {}",
                    err
                )))
            }
        }
    }

    /// ### save_config
    ///
    /// Save configuration
    pub(super) fn save_config(&mut self) -> Result<(), String> {
        match &self.config_cli {
            Some(cli) => match cli.write_config() {
                Ok(_) => Ok(()),
                Err(err) => Err(format!("Could not save configuration: {}", err)),
            },
            None => Ok(()),
        }
    }

    /// ### reset_config_changes
    ///
    /// Reset configuration changes; pratically read config from file, overwriting any change made
    /// since last write action
    pub(super) fn reset_config_changes(&mut self) -> Result<(), String> {
        match self.config_cli.as_mut() {
            Some(cli) => match cli.read_config() {
                Ok(_) => Ok(()),
                Err(err) => Err(format!("Could not restore configuration: {}", err)),
            },
            None => Ok(()),
        }
    }

    /// ### delete_ssh_key
    ///
    /// Delete ssh key from config cli
    pub(super) fn delete_ssh_key(&mut self, host: &str, username: &str) -> Result<(), String> {
        match self.config_cli.as_mut() {
            Some(cli) => match cli.del_ssh_key(host, username) {
                Ok(_) => Ok(()),
                Err(err) => Err(format!(
                    "Could not delete ssh key \"{}@{}\": {}",
                    host, username, err
                )),
            },
            None => Ok(()),
        }
    }

    /// ### edit_ssh_key
    ///
    /// Edit selected ssh key
    pub(super) fn edit_ssh_key(&mut self) -> Result<(), String> {
        match self.config_cli.as_ref() {
            Some(cli) => {
                // Set text editor
                env::set_var("EDITOR", cli.get_text_editor());
                // Check if key exists
                match cli.iter_ssh_keys().nth(self.ssh_key_idx) {
                    Some(key) => {
                        // Get key path
                        match cli.get_ssh_key(key) {
                            Ok(ssh_key) => match ssh_key {
                                None => Ok(()),
                                Some((_, _, key_path)) => match edit::edit_file(key_path.as_path())
                                {
                                    Ok(_) => Ok(()),
                                    Err(err) => Err(format!("Could not edit ssh key: {}", err)),
                                },
                            },
                            Err(err) => Err(format!("Could not read ssh key: {}", err)),
                        }
                    }
                    None => Ok(()),
                }
            }
            None => Ok(()),
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
        match self.config_cli.as_mut() {
            Some(cli) => {
                // Add key to client
                match cli.add_ssh_key(host, username, rsa_key) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(format!("Could not add SSH key: {}", err)),
                }
            }
            None => Ok(()),
        }
    }
}
