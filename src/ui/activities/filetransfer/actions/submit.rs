//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

// locals
use super::{File, FileTransferActivity};

enum SubmitAction {
    ChangeDir,
    None,
}

impl FileTransferActivity {
    /// Decides which action to perform on submit, dispatching to local or remote
    /// based on the active tab.
    pub(crate) fn action_submit(&mut self, entry: File) {
        let (action, entry) = if entry.is_dir() {
            (SubmitAction::ChangeDir, entry)
        } else if entry.metadata().symlink.is_some() {
            // Stat file
            let symlink = entry.metadata().symlink.as_ref().unwrap();
            let stat_file = if self.is_local_tab() {
                self.host_bridge
                    .stat(symlink.as_path())
                    .map_err(|e| e.to_string())
            } else {
                self.client
                    .stat(symlink.as_path())
                    .map_err(|e| e.to_string())
            };
            match stat_file {
                Ok(e) => (SubmitAction::ChangeDir, e),
                Err(err) => {
                    warn!(
                        "Could not stat file pointed by {} ({}): {}",
                        entry.path().display(),
                        symlink.display(),
                        err
                    );
                    (SubmitAction::ChangeDir, entry)
                }
            }
        } else {
            (SubmitAction::None, entry)
        };
        if let (SubmitAction::ChangeDir, entry) = (action, entry) {
            self.action_enter_dir(entry)
        }
    }
}
