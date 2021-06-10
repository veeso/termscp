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
use super::{FileTransferActivity, FsEntry, LogLevel};

impl FileTransferActivity {
    /// ### action_open_local
    ///
    /// Open local file
    pub(crate) fn action_open_local(&mut self, entry: FsEntry, open_with: Option<String>) {
        let real_entry: FsEntry = entry.get_realfile();
        if let FsEntry::File(file) = real_entry {
            // Open file
            let result = match open_with {
                None => open::that(file.abs_path.as_path()),
                Some(with) => open::with(file.abs_path.as_path(), with),
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
    }

    /// ### action_open_local
    ///
    /// Open remote file. The file is first downloaded to a temporary directory on localhost
    pub(crate) fn action_open_remote(&mut self, entry: FsEntry, open_with: Option<String>) {
        let real_entry: FsEntry = entry.get_realfile();
        if let FsEntry::File(file) = real_entry {
            // Download file
            let tmp = match self.download_file_as_temp(&file) {
                Ok(f) => f,
                Err(err) => {
                    self.log(
                        LogLevel::Error,
                        format!("Could not open `{}`: {}", file.abs_path.display(), err),
                    );
                    return;
                }
            };
            // Open file
            let result = match open_with {
                None => open::that(tmp.as_path()),
                Some(with) => open::with(tmp.as_path(), with),
            };
            // Log result
            match result {
                Ok(_) => self.log(
                    LogLevel::Info,
                    format!("Opened file `{}`", entry.get_abs_path().display()),
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
    }
}
