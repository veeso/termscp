//! ## FileTransferActivity
//!
//! `filetransfer_activity` is the module which implements the Filetransfer activity, which is the main activity afterall

// locals
use std::path::PathBuf;

use super::{File, FileTransferActivity, LogLevel, SelectedFile, TransferOpts, TransferPayload};

impl FileTransferActivity {
    pub(crate) fn action_find_changedir(&mut self) {
        // Match entry
        if let Some(entry) = self.get_found_selected_file() {
            debug!("Changedir to: {}", entry.name());
            // Get path: if a directory, use directory path; if it is a File, get parent path
            let path = if entry.is_dir() {
                entry.path().to_path_buf()
            } else {
                match entry.path().parent() {
                    None => PathBuf::from("."),
                    Some(p) => p.to_path_buf(),
                }
            };
            // Change directory on the active tab's pane
            self.pane_changedir(path.as_path(), true);
        }
    }

    pub(crate) fn action_find_transfer(&mut self, opts: TransferOpts) {
        let wrkdir: PathBuf = self.browser.opposite_pane().explorer.wrkdir.clone();
        match self.get_found_selected_entries() {
            SelectedFile::One(entry) => {
                if self.is_local_tab() {
                    let file_to_check = Self::file_to_check(&entry, opts.save_as.as_ref());
                    if self.config().get_prompt_on_file_replace()
                        && self.file_exists(file_to_check.as_path(), false)
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
                            format!("Could not upload file: {err}"),
                        );
                    }
                } else {
                    let file_to_check = Self::file_to_check(&entry, opts.save_as.as_ref());
                    if self.config().get_prompt_on_file_replace()
                        && self.file_exists(file_to_check.as_path(), true)
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
                            format!("Could not download file: {err}"),
                        );
                    }
                }
            }
            SelectedFile::Many(entries) => {
                // In case of selection: save multiple files in wrkdir/input
                let mut dest_path: PathBuf = wrkdir;
                if let Some(save_as) = opts.save_as {
                    dest_path.push(save_as);
                }
                // Iter files
                if self.is_local_tab() {
                    let super::save::TransferFilesWithOverwritesResult::FilesToTransfer(entries) =
                        self.get_files_to_transfer_with_overwrites(
                            entries,
                            super::save::CheckFileExists::Remote,
                        )
                    else {
                        debug!("User cancelled file transfer due to overwrites");
                        return;
                    };
                    if let Err(err) = self.filetransfer_send(
                        TransferPayload::TransferQueue(entries),
                        dest_path.as_path(),
                        None,
                    ) {
                        self.log_and_alert(
                            LogLevel::Error,
                            format!("Could not upload file: {err}"),
                        );
                    }
                } else {
                    let super::save::TransferFilesWithOverwritesResult::FilesToTransfer(entries) =
                        self.get_files_to_transfer_with_overwrites(
                            entries,
                            super::save::CheckFileExists::HostBridge,
                        )
                    else {
                        debug!("User cancelled file transfer due to overwrites");
                        return;
                    };
                    if let Err(err) = self.filetransfer_recv(
                        TransferPayload::TransferQueue(entries),
                        dest_path.as_path(),
                        None,
                    ) {
                        self.log_and_alert(
                            LogLevel::Error,
                            format!("Could not download file: {err}"),
                        );
                    }

                    // clear selection
                    if let Some(f) = self.found_mut() {
                        f.clear_queue();
                        self.update_find_list();
                    }
                }
            }
            SelectedFile::None => {}
        }
    }

    pub(crate) fn action_find_delete(&mut self) {
        match self.get_found_selected_entries() {
            SelectedFile::One(entry) => {
                self.remove_file(&entry);
            }
            SelectedFile::Many(entries) => {
                for (entry, _) in entries.iter() {
                    self.remove_file(entry);
                }

                // clear selection
                if let Some(f) = self.found_mut() {
                    f.clear_queue();
                    self.update_find_list();
                }
            }
            SelectedFile::None => {}
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
                for (entry, _) in entries.iter() {
                    // Open file
                    self.open_found_file(entry, None);
                }
                // clear selection
                if let Some(f) = self.found_mut() {
                    f.clear_queue();
                    self.update_find_list();
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
                for (entry, _) in entries.iter() {
                    // Open file
                    self.open_found_file(entry, Some(with));
                }
                // clear selection
                if let Some(f) = self.found_mut() {
                    f.clear_queue();
                    self.update_find_list();
                }
            }
            SelectedFile::None => {}
        }
    }

    fn open_found_file(&mut self, entry: &File, with: Option<&str>) {
        self.open_file(entry, with);
    }
}
