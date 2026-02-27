//! ## FileTransferActivity
//!
//! `filetransfer_activity` is the module which implements the Filetransfer activity, which is the main activity afterall

// locals
use std::path::PathBuf;

use super::{FileTransferActivity, LogLevel};

impl FileTransferActivity {
    /// Create a symlink for the currently selected file via the active tab's pane.
    pub(crate) fn action_symlink(&mut self, name: String) {
        let entry = if let Some(e) = self.get_selected_file() {
            e
        } else {
            return;
        };
        let link_path = PathBuf::from(name.as_str());
        match self
            .browser
            .fs_pane_mut()
            .fs
            .symlink(link_path.as_path(), entry.path())
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
