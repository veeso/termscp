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
use super::{
    super::STORAGE_PENDING_TRANSFER, FileExplorerTab, FileTransferActivity, FsEntry, LogLevel,
    SelectedEntry, TransferOpts, TransferPayload,
};
use std::path::{Path, PathBuf};

impl FileTransferActivity {
    pub(crate) fn action_local_saveas(&mut self, input: String) {
        self.local_send_file(TransferOpts::default().save_as(Some(input)));
    }

    pub(crate) fn action_remote_saveas(&mut self, input: String) {
        self.remote_recv_file(TransferOpts::default().save_as(Some(input)));
    }

    pub(crate) fn action_local_send(&mut self) {
        self.local_send_file(TransferOpts::default());
    }

    pub(crate) fn action_remote_recv(&mut self) {
        self.remote_recv_file(TransferOpts::default());
    }

    /// ### action_finalize_pending_transfer
    ///
    /// Finalize "pending" transfer.
    /// The pending transfer is created after a transfer which required a user action to be completed first.
    /// The name of the file to transfer, is contained in the storage at `STORAGE_PENDING_TRANSFER`.
    /// NOTE: Panics if `STORAGE_PENDING_TRANSFER` is undefined
    pub(crate) fn action_finalize_pending_transfer(&mut self) {
        // Retrieve pending transfer
        let file_name = self
            .context_mut()
            .store_mut()
            .take_string(STORAGE_PENDING_TRANSFER);
        // Send file
        match self.browser.tab() {
            FileExplorerTab::Local => self.local_send_file(
                TransferOpts::default()
                    .save_as(file_name)
                    .check_replace(false),
            ),
            FileExplorerTab::Remote => self.remote_recv_file(
                TransferOpts::default()
                    .save_as(file_name)
                    .check_replace(false),
            ),
            FileExplorerTab::FindLocal | FileExplorerTab::FindRemote => self.action_find_transfer(
                TransferOpts::default()
                    .save_as(file_name)
                    .check_replace(false),
            ),
        }
        // Reload browsers
        match self.browser.tab() {
            FileExplorerTab::Local | FileExplorerTab::FindLocal => {
                self.update_remote_filelist();
            }
            FileExplorerTab::Remote | FileExplorerTab::FindRemote => {
                self.update_local_filelist();
            }
        }
    }

    fn local_send_file(&mut self, opts: TransferOpts) {
        let wrkdir: PathBuf = self.remote().wrkdir.clone();
        match self.get_local_selected_entries() {
            SelectedEntry::One(entry) => {
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
                    {
                        self.log_and_alert(
                            LogLevel::Error,
                            format!("Could not upload file: {}", err),
                        );
                    }
                }
            }
            SelectedEntry::Many(entries) => {
                // In case of selection: save multiple files in wrkdir/input
                let mut dest_path: PathBuf = wrkdir;
                if let Some(save_as) = opts.save_as {
                    dest_path.push(save_as);
                }
                // Iter files
                let entries: Vec<FsEntry> = entries.iter().map(|x| x.get_realfile()).collect();
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
            SelectedEntry::None => {}
        }
    }

    fn remote_recv_file(&mut self, opts: TransferOpts) {
        let wrkdir: PathBuf = self.local().wrkdir.clone();
        match self.get_remote_selected_entries() {
            SelectedEntry::One(entry) => {
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
                    {
                        self.log_and_alert(
                            LogLevel::Error,
                            format!("Could not download file: {}", err),
                        );
                    }
                }
            }
            SelectedEntry::Many(entries) => {
                // In case of selection: save multiple files in wrkdir/input
                let mut dest_path: PathBuf = wrkdir;
                if let Some(save_as) = opts.save_as {
                    dest_path.push(save_as);
                }
                // Iter files
                let entries: Vec<FsEntry> = entries.iter().map(|x| x.get_realfile()).collect();
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
                    {
                        self.log_and_alert(
                            LogLevel::Error,
                            format!("Could not download file: {}", err),
                        );
                    }
                }
            }
            SelectedEntry::None => {}
        }
    }

    /// ### set_pending_transfer
    ///
    /// Set pending transfer into storage
    pub(crate) fn set_pending_transfer(&mut self, file_name: &str) {
        self.mount_radio_replace(file_name);
        // Put pending transfer in store
        self.context_mut()
            .store_mut()
            .set_string(STORAGE_PENDING_TRANSFER, file_name.to_string());
    }

    /// ### set_pending_transfer_many
    ///
    /// Set pending transfer for many files into storage and mount radio
    pub(crate) fn set_pending_transfer_many(&mut self, files: Vec<&FsEntry>, dest_path: &str) {
        let file_names: Vec<&str> = files.iter().map(|x| x.get_name()).collect();
        self.mount_radio_replace_many(file_names.as_slice());
        self.context_mut()
            .store_mut()
            .set_string(STORAGE_PENDING_TRANSFER, dest_path.to_string());
    }

    /// ### file_to_check
    ///
    /// Get file to check for path
    pub(crate) fn file_to_check(e: &FsEntry, alt: Option<&String>) -> PathBuf {
        match alt {
            Some(s) => PathBuf::from(s),
            None => PathBuf::from(e.get_name()),
        }
    }

    pub(crate) fn file_to_check_many(e: &FsEntry, wrkdir: &Path) -> PathBuf {
        let mut p = wrkdir.to_path_buf();
        p.push(e.get_name());
        p
    }
}
