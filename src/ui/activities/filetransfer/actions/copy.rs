//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

// locals
use std::path::{Path, PathBuf};

use remotefs::{File, RemoteErrorType};

use super::{FileTransferActivity, LogLevel, SelectedFile, TransferPayload};

impl FileTransferActivity {
    /// Copy file on local
    pub(crate) fn action_local_copy(&mut self, input: String) {
        match self.get_local_selected_entries() {
            SelectedFile::One(entry) => {
                let dest_path: PathBuf = PathBuf::from(input);
                self.local_copy_file(&entry, dest_path.as_path());
            }
            SelectedFile::Many(entries) => {
                // Try to copy each file to Input/{FILE_NAME}
                let base_path: PathBuf = PathBuf::from(input);
                // Iter files
                for entry in entries.iter() {
                    let mut dest_path: PathBuf = base_path.clone();
                    dest_path.push(entry.name());
                    self.local_copy_file(entry, dest_path.as_path());
                }
            }
            SelectedFile::None => {}
        }
    }

    /// Copy file on remote
    pub(crate) fn action_remote_copy(&mut self, input: String) {
        match self.get_remote_selected_entries() {
            SelectedFile::One(entry) => {
                let dest_path: PathBuf = PathBuf::from(input);
                self.remote_copy_file(entry, dest_path.as_path());
            }
            SelectedFile::Many(entries) => {
                // Try to copy each file to Input/{FILE_NAME}
                let base_path: PathBuf = PathBuf::from(input);
                // Iter files
                for entry in entries.into_iter() {
                    let mut dest_path: PathBuf = base_path.clone();
                    dest_path.push(entry.name());
                    self.remote_copy_file(entry, dest_path.as_path());
                }
            }
            SelectedFile::None => {}
        }
    }

    fn local_copy_file(&mut self, entry: &File, dest: &Path) {
        match self.host.copy(entry, dest) {
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
            Err(err) => self.log_and_alert(
                LogLevel::Error,
                format!(
                    "Could not copy \"{}\" to \"{}\": {}",
                    entry.path().display(),
                    dest.display(),
                    err
                ),
            ),
        }
    }

    fn remote_copy_file(&mut self, entry: File, dest: &Path) {
        match self.client.as_mut().copy(entry.path(), dest) {
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
            Err(err) => match err.kind {
                RemoteErrorType::UnsupportedFeature => {
                    // If copy is not supported, perform the tricky copy
                    let _ = self.tricky_copy(entry, dest);
                }
                _ => self.log_and_alert(
                    LogLevel::Error,
                    format!(
                        "Could not copy \"{}\" to \"{}\": {}",
                        entry.path().display(),
                        dest.display(),
                        err
                    ),
                ),
            },
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
            let tempdir_entry = match self.host.stat(tempdir_path.as_path()) {
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
            let tmpfile_entry = match self.host.stat(tmpfile.path()) {
                Ok(e) if e.is_file() => e,
                Ok(_) => panic!("{} is not a file", tmpfile.path().display()),
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
