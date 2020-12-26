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
use super::{
    Color, ConfigClient, FileTransferActivity, InputField, InputMode, LogLevel, LogRecord,
    PopupType,
};
use crate::fs::explorer::{builder::FileExplorerBuilder, FileExplorer, FileSorting, GroupDirs};
use crate::system::environment;
use crate::system::sshkey_storage::SshKeyStorage;
// Ext
use std::env;
use std::path::PathBuf;

impl FileTransferActivity {
    /// ### log
    ///
    /// Add message to log events
    pub(super) fn log(&mut self, level: LogLevel, msg: &str) {
        // Create log record
        let record: LogRecord = LogRecord::new(level, msg);
        //Check if history overflows the size
        if self.log_records.len() + 1 > self.log_size {
            self.log_records.pop_back(); // Start cleaning events from back
        }
        // Eventually push front the new record
        self.log_records.push_front(record);
        // Set log index
        self.log_index = 0;
    }

    /// ### log_and_alert
    ///
    /// Add message to log events and also display it as an alert
    pub(super) fn log_and_alert(&mut self, level: LogLevel, msg: String) {
        // Set input mode
        let color: Color = match level {
            LogLevel::Error => Color::Red,
            LogLevel::Info => Color::Green,
            LogLevel::Warn => Color::Yellow,
        };
        self.log(level, msg.as_str());
        self.input_mode = InputMode::Popup(PopupType::Alert(color, msg));
    }

    /// ### create_quit_popup
    ///
    /// Create quit popup input mode (since must be shared between different input handlers)
    pub(super) fn create_disconnect_popup(&mut self) -> InputMode {
        InputMode::Popup(PopupType::YesNo(
            String::from("Are you sure you want to disconnect?"),
            FileTransferActivity::disconnect,
            FileTransferActivity::callback_nothing_to_do,
        ))
    }

    /// ### create_quit_popup
    ///
    /// Create quit popup input mode (since must be shared between different input handlers)
    pub(super) fn create_quit_popup(&mut self) -> InputMode {
        InputMode::Popup(PopupType::YesNo(
            String::from("Are you sure you want to quit?"),
            FileTransferActivity::disconnect_and_quit,
            FileTransferActivity::callback_nothing_to_do,
        ))
    }

    /// ### switch_input_field
    ///
    /// Switch input field based on current input field
    pub(super) fn switch_input_field(&mut self) {
        self.input_field = match self.input_field {
            InputField::Explorer => InputField::Logs,
            InputField::Logs => InputField::Explorer,
        }
    }

    /// ### init_config_client
    ///
    /// Initialize configuration client if possible.
    /// This function doesn't return errors.
    pub(super) fn init_config_client() -> Option<ConfigClient> {
        match environment::init_config_dir() {
            Ok(termscp_dir) => match termscp_dir {
                Some(termscp_dir) => {
                    // Make configuration file path and ssh keys path
                    let (config_path, ssh_keys_path): (PathBuf, PathBuf) =
                        environment::get_config_paths(termscp_dir.as_path());
                    match ConfigClient::new(config_path.as_path(), ssh_keys_path.as_path()) {
                        Ok(config_client) => Some(config_client),
                        Err(_) => None,
                    }
                }
                None => None,
            },
            Err(_) => None,
        }
    }

    /// ### make_ssh_storage
    ///
    /// Make ssh storage from `ConfigClient` if possible, empty otherwise
    pub(super) fn make_ssh_storage(cli: Option<&ConfigClient>) -> SshKeyStorage {
        match cli {
            Some(cli) => SshKeyStorage::storage_from_config(cli),
            None => SshKeyStorage::empty(),
        }
    }

    /// ### build_explorer
    ///
    /// Build explorer reading configuration from `ConfigClient`
    pub(super) fn build_explorer(cli: Option<&ConfigClient>) -> FileExplorer {
        FileExplorerBuilder::new()
            .with_file_sorting(FileSorting::ByName)
            .with_group_dirs(Some(GroupDirs::First))
            .with_stack_size(16)
            .build()
    }

    /// ### setup_text_editor
    ///
    /// Set text editor to use
    pub(super) fn setup_text_editor(&self) {
        if let Some(config_cli) = &self.config_cli {
            // Set text editor
            env::set_var("EDITOR", config_cli.get_text_editor());
        }
    }
}
