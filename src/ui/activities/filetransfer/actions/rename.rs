//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

// locals
use std::path::{Path, PathBuf};

use remotefs::RemoteErrorType;

use super::{File, FileTransferActivity, LogLevel, SelectedFile};

impl FileTransferActivity {
    pub(crate) fn action_local_rename(&mut self, input: String) {
        match self.get_local_selected_entries() {
            SelectedFile::One(entry) => {
                let dest_path: PathBuf = PathBuf::from(input);
                self.local_rename_file(&entry, dest_path.as_path());
            }
            SelectedFile::Many(entries) => {
                // Try to copy each file to Input/{FILE_NAME}
                // Iter files
                for (entry, mut dest_path) in entries.into_iter() {
                    dest_path.push(entry.name());
                    self.local_rename_file(&entry, dest_path.as_path());
                }

                // clear selection
                self.host_bridge_mut().clear_queue();
                self.reload_host_bridge_filelist();
            }
            SelectedFile::None => {}
        }
    }

    pub(crate) fn action_remote_rename(&mut self, input: String) {
        match self.get_remote_selected_entries() {
            SelectedFile::One(entry) => {
                let dest_path: PathBuf = PathBuf::from(input);
                self.remote_rename_file(&entry, dest_path.as_path());
            }
            SelectedFile::Many(entries) => {
                // Try to copy each file to Input/{FILE_NAME}
                // Iter files
                for (entry, mut dest_path) in entries.into_iter() {
                    dest_path.push(entry.name());
                    self.remote_rename_file(&entry, dest_path.as_path());
                }

                // clear selection
                self.remote_mut().clear_queue();
                // reload remote
                self.reload_remote_filelist();
            }
            SelectedFile::None => {}
        }
    }

    fn local_rename_file(&mut self, entry: &File, dest: &Path) {
        match self.host_bridge.rename(entry, dest) {
            Ok(_) => {
                self.log(
                    LogLevel::Info,
                    format!(
                        "Moved \"{}\" to \"{}\"",
                        entry.path().display(),
                        dest.display()
                    ),
                );
            }
            Err(err) => self.log_and_alert(
                LogLevel::Error,
                format!(
                    "Could not move \"{}\" to \"{}\": {}",
                    entry.path().display(),
                    dest.display(),
                    err
                ),
            ),
        }
    }

    pub(crate) fn remote_rename_file(&mut self, entry: &File, dest: &Path) {
        match self.client.as_mut().mov(entry.path(), dest) {
            Ok(_) => {
                self.log(
                    LogLevel::Info,
                    format!(
                        "Moved \"{}\" to \"{}\"",
                        entry.path().display(),
                        dest.display()
                    ),
                );
            }
            Err(err) if err.kind == RemoteErrorType::UnsupportedFeature => {
                self.tricky_move(entry, dest);
            }
            Err(err) => self.log_and_alert(
                LogLevel::Error,
                format!(
                    "Could not move \"{}\" to \"{}\": {}",
                    entry.path().display(),
                    dest.display(),
                    err
                ),
            ),
        }
    }

    /// Tricky move will be used whenever copy command is not available on remote host.
    /// It basically uses the tricky_copy function, then it just deletes the previous entry (`entry`)
    fn tricky_move(&mut self, entry: &File, dest: &Path) {
        debug!(
            "Using tricky-move to move entry {} to {}",
            entry.path().display(),
            dest.display()
        );
        if self.tricky_copy(entry.clone(), dest).is_ok() {
            // Delete remote existing entry
            debug!("Tricky-copy worked; removing existing remote entry");
            match self.client.remove_dir_all(entry.path()) {
                Ok(_) => self.log(
                    LogLevel::Info,
                    format!(
                        "Moved \"{}\" to \"{}\"",
                        entry.path().display(),
                        dest.display()
                    ),
                ),
                Err(err) => self.log_and_alert(
                    LogLevel::Error,
                    format!(
                        "Copied \"{}\" to \"{}\"; but failed to remove src: {}",
                        entry.path().display(),
                        dest.display(),
                        err
                    ),
                ),
            }
        } else {
            error!("Tricky move aborted due to tricky-copy failure");
        }
    }
}
