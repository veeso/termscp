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
use super::{ConfigClient, FileTransferActivity, LogLevel, LogRecord};
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
        // Update log
        let msg = self.update_logbox();
        self.update(msg);
    }

    /// ### log_and_alert
    ///
    /// Add message to log events and also display it as an alert
    pub(super) fn log_and_alert(&mut self, level: LogLevel, msg: String) {
        self.log(level, msg.as_str());
        self.mount_error(msg.as_str());
        // Update log
        let msg = self.update_logbox();
        self.update(msg);
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
    fn build_explorer(cli: Option<&ConfigClient>) -> FileExplorerBuilder {
        let mut builder: FileExplorerBuilder = FileExplorerBuilder::new();
        // Set common keys
        builder
            .with_file_sorting(FileSorting::ByName)
            .with_stack_size(16);
        match &cli {
            Some(cli) => {
                builder // Build according to current configuration
                    .with_group_dirs(cli.get_group_dirs())
                    .with_hidden_files(cli.get_show_hidden_files());
            }
            None => {
                builder // Build default
                    .with_group_dirs(Some(GroupDirs::First));
            }
        };
        builder
    }

    /// ### build_local_explorer
    ///
    /// Build a file explorer with local host setup
    pub(super) fn build_local_explorer(cli: Option<&ConfigClient>) -> FileExplorer {
        let mut builder = Self::build_explorer(cli);
        if let Some(cli) = cli {
            builder.with_formatter(cli.get_local_file_fmt().as_deref());
        }
        builder.build()
    }

    /// ### build_remote_explorer
    ///
    /// Build a file explorer with remote host setup
    pub(super) fn build_remote_explorer(cli: Option<&ConfigClient>) -> FileExplorer {
        let mut builder = Self::build_explorer(cli);
        if let Some(cli) = cli {
            builder.with_formatter(cli.get_remote_file_fmt().as_deref());
        }
        builder.build()
    }

    /// ### build_found_explorer
    ///
    /// Build explorer reading from `ConfigClient`, for found result (has some differences)
    pub(super) fn build_found_explorer() -> FileExplorer {
        FileExplorerBuilder::new()
            .with_file_sorting(FileSorting::ByName)
            .with_group_dirs(Some(GroupDirs::First))
            .with_hidden_files(true)
            .with_stack_size(0)
            .with_formatter(Some("{NAME} {SYMLINK}"))
            .build()
    }

    /// ### setup_text_editor
    ///
    /// Set text editor to use
    pub(super) fn setup_text_editor(&self) {
        if let Some(config_cli) = self.context.as_ref().unwrap().config_client.as_ref() {
            // Set text editor
            env::set_var("EDITOR", config_cli.get_text_editor());
        }
    }

    /// ### read_input_event
    ///
    /// Read one event.
    /// Returns whether at least one event has been handled
    pub(super) fn read_input_event(&mut self) -> bool {
        if let Ok(Some(event)) = self.context.as_ref().unwrap().input_hnd.read_event() {
            // Handle event
            let msg = self.view.on(event);
            self.update(msg);
            // Return true
            true
        } else {
            // Error
            false
        }
    }
}
