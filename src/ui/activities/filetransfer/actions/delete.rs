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
    pub(crate) fn action_local_delete(&mut self) {
        let entry: Option<FsEntry> = self.get_local_file_entry().cloned();
        if let Some(entry) = entry {
            let full_path: PathBuf = entry.get_abs_path();
            // Delete file or directory and report status as popup
            match self.host.remove(&entry) {
                Ok(_) => {
                    // Reload files
                    let p: PathBuf = self.local().wrkdir.clone();
                    self.local_scan(p.as_path());
                    // Log
                    self.log(
                        LogLevel::Info,
                        format!("Removed file \"{}\"", full_path.display()),
                    );
                }
                Err(err) => {
                    self.log_and_alert(
                        LogLevel::Error,
                        format!("Could not delete file \"{}\": {}", full_path.display(), err),
                    );
                }
            }
        }
    }

    pub(crate) fn action_remote_delete(&mut self) {
        if let Some(idx) = self.get_remote_file_idx() {
            // Check if file entry exists
            let entry = self.remote().get(idx).cloned();
            if let Some(entry) = entry {
                let full_path: PathBuf = entry.get_abs_path();
                // Delete file
                match self.client.remove(&entry) {
                    Ok(_) => {
                        self.reload_remote_dir();
                        self.log(
                            LogLevel::Info,
                            format!("Removed file \"{}\"", full_path.display()),
                        );
                    }
                    Err(err) => {
                        self.log_and_alert(
                            LogLevel::Error,
                            format!("Could not delete file \"{}\": {}", full_path.display(), err),
                        );
                    }
                }
            }
        }
    }
}
