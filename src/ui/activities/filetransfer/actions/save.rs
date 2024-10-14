//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

// locals
use std::path::{Path, PathBuf};

use super::{
    File, FileTransferActivity, LogLevel, Msg, PendingActionMsg, SelectedFile, TransferOpts,
    TransferPayload,
};

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
            SelectedFile::One(entry) => {
                let file_to_check = Self::file_to_check(&entry, opts.save_as.as_ref());
                if self.config().get_prompt_on_file_replace()
                    && self.remote_file_exists(file_to_check.as_path())
                    && !self
                        .should_replace_file(opts.save_as.clone().unwrap_or_else(|| entry.name()))
                {
                    // Do not replace
                    return;
                }
                if let Err(err) = self.filetransfer_send(
                    TransferPayload::Any(entry),
                    wrkdir.as_path(),
                    opts.save_as,
                ) {
                    {
                        self.log_and_alert(
                            LogLevel::Error,
                            format!("Could not upload file: {err}"),
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
                            format!("Could not upload file: {err}"),
                        );
                    }
                }
            }
            SelectedFile::None => {}
        }
    }

    fn remote_recv_file(&mut self, opts: TransferOpts) {
        let wrkdir: PathBuf = self.host_bridge().wrkdir.clone();
        match self.get_remote_selected_entries() {
            SelectedFile::One(entry) => {
                let file_to_check = Self::file_to_check(&entry, opts.save_as.as_ref());
                if self.config().get_prompt_on_file_replace()
                    && self.host_bridge_file_exists(file_to_check.as_path())
                    && !self
                        .should_replace_file(opts.save_as.clone().unwrap_or_else(|| entry.name()))
                {
                    return;
                }
                if let Err(err) = self.filetransfer_recv(
                    TransferPayload::Any(entry),
                    wrkdir.as_path(),
                    opts.save_as,
                ) {
                    {
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
                if self.config().get_prompt_on_file_replace() {
                    // Check which file would be replaced
                    let existing_files: Vec<&File> = entries
                        .iter()
                        .filter(|x| {
                            self.host_bridge_file_exists(
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
                            format!("Could not download file: {err}"),
                        );
                    }
                }
            }
            SelectedFile::None => {}
        }
    }

    /// Set pending transfer into storage
    pub(crate) fn should_replace_file(&mut self, file_name: String) -> bool {
        self.mount_radio_replace(&file_name);
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
    pub(crate) fn should_replace_files(&mut self, files: Vec<&File>) -> bool {
        let file_names: Vec<String> = files.iter().map(|x| x.name()).collect();
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
    pub(crate) fn file_to_check(e: &File, alt: Option<&String>) -> PathBuf {
        match alt {
            Some(s) => PathBuf::from(s),
            None => PathBuf::from(e.name()),
        }
    }

    pub(crate) fn file_to_check_many(e: &File, wrkdir: &Path) -> PathBuf {
        let mut p = wrkdir.to_path_buf();
        p.push(e.name());
        p
    }
}
