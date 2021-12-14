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
use super::{Entry, FileTransferActivity, LogLevel, SelectedEntry};

use remotefs::RemoteErrorType;
use std::path::{Path, PathBuf};

impl FileTransferActivity {
    pub(crate) fn action_local_rename(&mut self, input: String) {
        match self.get_local_selected_entries() {
            SelectedEntry::One(entry) => {
                let dest_path: PathBuf = PathBuf::from(input);
                self.local_rename_file(&entry, dest_path.as_path());
            }
            SelectedEntry::Many(entries) => {
                // Try to copy each file to Input/{FILE_NAME}
                let base_path: PathBuf = PathBuf::from(input);
                // Iter files
                for entry in entries.iter() {
                    let mut dest_path: PathBuf = base_path.clone();
                    dest_path.push(entry.name());
                    self.local_rename_file(entry, dest_path.as_path());
                }
            }
            SelectedEntry::None => {}
        }
    }

    pub(crate) fn action_remote_rename(&mut self, input: String) {
        match self.get_remote_selected_entries() {
            SelectedEntry::One(entry) => {
                let dest_path: PathBuf = PathBuf::from(input);
                self.remote_rename_file(&entry, dest_path.as_path());
            }
            SelectedEntry::Many(entries) => {
                // Try to copy each file to Input/{FILE_NAME}
                let base_path: PathBuf = PathBuf::from(input);
                // Iter files
                for entry in entries.iter() {
                    let mut dest_path: PathBuf = base_path.clone();
                    dest_path.push(entry.name());
                    self.remote_rename_file(entry, dest_path.as_path());
                }
            }
            SelectedEntry::None => {}
        }
    }

    fn local_rename_file(&mut self, entry: &Entry, dest: &Path) {
        match self.host.rename(entry, dest) {
            Ok(_) => {
                self.log(
                    LogLevel::Info,
                    format!(
                        "Moved \"{}\" to \"{}\"",
                        entry.path().display(),
                        dest.display()
                    ),
                );
            }
            Err(err) => self.log_and_alert(
                LogLevel::Error,
                format!(
                    "Could not move \"{}\" to \"{}\": {}",
                    entry.path().display(),
                    dest.display(),
                    err
                ),
            ),
        }
    }

    fn remote_rename_file(&mut self, entry: &Entry, dest: &Path) {
        match self.client.as_mut().mov(entry.path(), dest) {
            Ok(_) => {
                self.log(
                    LogLevel::Info,
                    format!(
                        "Moved \"{}\" to \"{}\"",
                        entry.path().display(),
                        dest.display()
                    ),
                );
            }
            Err(err) if err.kind == RemoteErrorType::UnsupportedFeature => {
                self.tricky_move(entry, dest);
            }
            Err(err) => self.log_and_alert(
                LogLevel::Error,
                format!(
                    "Could not move \"{}\" to \"{}\": {}",
                    entry.path().display(),
                    dest.display(),
                    err
                ),
            ),
        }
    }

    /// Tricky move will be used whenever copy command is not available on remote host.
    /// It basically uses the tricky_copy function, then it just deletes the previous entry (`entry`)
    fn tricky_move(&mut self, entry: &Entry, dest: &Path) {
        debug!(
            "Using tricky-move to move entry {} to {}",
            entry.path().display(),
            dest.display()
        );
        if self.tricky_copy(entry.clone(), dest).is_ok() {
            // Delete remote existing entry
            debug!("Tricky-copy worked; removing existing remote entry");
            match self.client.remove_dir_all(entry.path()) {
                Ok(_) => self.log(
                    LogLevel::Info,
                    format!(
                        "Moved \"{}\" to \"{}\"",
                        entry.path().display(),
                        dest.display()
                    ),
                ),
                Err(err) => self.log_and_alert(
                    LogLevel::Error,
                    format!(
                        "Copied \"{}\" to \"{}\"; but failed to remove src: {}",
                        entry.path().display(),
                        dest.display(),
                        err
                    ),
                ),
            }
        } else {
            error!("Tricky move aborted due to tricky-copy failure");
        }
    }
}
