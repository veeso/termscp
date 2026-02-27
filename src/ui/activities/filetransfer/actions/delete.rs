//! ## FileTransferActivity
//!
//! `filetransfer_activity` is the module which implements the Filetransfer activity, which is the main activity afterall

// locals
use remotefs::File;

use super::{FileTransferActivity, LogLevel, SelectedFile};

impl FileTransferActivity {
    /// Delete the currently selected file(s) via the active tab's pane.
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
                self.browser.explorer_mut().clear_queue();
                self.reload_browser_file_list();
            }
            SelectedFile::None => {}
        }
    }

    /// Remove a single file or directory via the active tab's pane.
    pub(crate) fn remove_file(&mut self, entry: &File) {
        match self.browser.fs_pane_mut().fs.remove(entry) {
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
