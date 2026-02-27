//! ## FileTransferActivity
//!
//! `filetransfer_activity` is the module which implements the Filetransfer activity, which is the main activity afterall

// locals
use std::path::{Path, PathBuf};

use super::{
    File, FileTransferActivity, LogLevel, Msg, PendingActionMsg, SelectedFile, TransferOpts,
    TransferPayload,
};

enum GetFileToReplaceResult {
    Replace(Vec<(File, PathBuf)>),
    Cancel,
}

/// Result of getting files to transfer with overwrites.
///
/// - FilesToTransfer: files to transfer.
/// - Cancel: user cancelled the operation.
pub(crate) enum TransferFilesWithOverwritesResult {
    FilesToTransfer(Vec<(File, PathBuf)>),
    Cancel,
}

/// Decides whether to check file existence on host bridge or remote side.
pub(crate) enum CheckFileExists {
    HostBridge,
    Remote,
}

/// Options for all files replacement.
///
/// - ReplaceAll: user wants to replace all files.
/// - SkipAll: user wants to skip all files.
/// - Unset: no option set yet.
enum AllOpts {
    ReplaceAll,
    SkipAll,
    Unset,
}

impl FileTransferActivity {
    pub(crate) fn action_saveas(&mut self, input: String) {
        if self.is_local_tab() {
            self.local_send_file(TransferOpts::default().save_as(Some(input)));
        } else {
            self.remote_recv_file(TransferOpts::default().save_as(Some(input)));
        }
    }

    pub(crate) fn action_transfer_file(&mut self) {
        if self.is_local_tab() {
            self.local_send_file(TransferOpts::default());
        } else {
            self.remote_recv_file(TransferOpts::default());
        }
    }

