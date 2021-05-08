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
    /// ### action_local_copy
    ///
    /// Copy file on local
    pub(crate) fn action_local_copy(&mut self, input: String) {
        if let Some(idx) = self.get_local_file_idx() {
            let dest_path: PathBuf = PathBuf::from(input);
            let entry: FsEntry = self.local().get(idx).unwrap().clone();
            match self.host.copy(&entry, dest_path.as_path()) {
                Ok(_) => {
                    self.log(
                        LogLevel::Info,
                        format!(
                            "Copied \"{}\" to \"{}\"",
                            entry.get_abs_path().display(),
                            dest_path.display()
                        ),
                    );
                    // Reload entries
                    let wrkdir: PathBuf = self.local().wrkdir.clone();
                    self.local_scan(wrkdir.as_path());
                }
                Err(err) => self.log_and_alert(
                    LogLevel::Error,
                    format!(
                        "Could not copy \"{}\" to \"{}\": {}",
                        entry.get_abs_path().display(),
                        dest_path.display(),
                        err
                    ),
                ),
            }
        }
    }

    /// ### action_remote_copy
    ///
    /// Copy file on remote
    pub(crate) fn action_remote_copy(&mut self, input: String) {
        if let Some(idx) = self.get_remote_file_idx() {
            let dest_path: PathBuf = PathBuf::from(input);
            let entry: FsEntry = self.remote().get(idx).unwrap().clone();
            match self.client.as_mut().copy(&entry, dest_path.as_path()) {
                Ok(_) => {
                    self.log(
                        LogLevel::Info,
                        format!(
                            "Copied \"{}\" to \"{}\"",
                            entry.get_abs_path().display(),
                            dest_path.display()
                        ),
                    );
                    self.reload_remote_dir();
                }
                Err(err) => self.log_and_alert(
                    LogLevel::Error,
                    format!(
                        "Could not copy \"{}\" to \"{}\": {}",
                        entry.get_abs_path().display(),
                        dest_path.display(),
                        err
                    ),
                ),
            }
        }
    }
}
