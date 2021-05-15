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
use super::{FileTransferActivity, FsEntry, LogLevel, SelectedEntry};
use std::path::{Path, PathBuf};

impl FileTransferActivity {
    /// ### action_local_copy
    ///
    /// Copy file on local
    pub(crate) fn action_local_copy(&mut self, input: String) {
        match self.get_local_selected_entries() {
            SelectedEntry::One(entry) => {
                let dest_path: PathBuf = PathBuf::from(input);
                self.local_copy_file(&entry, dest_path.as_path());
                // Reload entries
                self.reload_local_dir();
            }
            SelectedEntry::Multi(entries) => {
                // Try to copy each file to Input/{FILE_NAME}
                let base_path: PathBuf = PathBuf::from(input);
                // Iter files
                for entry in entries.iter() {
                    let mut dest_path: PathBuf = base_path.clone();
                    dest_path.push(entry.get_name());
                    self.local_copy_file(entry, dest_path.as_path());
                }
                // Reload entries
                self.reload_local_dir();
            }
            SelectedEntry::None => {}
        }
    }

    /// ### action_remote_copy
    ///
    /// Copy file on remote
    pub(crate) fn action_remote_copy(&mut self, input: String) {
        match self.get_remote_selected_entries() {
            SelectedEntry::One(entry) => {
                let dest_path: PathBuf = PathBuf::from(input);
                self.remote_copy_file(&entry, dest_path.as_path());
                // Reload entries
                self.reload_remote_dir();
            }
            SelectedEntry::Multi(entries) => {
                // Try to copy each file to Input/{FILE_NAME}
                let base_path: PathBuf = PathBuf::from(input);
                // Iter files
                for entry in entries.iter() {
                    let mut dest_path: PathBuf = base_path.clone();
                    dest_path.push(entry.get_name());
                    self.remote_copy_file(entry, dest_path.as_path());
                }
                // Reload entries
                self.reload_remote_dir();
            }
            SelectedEntry::None => {}
        }
    }

    fn local_copy_file(&mut self, entry: &FsEntry, dest: &Path) {
        match self.host.copy(entry, dest) {
            Ok(_) => {
                self.log(
                    LogLevel::Info,
                    format!(
                        "Copied \"{}\" to \"{}\"",
                        entry.get_abs_path().display(),
                        dest.display()
                    ),
                );
            }
            Err(err) => self.log_and_alert(
                LogLevel::Error,
                format!(
                    "Could not copy \"{}\" to \"{}\": {}",
                    entry.get_abs_path().display(),
                    dest.display(),
                    err
                ),
            ),
        }
    }

    fn remote_copy_file(&mut self, entry: &FsEntry, dest: &Path) {
        match self.client.as_mut().copy(entry, dest) {
            Ok(_) => {
                self.log(
                    LogLevel::Info,
                    format!(
                        "Copied \"{}\" to \"{}\"",
                        entry.get_abs_path().display(),
                        dest.display()
                    ),
                );
            }
            Err(err) => self.log_and_alert(
                LogLevel::Error,
                format!(
                    "Could not copy \"{}\" to \"{}\": {}",
                    entry.get_abs_path().display(),
                    dest.display(),
                    err
                ),
            ),
        }
    }
}
