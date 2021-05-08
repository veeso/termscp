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
use super::super::browser::FileExplorerTab;
use super::{FileTransferActivity, FsEntry, LogLevel};

use std::path::PathBuf;

impl FileTransferActivity {
    pub(crate) fn action_local_find(&mut self, input: String) -> Result<Vec<FsEntry>, String> {
        match self.host.find(input.as_str()) {
            Ok(entries) => Ok(entries),
            Err(err) => Err(format!("Could not search for files: {}", err)),
        }
    }

    pub(crate) fn action_remote_find(&mut self, input: String) -> Result<Vec<FsEntry>, String> {
        match self.client.as_mut().find(input.as_str()) {
            Ok(entries) => Ok(entries),
            Err(err) => Err(format!("Could not search for files: {}", err)),
        }
    }

    pub(crate) fn action_find_changedir(&mut self, idx: usize) {
        // Match entry
        if let Some(entry) = self.found().as_ref().unwrap().get(idx) {
            // Get path: if a directory, use directory path; if it is a File, get parent path
            let path: PathBuf = match entry {
                FsEntry::Directory(dir) => dir.abs_path.clone(),
                FsEntry::File(file) => match file.abs_path.parent() {
                    None => PathBuf::from("."),
                    Some(p) => p.to_path_buf(),
                },
            };
            // Change directory
            match self.browser.tab() {
                FileExplorerTab::FindLocal | FileExplorerTab::Local => {
                    self.local_changedir(path.as_path(), true)
                }
                FileExplorerTab::FindRemote | FileExplorerTab::Remote => {
                    self.remote_changedir(path.as_path(), true)
                }
            }
        }
    }

    pub(crate) fn action_find_transfer(&mut self, idx: usize, name: Option<String>) {
        let entry: Option<FsEntry> = self.found().as_ref().unwrap().get(idx).cloned();
        if let Some(entry) = entry {
            // Download file
            match self.browser.tab() {
                FileExplorerTab::FindLocal | FileExplorerTab::Local => {
                    let wrkdir: PathBuf = self.remote().wrkdir.clone();
                    self.filetransfer_send(&entry.get_realfile(), wrkdir.as_path(), name);
                }
                FileExplorerTab::FindRemote | FileExplorerTab::Remote => {
                    let wrkdir: PathBuf = self.local().wrkdir.clone();
                    self.filetransfer_recv(&entry.get_realfile(), wrkdir.as_path(), name);
                }
            }
        }
    }

    pub(crate) fn action_find_delete(&mut self, idx: usize) {
        let entry: Option<FsEntry> = self.found().as_ref().unwrap().get(idx).cloned();
        if let Some(entry) = entry {
            // Download file
            match self.browser.tab() {
                FileExplorerTab::FindLocal | FileExplorerTab::Local => {
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
                                format!(
                                    "Could not delete file \"{}\": {}",
                                    full_path.display(),
                                    err
                                ),
                            );
                        }
                    }
                }
                FileExplorerTab::FindRemote | FileExplorerTab::Remote => {
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
                                format!(
                                    "Could not delete file \"{}\": {}",
                                    full_path.display(),
                                    err
                                ),
                            );
                        }
                    }
                }
            }
        }
    }
}
