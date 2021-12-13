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
    Entry, FileTransferActivity, LogLevel, Msg, PendingActionMsg, SelectedEntry, TransferOpts,
    TransferPayload,
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

    fn local_send_file(&mut self, opts: TransferOpts) {
        let wrkdir: PathBuf = self.remote().wrkdir.clone();
        match self.get_local_selected_entries() {
            SelectedEntry::One(entry) => {
                let file_to_check = Self::file_to_check(&entry, opts.save_as.as_ref());
                if self.config().get_prompt_on_file_replace()
                    && self.remote_file_exists(file_to_check.as_path())
                    && !self.should_replace_file(
                        opts.save_as.as_deref().unwrap_or_else(|| entry.name()),
                    )
                {
                    // Do not replace
                    return;
                }
                if let Err(err) = self.filetransfer_send(
                    TransferPayload::Any(entry.clone()),
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
                if self.config().get_prompt_on_file_replace() {
                    // Check which file would be replaced
                    let existing_files: Vec<&Entry> = entries
                        .iter()
                        .filter(|x| {
                            self.remote_file_exists(
                                Self::file_to_check_many(x, dest_path.as_path()).as_path(),
                            )
                        })
                        .collect();
                    // Check whether to replace files
                    if !existing_files.is_empty() && !self.should_replace_files(existing_files) {
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
                if self.config().get_prompt_on_file_replace()
                    && self.local_file_exists(file_to_check.as_path())
                    && !self.should_replace_file(
                        opts.save_as.as_deref().unwrap_or_else(|| entry.name()),
                    )
                {
                    return;
                }
                if let Err(err) = self.filetransfer_recv(
                    TransferPayload::Any(entry.clone()),
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
                if self.config().get_prompt_on_file_replace() {
                    // Check which file would be replaced
                    let existing_files: Vec<&Entry> = entries
                        .iter()
                        .filter(|x| {
                            self.local_file_exists(
                                Self::file_to_check_many(x, dest_path.as_path()).as_path(),
                            )
                        })
                        .collect();
                    // Check whether to replace files
                    if !existing_files.is_empty() && !self.should_replace_files(existing_files) {
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

    /// Set pending transfer into storage
    pub(crate) fn should_replace_file(&mut self, file_name: &str) -> bool {
        self.mount_radio_replace(file_name);
        // Wait for answer
        trace!("Asking user whether he wants to replace file {}", file_name);
        if self.wait_for_pending_msg(&[
            Msg::PendingAction(PendingActionMsg::CloseReplacePopups),
            Msg::PendingAction(PendingActionMsg::TransferPendingFile),
        ]) == Msg::PendingAction(PendingActionMsg::TransferPendingFile)
        {
            trace!("User wants to replace file");
            self.umount_radio_replace();
            true
        } else {
            trace!("The user doesn't want replace file");
            self.umount_radio_replace();
            false
        }
    }

    /// Set pending transfer for many files into storage and mount radio
    pub(crate) fn should_replace_files(&mut self, files: Vec<&Entry>) -> bool {
        let file_names: Vec<&str> = files.iter().map(|x| x.name()).collect();
        self.mount_radio_replace_many(file_names.as_slice());
        // Wait for answer
        trace!(
            "Asking user whether he wants to replace files {:?}",
            file_names
        );
        if self.wait_for_pending_msg(&[
            Msg::PendingAction(PendingActionMsg::CloseReplacePopups),
            Msg::PendingAction(PendingActionMsg::TransferPendingFile),
        ]) == Msg::PendingAction(PendingActionMsg::TransferPendingFile)
        {
            trace!("User wants to replace files");
            self.umount_radio_replace();
            true
        } else {
            trace!("The user doesn't want replace file");
            self.umount_radio_replace();
            false
        }
    }

    /// Get file to check for path
    pub(crate) fn file_to_check(e: &Entry, alt: Option<&String>) -> PathBuf {
        match alt {
            Some(s) => PathBuf::from(s),
            None => PathBuf::from(e.name()),
        }
    }

    pub(crate) fn file_to_check_many(e: &Entry, wrkdir: &Path) -> PathBuf {
        let mut p = wrkdir.to_path_buf();
        p.push(e.name());
        p
    }
}