    fn local_send_file(&mut self, opts: TransferOpts) {
        let wrkdir: PathBuf = self.remote().wrkdir.clone();
        match self.get_local_selected_entries() {
            SelectedFile::One(entry) => {
                let file_to_check = Self::file_to_check(&entry, opts.save_as.as_ref());
                if self.config().get_prompt_on_file_replace()
                    && self.file_exists(file_to_check.as_path(), false)
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
                let TransferFilesWithOverwritesResult::FilesToTransfer(entries) =
                    self.get_files_to_transfer_with_overwrites(entries, CheckFileExists::Remote)
                else {
                    debug!("User cancelled file transfer due to overwrites");
                    return;
                };
                if let Err(err) = self.filetransfer_send(
                    TransferPayload::TransferQueue(entries),
                    dest_path.as_path(),
                    None,
                ) {
                    {
                        self.log_and_alert(
                            LogLevel::Error,
                            format!("Could not upload file: {err}"),
                        );
                    }
                } else {
                    // clear selection
                    self.host_bridge_mut().clear_queue();
                    self.reload_host_bridge_filelist();
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
                    && self.file_exists(file_to_check.as_path(), true)
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
                let TransferFilesWithOverwritesResult::FilesToTransfer(entries) = self
                    .get_files_to_transfer_with_overwrites(entries, CheckFileExists::HostBridge)
                else {
                    debug!("User cancelled file transfer due to overwrites");
                    return;
                };

                if let Err(err) = self.filetransfer_recv(
                    TransferPayload::TransferQueue(entries),
                    dest_path.as_path(),
                    None,
                ) {
                    {
                        self.log_and_alert(
                            LogLevel::Error,
                            format!("Could not download file: {err}"),
                        );
                    }
                } else {
                    // clear selection
                    self.remote_mut().clear_queue();
                    // reload remote
                    self.reload_remote_filelist();
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
        if matches!(
            self.wait_for_pending_msg(&[
                Msg::PendingAction(PendingActionMsg::ReplaceCancel),
                Msg::PendingAction(PendingActionMsg::ReplaceOverwrite),
                Msg::PendingAction(PendingActionMsg::ReplaceSkip),
                Msg::PendingAction(PendingActionMsg::ReplaceSkipAll),
                Msg::PendingAction(PendingActionMsg::ReplaceOverwriteAll),
            ]),
            Msg::PendingAction(PendingActionMsg::ReplaceOverwrite)
                | Msg::PendingAction(PendingActionMsg::ReplaceOverwriteAll)
        ) {
            trace!("User wants to replace file");
            self.umount_radio_replace();
            true
        } else {
            trace!("The user doesn't want replace file");
            self.umount_radio_replace();
            false
        }
    }

    /// Get files to replace
    fn get_files_to_replace(&mut self, files: Vec<(File, PathBuf)>) -> GetFileToReplaceResult {
        // keep only files the user want to replace
        let mut files_to_replace = vec![];
        let mut all_opts = AllOpts::Unset;
        for (file, p) in files {
            // Check for all opts
            match all_opts {
                AllOpts::ReplaceAll => {
                    trace!(
                        "User wants to replace all files, including file {}",
                        file.name()
                    );
                    files_to_replace.push((file, p));
                    continue;
                }
                AllOpts::SkipAll => {
                    trace!(
                        "User wants to skip all files, including file {}",
                        file.name()
                    );
                    continue;
                }
                AllOpts::Unset => {}
            }

            let file_name = file.name();
            self.mount_radio_replace(&file_name);

            // Wait for answer
            match self.wait_for_pending_msg(&[
                Msg::PendingAction(PendingActionMsg::ReplaceCancel),
                Msg::PendingAction(PendingActionMsg::ReplaceOverwrite),
                Msg::PendingAction(PendingActionMsg::ReplaceSkip),
                Msg::PendingAction(PendingActionMsg::ReplaceSkipAll),
                Msg::PendingAction(PendingActionMsg::ReplaceOverwriteAll),
            ]) {
                Msg::PendingAction(PendingActionMsg::ReplaceCancel) => {
                    trace!("The user cancelled the replace operation");
                    self.umount_radio_replace();
                    return GetFileToReplaceResult::Cancel;
                }
                Msg::PendingAction(PendingActionMsg::ReplaceOverwrite) => {
                    trace!("User wants to replace file {}", file_name);
                    files_to_replace.push((file, p));
                }
                Msg::PendingAction(PendingActionMsg::ReplaceOverwriteAll) => {
                    trace!(
                        "User wants to replace all files from now on, including file {}",
                        file_name
                    );
                    files_to_replace.push((file, p));
                    all_opts = AllOpts::ReplaceAll;
                }
                Msg::PendingAction(PendingActionMsg::ReplaceSkip) => {
                    trace!("The user skipped file {}", file_name);
                }
                Msg::PendingAction(PendingActionMsg::ReplaceSkipAll) => {
                    trace!(
                        "The user skipped all files from now on, including file {}",
                        file_name
                    );
                    all_opts = AllOpts::SkipAll;
                }
                _ => {}
            }
            self.umount_radio_replace();
        }

        GetFileToReplaceResult::Replace(files_to_replace)
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

    /// Get the files to transfer with overwrites.
    ///
    /// Existing and unexisting files are splitted, and only existing files are prompted for replacement.
    pub(crate) fn get_files_to_transfer_with_overwrites(
        &mut self,
        files: Vec<(File, PathBuf)>,
        file_exists: CheckFileExists,
    ) -> TransferFilesWithOverwritesResult {
        if !self.config().get_prompt_on_file_replace() {
            return TransferFilesWithOverwritesResult::FilesToTransfer(files);
        }

        // unzip between existing and non-existing files
        let (existing_files, new_files): (Vec<_>, Vec<_>) =
            files.into_iter().partition(|(x, dest_path)| {
                let p = Self::file_to_check_many(x, dest_path);
                match file_exists {
                    CheckFileExists::Remote => self.file_exists(p.as_path(), false),
                    CheckFileExists::HostBridge => self.file_exists(p.as_path(), true),
                }
            });

        // filter only files to replace
        let existing_files = match self.get_files_to_replace(existing_files) {
            GetFileToReplaceResult::Replace(files) => files,
            GetFileToReplaceResult::Cancel => {
                return TransferFilesWithOverwritesResult::Cancel;
            }
        };

        // merge back
        TransferFilesWithOverwritesResult::FilesToTransfer(
            existing_files.into_iter().chain(new_files).collect(),
        )
    }
}
