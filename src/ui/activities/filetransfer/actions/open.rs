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
// deps
extern crate open;
// locals
use super::{FileTransferActivity, FsEntry, LogLevel, SelectedEntry};
// ext
use std::path::PathBuf;

impl FileTransferActivity {
    /// ### action_open_local
    ///
    /// Open local file
    pub(crate) fn action_open_local(&mut self) {
        let entries: Vec<FsEntry> = match self.get_local_selected_entries() {
            SelectedEntry::One(entry) => vec![entry],
            SelectedEntry::Many(entries) => entries,
            SelectedEntry::None => vec![],
        };
        entries
            .iter()
            .for_each(|x| self.action_open_local_file(x, None));
    }

    /// ### action_open_remote
    ///
    /// Open local file
    pub(crate) fn action_open_remote(&mut self) {
        let entries: Vec<FsEntry> = match self.get_remote_selected_entries() {
            SelectedEntry::One(entry) => vec![entry],
            SelectedEntry::Many(entries) => entries,
            SelectedEntry::None => vec![],
        };
        entries
            .iter()
            .for_each(|x| self.action_open_remote_file(x, None));
    }

    /// ### action_open_local_file
    ///
    /// Perform open lopcal file
    pub(crate) fn action_open_local_file(&mut self, entry: &FsEntry, open_with: Option<&str>) {
        let entry: FsEntry = entry.get_realfile();
        // Open file
        let result = match open_with {
            None => open::that(entry.get_abs_path().as_path()),
            Some(with) => open::with(entry.get_abs_path().as_path(), with),
        };
        // Log result
        match result {
            Ok(_) => self.log(
                LogLevel::Info,
                format!("Opened file `{}`", entry.get_abs_path().display(),),
            ),
            Err(err) => self.log(
                LogLevel::Error,
                format!(
                    "Failed to open filoe `{}`: {}",
                    entry.get_abs_path().display(),
                    err
                ),
            ),
        }
    }

    /// ### action_open_local
    ///
    /// Open remote file. The file is first downloaded to a temporary directory on localhost
    pub(crate) fn action_open_remote_file(&mut self, entry: &FsEntry, open_with: Option<&str>) {
        let entry: FsEntry = entry.get_realfile();
        // Download file
        let tmpfile: String =
            match self.get_cache_tmp_name(entry.get_name(), entry.get_ftype().as_deref()) {
                None => {
                    self.log(LogLevel::Error, String::from("Could not create tempdir"));
                    return;
                }
                Some(p) => p,
            };
        let cache: PathBuf = match self.cache.as_ref() {
            None => {
                self.log(LogLevel::Error, String::from("Could not create tempdir"));
                return;
            }
            Some(p) => p.path().to_path_buf(),
        };
        self.filetransfer_recv(&entry, cache.as_path(), Some(tmpfile.clone()));
        // Make file and open if file exists
        let mut tmp: PathBuf = cache;
        tmp.push(tmpfile.as_str());
        if tmp.exists() {
            // Open file
            let result = match open_with {
                None => open::that(tmp.as_path()),
                Some(with) => open::with(tmp.as_path(), with),
            };
            // Log result
            match result {
                Ok(_) => self.log(LogLevel::Info, format!("Opened file `{}`", tmp.display())),
                Err(err) => self.log(
                    LogLevel::Error,
                    format!("Failed to open filoe `{}`: {}", tmp.display(), err),
                ),
            }
        }
    }

    /// ### action_local_open_with
    ///
    /// Open selected file with provided application
    pub(crate) fn action_local_open_with(&mut self, with: &str) {
        let entries: Vec<FsEntry> = match self.get_local_selected_entries() {
            SelectedEntry::One(entry) => vec![entry],
            SelectedEntry::Many(entries) => entries,
            SelectedEntry::None => vec![],
        };
        // Open all entries
        entries
            .iter()
            .for_each(|x| self.action_open_local_file(x, Some(with)));
    }

    /// ### action_remote_open_with
    ///
    /// Open selected file with provided application
    pub(crate) fn action_remote_open_with(&mut self, with: &str) {
        let entries: Vec<FsEntry> = match self.get_remote_selected_entries() {
            SelectedEntry::One(entry) => vec![entry],
            SelectedEntry::Many(entries) => entries,
            SelectedEntry::None => vec![],
        };
        // Open all entries
        entries
            .iter()
            .for_each(|x| self.action_open_remote_file(x, Some(with)));
    }
}
