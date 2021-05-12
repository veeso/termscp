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
    pub(crate) fn action_local_rename(&mut self, input: String) {
        let entry: Option<FsEntry> = self.get_local_file_entry().cloned();
        if let Some(entry) = entry {
            let mut dst_path: PathBuf = PathBuf::from(input);
            // Check if path is relative
            if dst_path.as_path().is_relative() {
                let mut wrkdir: PathBuf = self.local().wrkdir.clone();
                wrkdir.push(dst_path);
                dst_path = wrkdir;
            }
            let full_path: PathBuf = entry.get_abs_path();
            // Rename file or directory and report status as popup
            match self.host.rename(&entry, dst_path.as_path()) {
                Ok(_) => {
                    // Reload files
                    let path: PathBuf = self.local().wrkdir.clone();
                    self.local_scan(path.as_path());
                    // Log
                    self.log(
                        LogLevel::Info,
                        format!(
                            "Renamed file \"{}\" to \"{}\"",
                            full_path.display(),
                            dst_path.display()
                        ),
                    );
                }
                Err(err) => {
                    self.log_and_alert(
                        LogLevel::Error,
                        format!("Could not rename file \"{}\": {}", full_path.display(), err),
                    );
                }
            }
        }
    }

    pub(crate) fn action_remote_rename(&mut self, input: String) {
        if let Some(idx) = self.get_remote_file_state() {
            let entry = self.remote().get(idx).cloned();
            if let Some(entry) = entry {
                let dst_path: PathBuf = PathBuf::from(input);
                let full_path: PathBuf = entry.get_abs_path();
                // Rename file or directory and report status as popup
                match self.client.as_mut().rename(&entry, dst_path.as_path()) {
                    Ok(_) => {
                        // Reload files
                        let path: PathBuf = self.remote().wrkdir.clone();
                        self.remote_scan(path.as_path());
                        // Log
                        self.log(
                            LogLevel::Info,
                            format!(
                                "Renamed file \"{}\" to \"{}\"",
                                full_path.display(),
                                dst_path.display()
                            ),
                        );
                    }
                    Err(err) => {
                        self.log_and_alert(
                            LogLevel::Error,
                            format!("Could not rename file \"{}\": {}", full_path.display(), err),
                        );
                    }
                }
            }
        }
    }
}
