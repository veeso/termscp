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
use super::{FileTransferActivity, FsEntry, LogLevel};
use std::path::PathBuf;

impl FileTransferActivity {
    pub(crate) fn action_edit_local_file(&mut self) {
        if self.get_local_file_entry().is_some() {
            let fsentry: FsEntry = self.get_local_file_entry().unwrap().clone();
            // Check if file
            if fsentry.is_file() {
                self.log(
                    LogLevel::Info,
                    format!("Opening file \"{}\"...", fsentry.get_abs_path().display()),
                );
                // Edit file
                match self.edit_local_file(fsentry.get_abs_path().as_path()) {
                    Ok(_) => {
                        // Reload directory
                        let pwd: PathBuf = self.local().wrkdir.clone();
                        self.local_scan(pwd.as_path());
                    }
                    Err(err) => self.log_and_alert(LogLevel::Error, err),
                }
            }
        }
    }

    pub(crate) fn action_edit_remote_file(&mut self) {
        if self.get_remote_file_entry().is_some() {
            let fsentry: FsEntry = self.get_remote_file_entry().unwrap().clone();
            // Check if file
            if let FsEntry::File(file) = fsentry.clone() {
                self.log(
                    LogLevel::Info,
                    format!("Opening file \"{}\"...", fsentry.get_abs_path().display()),
                );
                // Edit file
                match self.edit_remote_file(&file) {
                    Ok(_) => {
                        // Reload directory
                        let pwd: PathBuf = self.remote().wrkdir.clone();
                        self.remote_scan(pwd.as_path());
                    }
                    Err(err) => self.log_and_alert(LogLevel::Error, err),
                }
            }
        }
    }
}
