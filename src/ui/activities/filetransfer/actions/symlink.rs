//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

// locals
use std::path::PathBuf;

use super::{FileTransferActivity, LogLevel, SelectedFile};

impl FileTransferActivity {
    /// Create symlink on localhost
    #[cfg(target_family = "unix")]
    pub(crate) fn action_local_symlink(&mut self, name: String) {
        if let SelectedFile::One(entry) = self.get_local_selected_entries() {
            match self
                .host
                .symlink(PathBuf::from(name.as_str()).as_path(), entry.path())
            {
                Ok(_) => {
                    self.log(
                        LogLevel::Info,
                        format!(
                            "Created symlink at {}, pointing to {}",
                            name,
                            entry.path().display()
                        ),
                    );
                }
                Err(err) => {
                    self.log_and_alert(LogLevel::Error, format!("Could not create symlink: {err}"));
                }
            }
        }
    }

    #[cfg(target_family = "windows")]
    pub(crate) fn action_local_symlink(&mut self, _name: String) {
        self.mount_error("Symlinks are not supported on Windows hosts");
    }

    /// Copy file on remote
    pub(crate) fn action_remote_symlink(&mut self, name: String) {
        if let SelectedFile::One(entry) = self.get_remote_selected_entries() {
            match self
                .client
                .symlink(PathBuf::from(name.as_str()).as_path(), entry.path())
            {
                Ok(_) => {
                    self.log(
                        LogLevel::Info,
                        format!(
                            "Created symlink at {}, pointing to {}",
                            name,
                            entry.path().display()
                        ),
                    );
                }
                Err(err) => {
                    self.log_and_alert(
                        LogLevel::Error,
                        format!(
                            "Could not create symlink pointing to {}: {}",
                            entry.path().display(),
                            err
                        ),
                    );
                }
            }
        }
    }
}
