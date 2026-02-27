//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

// locals
use remotefs::File;

use super::{FileTransferActivity, LogLevel, SelectedFile};

impl FileTransferActivity {
    /// Delete the currently selected file(s) using the active tab to decide
    /// whether to operate on the local host-bridge or the remote client.
    pub(crate) fn action_delete(&mut self) {
        match self.get_selected_entries() {
            SelectedFile::One(entry) => {
                self.remove_file(&entry);
            }
            SelectedFile::Many(entries) => {
                for (entry, _) in entries.iter() {
                    self.remove_file(entry);
                }

                // clear selection and reload
                if self.is_local_tab() {
                    self.host_bridge_mut().clear_queue();
                    self.reload_host_bridge_filelist();
                } else {
                    self.remote_mut().clear_queue();
                    self.reload_remote_filelist();
                }
            }
            SelectedFile::None => {}
        }
    }

    /// Remove a single file or directory, branching on the current tab.
    pub(crate) fn remove_file(&mut self, entry: &File) {
        let result: Result<(), String> = if self.is_local_tab() {
            self.host_bridge.remove(entry).map_err(|e| e.to_string())
        } else {
            self.client
                .remove_dir_all(entry.path())
                .map_err(|e| e.to_string())
        };
        match result {
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
