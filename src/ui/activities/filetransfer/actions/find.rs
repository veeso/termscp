//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

// locals
use super::super::browser::FileExplorerTab;
use super::{File, FileTransferActivity, LogLevel, SelectedFile, TransferOpts, TransferPayload};

use std::path::PathBuf;

impl FileTransferActivity {
    pub(crate) fn action_local_find(&mut self, input: String) -> Result<Vec<File>, String> {
        match self.host.find(input.as_str()) {
            Ok(entries) => Ok(entries),
            Err(err) => Err(format!("Could not search for files: {}", err)),
        }
    }

    pub(crate) fn action_remote_find(&mut self, input: String) -> Result<Vec<File>, String> {
        match self.client.as_mut().find(input.as_str()) {
            Ok(entries) => Ok(entries),
            Err(err) => Err(format!("Could not search for files: {}", err)),
        }
    }

    pub(crate) fn action_find_changedir(&mut self) {
        // Match entry
        if let SelectedFile::One(entry) = self.get_found_selected_entries() {
            // Get path: if a directory, use directory path; if it is a File, get parent path
            let path = if entry.is_dir() {
                entry.path().to_path_buf()
            } else {
                match entry.path().parent() {
                    None => PathBuf::from("."),
                    Some(p) => p.to_path_buf(),
                }
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
            SelectedFile::One(entry) => match self.browser.tab() {
                FileExplorerTab::FindLocal | FileExplorerTab::Local => {
                    let file_to_check = Self::file_to_check(&entry, opts.save_as.as_ref());
                    if self.config().get_prompt_on_file_replace()
                        && self.remote_file_exists(file_to_check.as_path())
                        && !self.should_replace_file(
                            opts.save_as.clone().unwrap_or_else(|| entry.name()),
                        )
                    {
                        // Do not replace
                        return;
                    }
                    if let Err(err) = self.filetransfer_send(
                        TransferPayload::Any(entry),
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
                    if self.config().get_prompt_on_file_replace()
                        && self.local_file_exists(file_to_check.as_path())
                        && !self.should_replace_file(
                            opts.save_as.clone().unwrap_or_else(|| entry.name()),
                        )
                    {
                        // Do not replace
                        return;
                    }
                    if let Err(err) = self.filetransfer_recv(
                        TransferPayload::Any(entry),
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
            SelectedFile::Many(entries) => {
                // In case of selection: save multiple files in wrkdir/input
                let mut dest_path: PathBuf = wrkdir;
                if let Some(save_as) = opts.save_as {
                    dest_path.push(save_as);
                }
                // Iter files
                match self.browser.tab() {
                    FileExplorerTab::FindLocal | FileExplorerTab::Local => {
                        if self.config().get_prompt_on_file_replace() {
                            // Check which file would be replaced
                            let existing_files: Vec<&File> = entries
                                .iter()
                                .filter(|x| {
                                    self.remote_file_exists(
                                        Self::file_to_check_many(x, dest_path.as_path()).as_path(),
                                    )
                                })
                                .collect();
                            // Check whether to replace files
                            if !existing_files.is_empty()
                                && !self.should_replace_files(existing_files)
                            {
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
                        if self.config().get_prompt_on_file_replace() {
                            // Check which file would be replaced
                            let existing_files: Vec<&File> = entries
                                .iter()
                                .filter(|x| {
                                    self.local_file_exists(
                                        Self::file_to_check_many(x, dest_path.as_path()).as_path(),
                                    )
                                })
                                .collect();
                            // Check whether to replace files
                            if !existing_files.is_empty()
                                && !self.should_replace_files(existing_files)
                            {
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
            SelectedFile::None => {}
        }
    }

    pub(crate) fn action_find_delete(&mut self) {
        match self.get_found_selected_entries() {
            SelectedFile::One(entry) => {
                // Delete file
                self.remove_found_file(&entry);
            }
            SelectedFile::Many(entries) => {
                // Iter files
                for entry in entries.iter() {
                    // Delete file
                    self.remove_found_file(entry);
                }
            }
            SelectedFile::None => {}
        }
    }

    fn remove_found_file(&mut self, entry: &File) {
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
            SelectedFile::One(entry) => {
                // Open file
                self.open_found_file(&entry, None);
            }
            SelectedFile::Many(entries) => {
                // Iter files
                for entry in entries.iter() {
                    // Open file
                    self.open_found_file(entry, None);
                }
            }
            SelectedFile::None => {}
        }
    }

    pub(crate) fn action_find_open_with(&mut self, with: &str) {
        match self.get_found_selected_entries() {
            SelectedFile::One(entry) => {
                // Open file
                self.open_found_file(&entry, Some(with));
            }
            SelectedFile::Many(entries) => {
                // Iter files
                for entry in entries.iter() {
                    // Open file
                    self.open_found_file(entry, Some(with));
                }
            }
            SelectedFile::None => {}
        }
    }

    fn open_found_file(&mut self, entry: &File, with: Option<&str>) {
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
