//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

// locals
use std::path::PathBuf;

use super::{FileTransferActivity, LogLevel};

impl FileTransferActivity {
    /// Create a symlink for the currently selected file.
    /// Branches on the active tab (local vs remote).
    pub(crate) fn action_symlink(&mut self, name: String) {
        let entry = if let Some(e) = self.get_selected_file() {
            e
        } else {
            return;
        };
        let link_path = PathBuf::from(name.as_str());
        let result: Result<(), String> = if self.is_local_tab() {
            self.host_bridge
                .symlink(link_path.as_path(), entry.path())
                .map_err(|e| e.to_string())
        } else {
            self.client
                .symlink(link_path.as_path(), entry.path())
                .map_err(|e| e.to_string())
        };
        match result {
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
