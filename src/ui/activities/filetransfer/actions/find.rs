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
use super::{
    FileTransferActivity, FsEntry, LogLevel, SelectedEntry, TransferOpts, TransferPayload,
};

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

    pub(crate) fn action_find_changedir(&mut self) {
        // Match entry
        if let SelectedEntry::One(entry) = self.get_found_selected_entries() {
            // Get path: if a directory, use directory path; if it is a File, get parent path
            let path: PathBuf = match entry {
                FsEntry::Directory(dir) => dir.abs_path,
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

    pub(crate) fn action_find_transfer(&mut self, opts: TransferOpts) {
        let wrkdir: PathBuf = match self.browser.tab() {
            FileExplorerTab::FindLocal | FileExplorerTab::Local => self.remote().wrkdir.clone(),
            FileExplorerTab::FindRemote | FileExplorerTab::Remote => self.local().wrkdir.clone(),
        };
        match self.get_found_selected_entries() {
            SelectedEntry::One(entry) => match self.browser.tab() {
                FileExplorerTab::FindLocal | FileExplorerTab::Local => {
                    let file_to_check = Self::file_to_check(&entry, opts.save_as.as_ref());
                    if opts.check_replace
                        && self.config().get_prompt_on_file_replace()
                        && self.remote_file_exists(file_to_check.as_path())
                    {
                        // Save pending transfer
                        self.set_pending_transfer(
                            opts.save_as.as_deref().unwrap_or_else(|| entry.get_name()),
                        );
                    } else if let Err(err) = self.filetransfer_send(
                        TransferPayload::Any(entry.get_realfile()),
                        wrkdir.as_path(),
                        opts.save_as,
                    ) {
                        self.log_and_alert(
                            LogLevel::Error,
                            format!("Could not upload file: {}", err),
                        );
                    }
                }
                FileExplorerTab::FindRemote | FileExplorerTab::Remote => {
                    let file_to_check = Self::file_to_check(&entry, opts.save_as.as_ref());
                    if opts.check_replace
                        && self.config().get_prompt_on_file_replace()
                        && self.local_file_exists(file_to_check.as_path())
                    {
                        // Save pending transfer
                        self.set_pending_transfer(
                            opts.save_as.as_deref().unwrap_or_else(|| entry.get_name()),
                        );
                    } else if let Err(err) = self.filetransfer_recv(
                        TransferPayload::Any(entry.get_realfile()),
                        wrkdir.as_path(),
                        opts.save_as,
                    ) {
                        self.log_and_alert(
                            LogLevel::Error,
                            format!("Could not download file: {}", err),
                        );
                    }
                }
            },
            SelectedEntry::Many(entries) => {
                // In case of selection: save multiple files in wrkdir/input
                let mut dest_path: PathBuf = wrkdir;
                if let Some(save_as) = opts.save_as {
                    dest_path.push(save_as);
                }
                // Iter files
                let entries: Vec<FsEntry> = entries.iter().map(|x| x.get_realfile()).collect();
                match self.browser.tab() {
                    FileExplorerTab::FindLocal | FileExplorerTab::Local => {
                        if opts.check_replace && self.config().get_prompt_on_file_replace() {
                            // Check which file would be replaced
                            let existing_files: Vec<&FsEntry> = entries
                                .iter()
                                .filter(|x| {
                                    self.remote_file_exists(
                                        Self::file_to_check_many(x, dest_path.as_path()).as_path(),
                                    )
                                })
                                .collect();
                            // Save pending transfer
                            if !existing_files.is_empty() {
                                self.set_pending_transfer_many(
                                    existing_files,
                                    &dest_path.to_string_lossy().to_owned(),
                                );
                                return;
                            }
                        }
                        if let Err(err) = self.filetransfer_send(
                            TransferPayload::Many(entries),
                            dest_path.as_path(),
                            None,
                        ) {
                            {
                                self.log_and_alert(
                                    LogLevel::Error,
                                    format!("Could not upload file: {}", err),
                                );
                            }
                        }
                    }
                    FileExplorerTab::FindRemote | FileExplorerTab::Remote => {
                        if opts.check_replace && self.config().get_prompt_on_file_replace() {
                            // Check which file would be replaced
                            let existing_files: Vec<&FsEntry> = entries
                                .iter()
                                .filter(|x| {
                                    self.local_file_exists(
                                        Self::file_to_check_many(x, dest_path.as_path()).as_path(),
                                    )
                                })
                                .collect();
                            // Save pending transfer
                            // Save pending transfer
                            if !existing_files.is_empty() {
                                self.set_pending_transfer_many(
                                    existing_files,
                                    &dest_path.to_string_lossy().to_owned(),
                                );
                                return;
                            }
                        }
                        if let Err(err) = self.filetransfer_recv(
                            TransferPayload::Many(entries),
                            dest_path.as_path(),
                            None,
                        ) {
                            self.log_and_alert(
                                LogLevel::Error,
                                format!("Could not download file: {}", err),
                            );
                        }
                    }
                }
            }
            SelectedEntry::None => {}
        }
    }

    pub(crate) fn action_find_delete(&mut self) {
        match self.get_found_selected_entries() {
            SelectedEntry::One(entry) => {
                // Delete file
                self.remove_found_file(&entry);
            }
            SelectedEntry::Many(entries) => {
                // Iter files
                for entry in entries.iter() {
                    // Delete file
                    self.remove_found_file(entry);
                }
            }
            SelectedEntry::None => {}
        }
    }

    fn remove_found_file(&mut self, entry: &FsEntry) {
        match self.browser.tab() {
            FileExplorerTab::FindLocal | FileExplorerTab::Local => {
                self.local_remove_file(entry);
            }
            FileExplorerTab::FindRemote | FileExplorerTab::Remote => {
                self.remote_remove_file(entry);
            }
        }
    }

    pub(crate) fn action_find_open(&mut self) {
        match self.get_found_selected_entries() {
            SelectedEntry::One(entry) => {
                // Open file
                self.open_found_file(&entry, None);
            }
            SelectedEntry::Many(entries) => {
                // Iter files
                for entry in entries.iter() {
                    // Open file
                    self.open_found_file(entry, None);
                }
            }
            SelectedEntry::None => {}
        }
    }

    pub(crate) fn action_find_open_with(&mut self, with: &str) {
        match self.get_found_selected_entries() {
            SelectedEntry::One(entry) => {
                // Open file
                self.open_found_file(&entry, Some(with));
            }
            SelectedEntry::Many(entries) => {
                // Iter files
                for entry in entries.iter() {
                    // Open file
                    self.open_found_file(entry, Some(with));
                }
            }
            SelectedEntry::None => {}
        }
    }

    fn open_found_file(&mut self, entry: &FsEntry, with: Option<&str>) {
        match self.browser.tab() {
            FileExplorerTab::FindLocal | FileExplorerTab::Local => {
                self.action_open_local_file(entry, with);
            }
            FileExplorerTab::FindRemote | FileExplorerTab::Remote => {
                self.action_open_remote_file(entry, with);
            }
        }
    }
}
