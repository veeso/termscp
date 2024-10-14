//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

// locals
use remotefs::File;

use super::{FileTransferActivity, LogLevel, SelectedFile};

impl FileTransferActivity {
    pub(crate) fn action_local_delete(&mut self) {
        match self.get_local_selected_entries() {
            SelectedFile::One(entry) => {
                // Delete file
                self.local_remove_file(&entry);
            }
            SelectedFile::Many(entries) => {
                // Iter files
                for entry in entries.iter() {
                    // Delete file
                    self.local_remove_file(entry);
                }
            }
            SelectedFile::None => {}
        }
    }

    pub(crate) fn action_remote_delete(&mut self) {
        match self.get_remote_selected_entries() {
            SelectedFile::One(entry) => {
                // Delete file
                self.remote_remove_file(&entry);
            }
            SelectedFile::Many(entries) => {
                // Iter files
                for entry in entries.iter() {
                    // Delete file
                    self.remote_remove_file(entry);
                }
            }
            SelectedFile::None => {}
        }
    }

    pub(crate) fn local_remove_file(&mut self, entry: &File) {
        match self.host_bridge.remove(entry) {
            Ok(_) => {
                // Log
                self.log(
                    LogLevel::Info,
                    format!("Removed file \"{}\"", entry.path().display()),
                );
            }
            Err(err) => {
                self.log_and_alert(
                    LogLevel::Error,
                    format!(
                        "Could not delete file \"{}\": {}",
                        entry.path().display(),
                        err
                    ),
                );
            }
        }
    }

    pub(crate) fn remote_remove_file(&mut self, entry: &File) {
        match self.client.remove_dir_all(entry.path()) {
            Ok(_) => {
                self.log(
                    LogLevel::Info,
                    format!("Removed file \"{}\"", entry.path().display()),
                );
            }
            Err(err) => {
                self.log_and_alert(
                    LogLevel::Error,
                    format!(
                        "Could not delete file \"{}\": {}",
                        entry.path().display(),
                        err
                    ),
                );
            }
        }
    }
}
