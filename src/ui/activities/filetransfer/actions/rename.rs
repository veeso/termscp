//! ## FileTransferActivity
//!
//! `filetransfer_activity` is the module which implements the Filetransfer activity, which is the main activity afterall

// locals
use std::path::{Path, PathBuf};

use super::{File, FileTransferActivity, LogLevel, SelectedFile};

impl FileTransferActivity {
    /// Rename / move the currently selected entries via the active tab's pane.
    pub(crate) fn action_rename(&mut self, input: String) {
        match self.get_selected_entries() {
            SelectedFile::One(entry) => {
                let dest_path = PathBuf::from(input);
                self.rename_file(&entry, dest_path.as_path());
            }
            SelectedFile::Many(entries) => {
                for (entry, mut dest_path) in entries.into_iter() {
                    dest_path.push(entry.name());
                    self.rename_file(&entry, dest_path.as_path());
                }

                // clear selection and reload
                self.browser.explorer_mut().clear_queue();
                self.reload_browser_file_list();
            }
            SelectedFile::None => {}
        }
    }

    /// Rename a single file via the active tab's pane.
    /// Falls back to `tricky_move` on remote tabs when rename fails.
    fn rename_file(&mut self, entry: &File, dest: &Path) {
        match self.browser.fs_pane_mut().fs.rename(entry, dest) {
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
            Err(err) => {
                if !self.is_local_tab() {
                    // Try tricky_move as a fallback on remote
                    debug!("Rename failed ({err}); attempting tricky_move");
                    self.tricky_move(entry, dest);
                } else {
                    self.log_and_alert(
                        LogLevel::Error,
                        format!(
                            "Could not move \"{}\" to \"{}\": {}",
                            entry.path().display(),
                            dest.display(),
                            err
                        ),
                    );
                }
            }
        }
    }

    /// Rename / move a file on the remote host.
    /// Falls back to `tricky_move` when the rename fails.
    /// Also used by fswatcher for syncing renames.
    pub(crate) fn remote_rename_file(&mut self, entry: &File, dest: &Path) {
        match self.browser.remote_pane_mut().fs.rename(entry, dest) {
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
            Err(err) => {
                // Try tricky_move as a fallback
                debug!("Rename failed ({err}); attempting tricky_move");
                self.tricky_move(entry, dest);
            }
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
            match self.browser.remote_pane_mut().fs.remove(entry) {
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
