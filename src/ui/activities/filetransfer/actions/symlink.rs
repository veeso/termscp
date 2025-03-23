//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

// locals
use std::path::PathBuf;

use super::{FileTransferActivity, LogLevel};

impl FileTransferActivity {
    /// Create symlink on localhost
    pub(crate) fn action_local_symlink(&mut self, name: String) {
        if let Some(entry) = self.get_local_selected_file() {
            match self
                .host_bridge
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

    /// Copy file on remote
    pub(crate) fn action_remote_symlink(&mut self, name: String) {
        if let Some(entry) = self.get_remote_selected_file() {
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
