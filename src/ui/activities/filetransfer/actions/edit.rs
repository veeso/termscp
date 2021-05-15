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

impl FileTransferActivity {
    pub(crate) fn action_edit_local_file(&mut self) {
        let entries: Vec<FsEntry> = match self.get_local_selected_entries() {
            SelectedEntry::One(entry) => vec![entry],
            SelectedEntry::Many(entries) => entries,
            SelectedEntry::None => vec![],
        };
        // Edit all entries
        for entry in entries.iter() {
            // Check if file
            if entry.is_file() {
                self.log(
                    LogLevel::Info,
                    format!("Opening file \"{}\"...", entry.get_abs_path().display()),
                );
                // Edit file
                if let Err(err) = self.edit_local_file(entry.get_abs_path().as_path()) {
                    self.log_and_alert(LogLevel::Error, err);
                }
            }
        }
        // Reload entries
        self.reload_local_dir();
    }

    pub(crate) fn action_edit_remote_file(&mut self) {
        let entries: Vec<FsEntry> = match self.get_remote_selected_entries() {
            SelectedEntry::One(entry) => vec![entry],
            SelectedEntry::Many(entries) => entries,
            SelectedEntry::None => vec![],
        };
        // Edit all entries
        for entry in entries.iter() {
            // Check if file
            if let FsEntry::File(file) = entry {
                self.log(
                    LogLevel::Info,
                    format!("Opening file \"{}\"...", entry.get_abs_path().display()),
                );
                // Edit file
                if let Err(err) = self.edit_remote_file(&file) {
                    self.log_and_alert(LogLevel::Error, err);
                }
            }
        }
        // Reload entries
        self.reload_remote_dir();
    }
}
