//! ## FileTransferActivity
//!
//! `filetransfer_activity` is the module which implements the Filetransfer activity, which is the main activity afterall

// locals
use std::path::{Path, PathBuf};

use remotefs::File;

use super::{FileTransferActivity, LogLevel, SelectedFile, TransferPayload};

impl FileTransferActivity {
    /// Copy the currently selected file(s) via the active tab's pane.
    pub(crate) fn action_copy(&mut self, input: String) {
        match self.get_selected_entries() {
            SelectedFile::One(entry) => {
                let dest_path = PathBuf::from(input);
                self.copy_file(entry, dest_path.as_path());
            }
            SelectedFile::Many(entries) => {
                for (entry, mut dest_path) in entries.into_iter() {
                    dest_path.push(entry.name());
                    self.copy_file(entry, dest_path.as_path());
                }

                // clear selection and reload
                self.browser.explorer_mut().clear_queue();
                self.reload_browser_file_list();
            }
            SelectedFile::None => {}
        }
    }

    fn copy_file(&mut self, entry: File, dest: &Path) {
        match self.browser.fs_pane_mut().fs.copy(&entry, dest) {
            Ok(_) => {
                self.log(
                    LogLevel::Info,
                    format!(
                        "Copied \"{}\" to \"{}\"",
                        entry.path().display(),
                        dest.display()
                    ),
                );
            }
            Err(err) => {
                // On remote tabs, fall back to tricky_copy (download + re-upload)
                // when the protocol doesn't support server-side copy.
                if !self.is_local_tab() {
                    let _ = self.tricky_copy(entry, dest);
                } else {
                    self.log_and_alert(
                        LogLevel::Error,
                        format!(
                            "Could not copy \"{}\" to \"{}\": {}",
                            entry.path().display(),
                            dest.display(),
                            err
                        ),
                    );
                }
            }
        }
    }

    /// Tricky copy will be used whenever copy command is not available on remote host
    pub(super) fn tricky_copy(&mut self, entry: File, dest: &Path) -> Result<(), String> {
        // NOTE: VERY IMPORTANT; wait block must be umounted or something really bad will happen
        self.umount_wait();
        // match entry
        if entry.is_dir() {
            let tempdir: tempfile::TempDir = match tempfile::TempDir::new() {
                Ok(d) => d,
                Err(err) => {
                    self.log_and_alert(
                        LogLevel::Error,
                        format!("Copy failed: could not create temporary directory: {err}"),
                    );
                    return Err(err.to_string());
                }
            };
            // Get path of dest
            let mut tempdir_path: PathBuf = tempdir.path().to_path_buf();
            tempdir_path.push(entry.name());
            // Download file
            if let Err(err) =
                self.filetransfer_recv(TransferPayload::Any(entry), tempdir.path(), None)
            {
                self.log_and_alert(
                    LogLevel::Error,
                    format!("Copy failed: failed to download file: {err}"),
                );
                return Err(err);
            }
            // Stat dir
            let tempdir_entry = match self
                .browser
                .local_pane_mut()
                .fs
                .stat(tempdir_path.as_path())
            {
                Ok(e) => e,
                Err(err) => {
                    self.log_and_alert(
                        LogLevel::Error,
                        format!(
                            "Copy failed: could not stat \"{}\": {}",
                            tempdir.path().display(),
                            err
                        ),
                    );
                    return Err(err.to_string());
                }
            };
            // Upload to destination
            let wrkdir: PathBuf = self.remote().wrkdir.clone();
            if let Err(err) = self.filetransfer_send(
                TransferPayload::Any(tempdir_entry),
                wrkdir.as_path(),
                Some(String::from(dest.to_string_lossy())),
            ) {
                self.log_and_alert(
                    LogLevel::Error,
                    format!("Copy failed: failed to send file: {err}"),
                );
                return Err(err);
            }
            Ok(())
        } else {
            // Create tempfile
            let tmpfile: tempfile::NamedTempFile = match tempfile::NamedTempFile::new() {
                Ok(f) => f,
                Err(err) => {
                    self.log_and_alert(
                        LogLevel::Error,
                        format!("Copy failed: could not create temporary file: {err}"),
                    );
                    return Err(String::from("Could not create temporary file"));
                }
            };
            // Download file
            let name = entry.name();
            let entry_path = entry.path().to_path_buf();
            if let Err(err) =
                self.filetransfer_recv(TransferPayload::File(entry), tmpfile.path(), Some(name))
            {
                self.log_and_alert(
                    LogLevel::Error,
                    format!("Copy failed: could not download to temporary file: {err}"),
                );
                return Err(err);
            }
            // Get local fs entry
            let tmpfile_entry = match self.browser.local_pane_mut().fs.stat(tmpfile.path()) {
                Ok(e) if e.is_file() => e,
                Ok(_) => {
                    let msg = format!(
                        "Copy failed: \"{}\" is not a file",
                        tmpfile.path().display()
                    );
                    error!("{msg}");
                    self.log_and_alert(LogLevel::Error, msg.clone());
                    return Err(msg);
                }
                Err(err) => {
                    self.log_and_alert(
                        LogLevel::Error,
                        format!(
                            "Copy failed: could not stat \"{}\": {}",
                            tmpfile.path().display(),
                            err
                        ),
                    );
                    return Err(err.to_string());
                }
            };
            // Upload file to destination
            let wrkdir = self.remote().wrkdir.clone();
            if let Err(err) = self.filetransfer_send(
                TransferPayload::File(tmpfile_entry),
                wrkdir.as_path(),
                Some(String::from(dest.to_string_lossy())),
            ) {
                self.log_and_alert(
                    LogLevel::Error,
                    format!(
                        "Copy failed: could not write file {}: {}",
                        entry_path.display(),
                        err
                    ),
                );
                return Err(err);
            }
            Ok(())
        }
    }
}
