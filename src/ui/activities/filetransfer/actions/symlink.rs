//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

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
// locals
use super::{FileTransferActivity, LogLevel, SelectedEntry};

use std::path::PathBuf;

impl FileTransferActivity {
    /// Create symlink on localhost
    #[cfg(target_family = "unix")]
    pub(crate) fn action_local_symlink(&mut self, name: String) {
        if let SelectedEntry::One(entry) = self.get_local_selected_entries() {
            match self
                .host
                .symlink(PathBuf::from(name.as_str()).as_path(), entry.path())
            {
                Ok(_) => {
                    self.log(
                        LogLevel::Info,
                        format!(
                            "Created symlink at {}, pointing to {}",
                            name,
                            entry.path().display()
                        ),
                    );
                }
                Err(err) => {
                    self.log_and_alert(
                        LogLevel::Error,
                        format!("Could not create symlink: {}", err),
                    );
                }
            }
        }
    }

    #[cfg(target_family = "windows")]
    pub(crate) fn action_local_symlink(&mut self, _name: String) {
        self.mount_error("Symlinks are not supported on Windows hosts");
    }

    /// Copy file on remote
    pub(crate) fn action_remote_symlink(&mut self, name: String) {
        if let SelectedEntry::One(entry) = self.get_remote_selected_entries() {
            match self
                .client
                .symlink(PathBuf::from(name.as_str()).as_path(), entry.path())
            {
                Ok(_) => {
                    self.log(
                        LogLevel::Info,
                        format!(
                            "Created symlink at {}, pointing to {}",
                            name,
                            entry.path().display()
                        ),
                    );
                }
                Err(err) => {
                    self.log_and_alert(
                        LogLevel::Error,
                        format!(
                            "Could not create symlink pointing to {}: {}",
                            entry.path().display(),
                            err
                        ),
                    );
                }
            }
        }
    }
}
